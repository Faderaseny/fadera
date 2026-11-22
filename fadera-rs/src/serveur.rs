//! Serveur web, dossiers et fichiers, mots de passe (scrypt), environnement : de quoi écrire une vraie application
//! réseau en Fadéra (voir docs/docs/SERVEUR.md).
//! Ces fonctions n'existent que dans le moteur natif (ligne de commande) : le navigateur ne peut ni ouvrir un port
//! ni lire un dossier, et répond alors par une erreur claire.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::path::Path;
use std::time::Duration;

use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64U;
use base64::Engine;

use crate::crypto;
use crate::erreur::*;
use crate::interp::Interp;
use crate::natives::NatFn;
use crate::util::*;
use crate::valeur::*;

pub fn trouver(nom: &str) -> Option<NatFn> {
    Some(match nom {
        "fichier_existe" => fichier_existe,
        "dossier_existe" => dossier_existe,
        "dossier_creer" => dossier_creer,
        "dossier_lister" => dossier_lister,
        "fichier_supprimer" => fichier_supprimer,
        "dossier_supprimer" => dossier_supprimer,
        "fichier_renommer" => fichier_renommer,
        "fichier_copier" => fichier_copier,
        "fichier_ecrire_sur" => fichier_ecrire_sur,
        "fichier_ecrire_base64" => fichier_ecrire_base64,
        "fichier_taille" => fichier_taille,
        "base64_taille" => base64_taille,
        "environnement" => environnement,
        "arguments" => arguments,
        "dossier_programme" => dossier_programme,
        "adresses_reseau" => adresses_reseau,
        "crypto_scrypt" => crypto_scrypt,
        "crypto_hmac_base64url" => crypto_hmac_base64url,
        "crypto_egal_constant" => crypto_egal_constant,
        "crypto_totp" => crypto_totp,
        "crypto_aleatoire_entier" => crypto_aleatoire_entier,
        "base64url_encoder" => base64url_encoder,
        "base64url_decoder" => base64url_decoder,
        "serveur_http" => serveur_http,
        "arreter" => arreter,
        _ => return None,
    })
}

fn t(s: impl AsRef<str>) -> Valeur {
    Valeur::texte(s)
}
fn txt(a: &[Valeur], i: usize, nom: &str) -> R<String> {
    match a.get(i) {
        Some(Valeur::Texte(s)) => Ok(s.to_string()),
        v => Err(fad(format!("{} : l'argument {} doit être un texte (reçu : {}).", nom, i + 1, v.map_or("rien", type_de))),
        ),
    }
}
fn io(nom: &str, chemin: &str, e: std::io::Error) -> Flow {
    fad(format!("{} : « {} » : {}.", nom, chemin, e))
}

// ------------------------------------------------------------------ fichiers et dossiers
fn fichier_existe(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(Valeur::Bool(Path::new(&txt(a, 0, "fichier_existe")?).is_file()))
}
fn dossier_existe(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(Valeur::Bool(Path::new(&txt(a, 0, "dossier_existe")?).is_dir()))
}
/// Crée le dossier et tous ses parents (sans erreur s'il existe déjà).
fn dossier_creer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let p = txt(a, 0, "dossier_creer")?;
    std::fs::create_dir_all(&p).map_err(|e| io("dossier_creer", &p, e))?;
    Ok(Valeur::Bool(true))
}
/// Noms (triés) des éléments d'un dossier.
fn dossier_lister(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let p = txt(a, 0, "dossier_lister")?;
    let mut noms: Vec<String> = std::fs::read_dir(&p)
        .map_err(|e| io("dossier_lister", &p, e))?
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    noms.sort();
    Ok(Valeur::liste(noms.into_iter().map(t).collect()))
}
fn fichier_supprimer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let p = txt(a, 0, "fichier_supprimer")?;
    std::fs::remove_file(&p).map_err(|e| io("fichier_supprimer", &p, e))?;
    Ok(Valeur::Bool(true))
}
/// Supprime un dossier ; le second argument « vrai » le supprime avec tout son contenu (sinon il doit être vide).
fn dossier_supprimer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let p = txt(a, 0, "dossier_supprimer")?;
    let tout = a.get(1).map_or(false, |v| v.vrai());
    let r = if tout { std::fs::remove_dir_all(&p) } else { std::fs::remove_dir(&p) };
    r.map_err(|e| io("dossier_supprimer", &p, e))?;
    Ok(Valeur::Bool(true))
}
/// Renomme ou déplace un fichier ou un dossier (remplace la destination si c'est un fichier).
fn fichier_renommer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let de = txt(a, 0, "fichier_renommer")?;
    let vers = txt(a, 1, "fichier_renommer")?;
    std::fs::rename(&de, &vers).map_err(|e| io("fichier_renommer", &de, e))?;
    Ok(Valeur::Bool(true))
}
fn fichier_copier(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let de = txt(a, 0, "fichier_copier")?;
    let vers = txt(a, 1, "fichier_copier")?;
    std::fs::copy(&de, &vers).map_err(|e| io("fichier_copier", &de, e))?;
    Ok(Valeur::Bool(true))
}
/// Écriture « tout ou rien » : le texte est écrit dans un fichier temporaire puis renommé, si bien qu'une coupure de
/// courant ne laisse jamais un fichier à moitié écrit.
fn fichier_ecrire_sur(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let p = txt(a, 0, "fichier_ecrire_sur")?;
    let c = txt(a, 1, "fichier_ecrire_sur")?;
    let tmp = format!("{}.tmp", p);
    std::fs::write(&tmp, c).map_err(|e| io("fichier_ecrire_sur", &tmp, e))?;
    std::fs::rename(&tmp, &p).map_err(|e| io("fichier_ecrire_sur", &p, e))?;
    Ok(Valeur::Bool(true))
}

/// Décode du base64 (avec ou sans « = » de remplissage) ; None si le texte n'est pas du base64 valide.
fn decoder_b64(texte: &str) -> Option<Vec<u8>> {
    use base64::engine::general_purpose::STANDARD_NO_PAD;
    STANDARD_NO_PAD.decode(texte.trim_end_matches('=')).ok()
}
/// Écrit les octets décodés d'un texte base64 dans un fichier (téléversement de fichiers binaires) ; renvoie leur nombre.
fn fichier_ecrire_base64(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let p = txt(a, 0, "fichier_ecrire_base64")?;
    let c = txt(a, 1, "fichier_ecrire_base64")?;
    let octets = decoder_b64(&c).ok_or_else(|| fad("fichier_ecrire_base64 : le texte n'est pas du base64 valide."))?;
    let tmp = format!("{}.tmp", p);
    std::fs::write(&tmp, &octets).map_err(|e| io("fichier_ecrire_base64", &tmp, e))?;
    std::fs::rename(&tmp, &p).map_err(|e| io("fichier_ecrire_base64", &p, e))?;
    Ok(Valeur::Nombre(octets.len() as f64))
}
/// Taille d'un fichier en octets.
fn fichier_taille(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let p = txt(a, 0, "fichier_taille")?;
    let m = std::fs::metadata(&p).map_err(|e| io("fichier_taille", &p, e))?;
    Ok(Valeur::Nombre(m.len() as f64))
}
/// Nombre d'octets que donnerait le décodage d'un texte base64 (sans écrire de fichier), ou « rien » si le texte est invalide.
fn base64_taille(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(match decoder_b64(&txt(a, 0, "base64_taille")?) {
        Some(b) => Valeur::Nombre(b.len() as f64),
        None => Valeur::Rien,
    })
}

// ------------------------------------------------------------------ environnement
fn environnement(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let nom = txt(a, 0, "environnement")?;
    Ok(match std::env::var(&nom) {
        Ok(v) => t(v),
        Err(_) => a.get(1).cloned().unwrap_or(Valeur::Rien),
    })
}
/// Arguments donnés après le nom du programme : `fadera server.fa --reset-admin ADMIN` → ["--reset-admin", "ADMIN"].
fn arguments(_: &mut Interp, _: &[Valeur], _: usize) -> R<Valeur> {
    let v: Vec<String> = std::env::args().collect();
    let debut = v.iter().position(|x| x.ends_with(".fa")).map_or(v.len(), |i| i + 1);
    Ok(Valeur::liste(v.into_iter().skip(debut).map(t).collect()))
}
/// Dossier qui contient le programme en cours (pour trouver ses fichiers quel que soit le dossier de lancement).
fn dossier_programme(_: &mut Interp, _: &[Valeur], _: usize) -> R<Valeur> {
    let v: Vec<String> = std::env::args().collect();
    let p = v.iter().find(|x| x.ends_with(".fa")).map(|x| std::fs::canonicalize(x).unwrap_or_else(|_| x.into()));
    let d = p.and_then(|p| p.parent().map(|d| d.to_path_buf())).or_else(|| std::env::current_dir().ok()).unwrap_or_default();
    let s = d.to_string_lossy().to_string();
    Ok(t(s.strip_prefix(r"\\?\").unwrap_or(&s)))
}
/// Adresses IPv4 de cette machine sur le réseau local (aucun paquet n'est envoyé).
fn adresses_reseau(_: &mut Interp, _: &[Valeur], _: usize) -> R<Valeur> {
    let mut r = Vec::new();
    if let Ok(s) = UdpSocket::bind("0.0.0.0:0") {
        if s.connect("192.0.2.1:9").is_ok() {
            if let Ok(a) = s.local_addr() {
                let ip = a.ip().to_string();
                if !ip.starts_with("127.") && !ip.starts_with("0.") {
                    r.push(t(ip));
                }
            }
        }
    }
    Ok(Valeur::liste(r))
}
/// Quitte le programme avec un code de sortie.
fn arreter(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let _ = std::io::stdout().flush();
    std::process::exit(a.get(0).map_or(0.0, js_nombre) as i32)
}

// ------------------------------------------------------------------ cryptographie pour comptes et jetons
/// Empreinte scrypt (N=16384, r=8, p=1) en hexadécimal : le sel est utilisé comme texte, comme `crypto.scryptSync` de Node.
fn crypto_scrypt(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let mdp = txt(a, 0, "crypto_scrypt")?;
    let sel = txt(a, 1, "crypto_scrypt")?;
    let lg = a.get(2).map_or(32.0, js_nombre);
    if !(1.0..=128.0).contains(&lg) {
        return Err(nat("la longueur doit être entre 1 et 128 octets."));
    }
    #[cfg(feature = "crypto")]
    {
        let p = scrypt::Params::new(14, 8, 1, lg as usize).map_err(|e| nat(e.to_string()))?;
        let mut sortie = vec![0u8; lg as usize];
        scrypt::scrypt(mdp.as_bytes(), sel.as_bytes(), &p, &mut sortie).map_err(|e| nat(e.to_string()))?;
        Ok(t(crypto::hex(&sortie)))
    }
    #[cfg(not(feature = "crypto"))]
    {
        let _ = (mdp, sel);
        Err(nat("cette version de Fadéra a été compilée sans la fonctionnalité « crypto »."))
    }
}
fn hex_octets(h: &str) -> Vec<u8> {
    (0..h.len() / 2).filter_map(|i| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).ok()).collect()
}
/// HMAC-SHA-256 d'un texte avec une clé, encodé en base64url (sans remplissage).
fn crypto_hmac_base64url(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let texte = txt(a, 0, "crypto_hmac_base64url")?;
    let cle = txt(a, 1, "crypto_hmac_base64url")?;
    let h = crypto::hmac_sha256_hex(&texte, &cle).map_err(nat)?;
    Ok(t(B64U.encode(hex_octets(&h))))
}
/// Comparaison de deux textes en temps constant (jetons, empreintes) : ne révèle pas la position de la première différence.
fn crypto_egal_constant(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = txt(a, 0, "crypto_egal_constant")?;
    let y = txt(a, 1, "crypto_egal_constant")?;
    let (x, y) = (x.as_bytes(), y.as_bytes());
    let mut d = (x.len() ^ y.len()) as u32;
    for i in 0..x.len().max(y.len()) {
        d |= (*x.get(i).unwrap_or(&0) ^ *y.get(i).unwrap_or(&0)) as u32;
    }
    Ok(Valeur::Bool(d == 0))
}
// ------------------------------------------------------- code à usage unique (TOTP, RFC 6238)
/// SHA-1 (RFC 3174). Nécessaire pour le TOTP des applications d'authentification ; ne sert à rien d'autre.
fn sha1(msg: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let mut m = msg.to_vec();
    m.push(0x80);
    while m.len() % 64 != 56 {
        m.push(0);
    }
    m.extend_from_slice(&((msg.len() as u64) * 8).to_be_bytes());
    for bloc in m.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([bloc[4 * i], bloc[4 * i + 1], bloc[4 * i + 2], bloc[4 * i + 3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for (i, wi) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let tmp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(*wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = tmp;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut r = [0u8; 20];
    for i in 0..5 {
        r[4 * i..4 * i + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    r
}
fn hmac_sha1(cle: &[u8], msg: &[u8]) -> [u8; 20] {
    let mut k = [0u8; 64];
    if cle.len() > 64 {
        k[..20].copy_from_slice(&sha1(cle));
    } else {
        k[..cle.len()].copy_from_slice(cle);
    }
    let mut interne: Vec<u8> = k.iter().map(|b| b ^ 0x36).collect();
    interne.extend_from_slice(msg);
    let mut externe: Vec<u8> = k.iter().map(|b| b ^ 0x5c).collect();
    externe.extend_from_slice(&sha1(&interne));
    sha1(&externe)
}
/// Décode du base32 (RFC 4648) : majuscules ou minuscules, espaces et « = » ignorés. None si un caractère est invalide.
fn base32_decoder(s: &str) -> Option<Vec<u8>> {
    let (mut tampon, mut bits, mut sortie) = (0u32, 0u32, Vec::new());
    for c in s.chars().filter(|c| !c.is_whitespace() && *c != '=' && *c != '-') {
        let v = match c.to_ascii_uppercase() {
            'A'..='Z' => c.to_ascii_uppercase() as u32 - 'A' as u32,
            '2'..='7' => c as u32 - '2' as u32 + 26,
            _ => return None,
        };
        tampon = (tampon << 5) | v;
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            sortie.push(((tampon >> bits) & 0xFF) as u8);
        }
    }
    Some(sortie)
}
/// crypto_totp(secret_base32 [, temps_ms]) : code à 6 chiffres (pas de 30 s, HMAC-SHA-1), compatible Google Authenticator.
fn crypto_totp(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let secret = txt(a, 0, "crypto_totp")?;
    let cle = match base32_decoder(&secret) {
        Some(c) if c.len() >= 10 => c,
        _ => return Err(fad("crypto_totp : le secret doit être un texte base32 (lettres A-Z et chiffres 2-7) d'au moins 16 caractères.")),
    };
    let ms = match a.get(1) {
        Some(v) => js_nombre(v),
        None => std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0.0, |d| d.as_millis() as f64),
    };
    if !ms.is_finite() || ms < 0.0 {
        return Err(fad("crypto_totp : le temps doit être un nombre de millisecondes positif."));
    }
    let pas = (ms / 30000.0).floor() as u64;
    let h = hmac_sha1(&cle, &pas.to_be_bytes());
    let o = (h[19] & 15) as usize;
    let v = (u32::from_be_bytes([h[o], h[o + 1], h[o + 2], h[o + 3]]) & 0x7fff_ffff) % 1_000_000;
    Ok(t(format!("{:06}", v)))
}
/// Entier aléatoire cryptographique dans [0, n[ (sans biais).
fn crypto_aleatoire_entier(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let n = a.get(0).map_or(f64::NAN, js_nombre);
    if !(n >= 1.0 && n <= 4294967296.0) || n.fract() != 0.0 {
        return Err(nat("la borne doit être un entier entre 1 et 4294967296."));
    }
    let n = n as u64;
    let limite = (1u64 << 32) / n * n;
    loop {
        let h = crypto::aleatoire_hex(4).map_err(nat)?;
        let v = u32::from_str_radix(&h, 16).unwrap_or(0) as u64;
        if v < limite {
            return Ok(Valeur::Nombre((v % n) as f64));
        }
    }
}
fn base64url_encoder(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(t(B64U.encode(txt(a, 0, "base64url_encoder")?.as_bytes())))
}
/// Renvoie le texte décodé, ou « rien » si l'entrée n'est pas du base64url valide.
fn base64url_decoder(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(match B64U.decode(txt(a, 0, "base64url_decoder")?.trim_end_matches('=')) {
        Ok(b) => match String::from_utf8(b) {
            Ok(s) => t(s),
            Err(_) => Valeur::Rien,
        },
        Err(_) => Valeur::Rien,
    })
}

// ------------------------------------------------------------------ serveur HTTP
struct Requete {
    methode: String,
    chemin: String,
    requete: Vec<(String, String)>,
    entetes: Vec<(String, String)>,
    corps: Vec<u8>,
}

fn decoder_url(s: &str) -> String {
    let b = s.as_bytes();
    let mut o = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'+' => o.push(b' '),
            b'%' if i + 2 < b.len() => {
                let h = std::str::from_utf8(&b[i + 1..i + 3]).ok().and_then(|x| u8::from_str_radix(x, 16).ok());
                match h {
                    Some(v) => {
                        o.push(v);
                        i += 2;
                    }
                    None => o.push(b'%'),
                }
            }
            c => o.push(c),
        }
        i += 1;
    }
    String::from_utf8_lossy(&o).to_string()
}

fn lire_requete(flux: &mut TcpStream, limite: usize) -> Result<Requete, (u16, &'static str)> {
    let mut tampon: Vec<u8> = Vec::new();
    let mut bloc = [0u8; 16384];
    let fin_entetes = loop {
        if let Some(p) = tampon.windows(4).position(|w| w == b"\r\n\r\n") {
            break p;
        }
        if tampon.len() > 65536 {
            return Err((431, "En-têtes trop volumineux"));
        }
        match flux.read(&mut bloc) {
            Ok(0) => return Err((400, "Requête incomplète")),
            Ok(n) => tampon.extend_from_slice(&bloc[..n]),
            Err(_) => return Err((408, "Délai dépassé")),
        }
    };
    let tete = String::from_utf8_lossy(&tampon[..fin_entetes]).to_string();
    let mut lignes = tete.split("\r\n");
    let premiere = lignes.next().unwrap_or("");
    let mut mots = premiere.split(' ');
    let methode = mots.next().unwrap_or("").to_uppercase();
    let cible = mots.next().unwrap_or("/").to_string();
    let mut entetes = Vec::new();
    for l in lignes {
        if let Some((k, v)) = l.split_once(':') {
            entetes.push((k.trim().to_lowercase(), v.trim().to_string()));
        }
    }
    let (chemin, qs) = match cible.split_once('?') {
        Some((c, q)) => (c.to_string(), q.to_string()),
        None => (cible.clone(), String::new()),
    };
    let requete = qs
        .split('&')
        .filter(|x| !x.is_empty())
        .map(|kv| match kv.split_once('=') {
            Some((k, v)) => (decoder_url(k), decoder_url(v)),
            None => (decoder_url(kv), String::new()),
        })
        .collect();
    let longueur: usize = entetes.iter().find(|(k, _)| k == "content-length").and_then(|(_, v)| v.parse().ok()).unwrap_or(0);
    if longueur > limite {
        return Err((413, "Requête trop volumineuse"));
    }
    let mut corps = tampon[fin_entetes + 4..].to_vec();
    let attend = entetes.iter().any(|(k, v)| k == "expect" && v.to_lowercase().contains("100-continue"));
    let morceaux = entetes.iter().any(|(k, v)| k == "transfer-encoding" && v.to_lowercase().contains("chunked"));
    if attend && (morceaux || corps.len() < longueur) {
        let _ = flux.write_all(b"HTTP/1.1 100 Continue\r\n\r\n");
    }
    if morceaux {
        // corps « chunked » : suite de blocs « taille en hexadécimal \r\n données \r\n », terminée par un bloc de taille 0
        let mut brut = corps;
        let mut decode: Vec<u8> = Vec::new();
        loop {
            // lire la ligne de taille
            let pos = loop {
                if let Some(p) = brut.windows(2).position(|w| w == b"\r\n") {
                    break p;
                }
                match flux.read(&mut bloc) {
                    Ok(0) | Err(_) => return Err((400, "Corps incomplet")),
                    Ok(n) => brut.extend_from_slice(&bloc[..n]),
                }
            };
            let ligne = String::from_utf8_lossy(&brut[..pos]).to_string();
            let taille = usize::from_str_radix(ligne.split(';').next().unwrap_or("0").trim(), 16).map_err(|_| (400, "Corps invalide"))?;
            brut.drain(..pos + 2);
            if taille == 0 {
                break;
            }
            if decode.len() + taille > limite {
                return Err((413, "Requête trop volumineuse"));
            }
            while brut.len() < taille + 2 {
                match flux.read(&mut bloc) {
                    Ok(0) | Err(_) => return Err((400, "Corps incomplet")),
                    Ok(n) => brut.extend_from_slice(&bloc[..n]),
                }
            }
            decode.extend_from_slice(&brut[..taille]);
            brut.drain(..taille + 2);
        }
        return Ok(Requete { methode, chemin: decoder_url(&chemin), requete, entetes, corps: decode });
    }
    while corps.len() < longueur {
        match flux.read(&mut bloc) {
            Ok(0) => return Err((400, "Corps incomplet")),
            Ok(n) => corps.extend_from_slice(&bloc[..n]),
            Err(_) => return Err((408, "Délai dépassé")),
        }
    }
    corps.truncate(longueur);
    Ok(Requete { methode, chemin: decoder_url(&chemin), requete, entetes, corps })
}

fn raison(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        408 => "Request Timeout",
        413 => "Payload Too Large",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        500 => "Internal Server Error",
        _ => "OK",
    }
}

fn repondre(flux: &mut TcpStream, code: u16, entetes: &[(String, String)], corps: &[u8]) {
    let mut s = format!("HTTP/1.1 {} {}\r\n", code, raison(code));
    let mut a_type = false;
    for (k, v) in entetes {
        let kl = k.to_lowercase();
        if kl == "content-length" || kl == "connection" {
            continue;
        }
        if kl == "content-type" {
            a_type = true;
        }
        s.push_str(&format!("{}: {}\r\n", k, v));
    }
    if !a_type {
        s.push_str("Content-Type: text/plain; charset=utf-8\r\n");
    }
    s.push_str(&format!("Content-Length: {}\r\nConnection: close\r\n\r\n", corps.len()));
    let _ = flux.write_all(s.as_bytes());
    let _ = flux.write_all(corps);
    let _ = flux.flush();
}

fn paires(v: &[(String, String)]) -> Valeur {
    let mut d = Dict::nouveau();
    for (k, x) in v {
        d.set(k, t(x));
    }
    Valeur::structure(d)
}

/// `serveur_http(port, gestionnaire, options)` : écoute sur le port et appelle `gestionnaire(requete)` pour chaque
/// requête (l'une après l'autre) ; ne rend jamais la main. `requete` = { methode, chemin, requete, entetes, corps, ip } ;
/// le gestionnaire renvoie { statut, entetes, corps } ou { statut, entetes, fichier: "chemin" } pour servir un fichier tel quel.
/// Le compteur d'étapes est remis à zéro à chaque requête. `options` (facultatif) : { limite_corps: octets }.
fn serveur_http(it: &mut Interp, a: &[Valeur], ligne: usize) -> R<Valeur> {
    let port = a.get(0).map_or(f64::NAN, js_nombre);
    if !(port >= 0.0 && port <= 65535.0) || port.fract() != 0.0 {
        return Err(nat("le port doit être un entier entre 0 et 65535."));
    }
    let gestionnaire = match a.get(1) {
        Some(f @ Valeur::Fonction(_)) => f.clone(),
        _ => return Err(nat("le deuxième argument doit être une fonction qui reçoit la requête.")),
    };
    let limite = match a.get(2) {
        Some(Valeur::Struct(d)) => d.borrow().get("limite_corps").map_or(100e6, js_nombre),
        _ => 100e6,
    } as usize;
    let ecoute = TcpListener::bind(("0.0.0.0", port as u16)).map_err(|e| fad(format!("serveur_http : impossible d'ouvrir le port {} ({}). Est-il déjà utilisé ?", port, e)))?;
    for flux in ecoute.incoming() {
        let mut flux = match flux {
            Ok(f) => f,
            Err(_) => continue,
        };
        let _ = flux.set_read_timeout(Some(Duration::from_secs(30)));
        let _ = flux.set_write_timeout(Some(Duration::from_secs(30)));
        let ip = flux.peer_addr().map(|a| a.ip().to_string()).unwrap_or_default();
        let rq = match lire_requete(&mut flux, limite) {
            Ok(r) => r,
            Err((c, m)) => {
                repondre(&mut flux, c, &[("Content-Type".into(), "application/json; charset=utf-8".into())], format!("{{\"error\":\"{}\"}}", m).as_bytes());
                continue;
            }
        };
        let arg = Valeur::struct_de(vec![
            ("methode", t(&rq.methode)),
            ("chemin", t(&rq.chemin)),
            ("requete", paires(&rq.requete)),
            ("entetes", paires(&rq.entetes)),
            ("corps", Valeur::texte_string(String::from_utf8_lossy(&rq.corps).to_string())),
            ("ip", t(ip)),
        ]);
        it.etapes = 0;
        match it.appeler_valeur(&gestionnaire, vec![arg], ligne) {
            Ok(Valeur::Struct(d)) => {
                let d = d.borrow();
                let statut = d.get("statut").map_or(200.0, js_nombre);
                let mut en: Vec<(String, String)> = Vec::new();
                if let Some(Valeur::Struct(h)) = d.get("entetes") {
                    let h = h.borrow();
                    for k in h.cles_ordonnees() {
                        en.push((k.clone(), js_texte(h.get(&k).unwrap_or(&Valeur::Rien))));
                    }
                }
                let corps: Vec<u8> = match d.get("fichier") {
                    Some(Valeur::Texte(p)) => match std::fs::read(p.as_str()) {
                        Ok(b) => b,
                        Err(_) => {
                            repondre(&mut flux, 404, &[("Content-Type".into(), "application/json; charset=utf-8".into())], b"{\"error\":\"Introuvable\"}");
                            continue;
                        }
                    },
                    _ => d.get("corps").map_or(String::new(), js_texte).into_bytes(),
                };
                repondre(&mut flux, if statut.is_nan() { 200 } else { statut as u16 }, &en, &corps);
            }
            Ok(_) => repondre(&mut flux, 500, &[("Content-Type".into(), "application/json; charset=utf-8".into())], b"{\"error\":\"Le gestionnaire doit renvoyer une structure\"}"),
            Err(Flow::Err(e)) => {
                eprintln!("Erreur dans le gestionnaire (ligne {}) : {}", e.ligne.map_or("?".into(), |l| l.to_string()), e.message);
                let corps = json_texte(&Valeur::struct_de(vec![("error", t(&e.message))]), "").unwrap_or_else(|_| "{}".into());
                repondre(&mut flux, 400, &[("Content-Type".into(), "application/json; charset=utf-8".into())], corps.as_bytes());
            }
            Err(Flow::Nat(m)) => {
                eprintln!("Erreur dans le gestionnaire : {}", m);
                repondre(&mut flux, 400, &[("Content-Type".into(), "application/json; charset=utf-8".into())], b"{\"error\":\"Erreur interne\"}");
            }
            Err(_) => repondre(&mut flux, 500, &[], b""),
        }
    }
    Ok(Valeur::Rien)
}
