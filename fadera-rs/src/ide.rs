//! `fadera ide [fichier.fa]` : sert l'éditeur (index.html + moteur WebAssembly) sur
//! http://127.0.0.1:4747 et l'ouvre dans une fenêtre de navigateur. Sans dépendance :
//! un petit serveur HTTP sur la boucle locale (jamais accessible depuis le réseau).
//!
//! - le fichier donné en argument est passé à l'éditeur par `/__ouvrir` ;
//! - l'éditeur envoie un signal de vie (`/__vivant`) : sans signal pendant 3 minutes
//!   (fenêtre fermée), le serveur s'arrête tout seul ;
//! - `--discret` (raccourcis Windows) masque la fenêtre noire de la console ;
//! - une seconde commande `fadera ide` réutilise le serveur déjà lancé ;
//! - terminal PowerShell / cmd intégré (`/__terminal`) et dossier réel partagé avec l'éditeur
//!   (`/__espace…`, par défaut Documents\Fadera ; `--espace=DOSSIER` pour en choisir un autre).
//!   Ces deux accès, qui touchent à l'ordinateur, exigent un jeton secret tiré au hasard à chaque
//!   lancement et remis seulement à la page d'éditeur servie par ce serveur (une autre page web
//!   ouverte dans le navigateur ne peut ni le lire ni l'obtenir).

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const PORT: u16 = 4747;
const DELAI_SANS_SIGNAL: Duration = Duration::from_secs(180);

struct Etat {
    racine: PathBuf,
    en_attente: Option<(String, String)>, // (nom, texte)
    dernier_signal: Option<Instant>,
    debut: Instant,
    jeton: String,
    espace: PathBuf,
}

fn echapper_json(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn lire_json_chaine(corps: &str, cle: &str) -> Option<String> {
    let motif = format!("\"{}\":\"", cle);
    let debut = corps.find(&motif)? + motif.len();
    let mut out = String::new();
    let mut it = corps[debut..].chars();
    while let Some(c) = it.next() {
        match c {
            '"' => return Some(out),
            '\\' => match it.next()? {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'u' => {
                    let h: String = it.by_ref().take(4).collect();
                    out.push(char::from_u32(u32::from_str_radix(&h, 16).ok()?)?);
                }
                x => out.push(x),
            },
            c => out.push(c),
        }
    }
    None
}

fn type_mime(p: &Path) -> &'static str {
    match p.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "wasm" => "application/wasm",
        "json" => "application/json",
        "css" => "text/css; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "fa" | "txt" | "md" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn repondre(f: &mut TcpStream, code: &str, mime: &str, corps: &[u8]) {
    let ent = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n\r\n",
        code,
        mime,
        corps.len()
    );
    let _ = f.write_all(ent.as_bytes());
    let _ = f.write_all(corps);
}

/// Chemin demandé → fichier sous la racine (jamais en dehors), ou None.
fn resoudre(racine: &Path, url: &str) -> Option<PathBuf> {
    let chemin = url.split(['?', '#']).next().unwrap_or("/");
    let chemin = chemin.trim_start_matches('/');
    let chemin = if chemin.is_empty() { "index.html" } else { chemin };
    if chemin.split('/').any(|s| s == ".." || s.contains('\\') || s.contains(':')) {
        return None;
    }
    let p = racine.join(chemin);
    if p.is_file() {
        Some(p)
    } else {
        None
    }
}

const MARQUEUR: &str = "@@FADERA_CWD@@";
const EXT_TEXTE: [&str; 14] = ["fa", "txt", "md", "json", "csv", "tsv", "html", "htm", "css", "xml", "yml", "yaml", "ini", "log"];
const TAILLE_MAX_FICHIER: u64 = 1024 * 1024;
const DELAI_COMMANDE: Duration = Duration::from_secs(120);

fn jeton_aleatoire() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut j = String::new();
    for i in 0..4u32 {
        let mut h = RandomState::new().build_hasher(); // clés aléatoires fournies par le système
        h.write_u128(nanos);
        h.write_u32(std::process::id() ^ i);
        j.push_str(&format!("{:016x}", h.finish()));
    }
    j
}

fn entete<'a>(entetes: &'a str, nom: &str) -> Option<&'a str> {
    entetes.lines().skip(1).find_map(|l| {
        let (k, v) = l.split_once(':')?;
        if k.trim().eq_ignore_ascii_case(nom) {
            Some(v.trim())
        } else {
            None
        }
    })
}

/// Accès réservé à la page d'éditeur : bon jeton, et origine locale si le navigateur en indique une.
fn acces_prive(entetes: &str, jeton: &str) -> bool {
    if entete(entetes, "x-fadera-jeton") != Some(jeton) {
        return false;
    }
    match entete(entetes, "origin") {
        None => true,
        Some(o) => o == format!("http://127.0.0.1:{}", PORT) || o == format!("http://localhost:{}", PORT),
    }
}

fn parametre_url(url: &str, cle: &str) -> Option<String> {
    let requete = url.split('#').next()?.split_once('?')?.1;
    for paire in requete.split('&') {
        if let Some((k, v)) = paire.split_once('=') {
            if k == cle {
                let b = v.as_bytes();
                let mut o = Vec::new();
                let mut i = 0;
                while i < b.len() {
                    if b[i] == b'%' && i + 2 < b.len() {
                        if let Some(x) = std::str::from_utf8(&b[i + 1..i + 3]).ok().and_then(|h| u8::from_str_radix(h, 16).ok()) {
                            o.push(x);
                            i += 3;
                            continue;
                        }
                    }
                    o.push(b[i]);
                    i += 1;
                }
                return String::from_utf8(o).ok();
            }
        }
    }
    None
}

/// Nom relatif sûr dans le dossier de travail : pas de « .. », pas de chemin absolu ni de lecteur,
/// pas de dossier caché, et seulement des extensions de texte.
fn nom_espace_valide(nom: &str) -> bool {
    if nom.is_empty() || nom.len() > 200 || nom.starts_with('/') || nom.contains('\\') || nom.contains(':') || nom.contains('\0') {
        return false;
    }
    let segs: Vec<&str> = nom.split('/').collect();
    if segs.len() > 6 || segs.iter().any(|s| s.is_empty() || *s == "." || *s == ".." || s.starts_with('.')) {
        return false;
    }
    match segs.last().and_then(|d| d.rsplit_once('.')) {
        Some((_, ext)) => EXT_TEXTE.contains(&ext.to_ascii_lowercase().as_str()),
        None => false,
    }
}

fn secondes(t: std::time::SystemTime) -> u64 {
    t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn lister_espace(base: &Path, rel: &str, sortie: &mut Vec<(String, u64, u64)>, profondeur: usize) {
    if profondeur > 5 || sortie.len() >= 500 {
        return;
    }
    let dossier = if rel.is_empty() { base.to_path_buf() } else { base.join(rel) };
    let Ok(entrees) = std::fs::read_dir(&dossier) else { return };
    let mut noms: Vec<_> = entrees.flatten().collect();
    noms.sort_by_key(|e| e.file_name());
    for e in noms {
        let nom = e.file_name().to_string_lossy().to_string();
        if nom.starts_with('.') || nom == "node_modules" || nom == "target" {
            continue;
        }
        let chemin_rel = if rel.is_empty() { nom.clone() } else { format!("{}/{}", rel, nom) };
        let Ok(meta) = e.metadata() else { continue };
        if meta.is_dir() {
            lister_espace(base, &chemin_rel, sortie, profondeur + 1);
        } else if meta.is_file() && meta.len() <= TAILLE_MAX_FICHIER && nom_espace_valide(&chemin_rel) {
            sortie.push((chemin_rel, meta.modified().map(secondes).unwrap_or(0), meta.len()));
            if sortie.len() >= 500 {
                return;
            }
        }
    }
}

fn lire_pipe<R: Read + Send + 'static>(mut r: R) -> std::thread::JoinHandle<Vec<u8>> {
    std::thread::spawn(move || {
        let mut tout = Vec::new();
        let mut bloc = [0u8; 8192];
        while let Ok(n) = r.read(&mut bloc) {
            if n == 0 {
                break;
            }
            if tout.len() < 1024 * 1024 {
                tout.extend_from_slice(&bloc[..n]);
            }
        }
        tout
    })
}

/// Lance une commande dans PowerShell, cmd (Windows) ou sh. Renvoie (sortie, erreur, code, nouveau dossier).
fn executer(shell: &str, commande: &str, dossier: &Path) -> (String, String, i32, Option<String>) {
    use std::process::{Command, Stdio};
    #[cfg(windows)]
    let mut c = {
        use std::os::windows::process::CommandExt;
        if shell == "cmd" {
            let mut c = Command::new("cmd.exe");
            c.raw_arg(format!("/D /S /C \"chcp 65001>nul & {} & echo {}%CD%\"", commande, MARQUEUR));
            c.creation_flags(0x0800_0000);
            c
        } else {
            let script = format!(
                "[Console]::OutputEncoding=[System.Text.Encoding]::UTF8;$OutputEncoding=[System.Text.Encoding]::UTF8;\n{}\n;$__q=$?;$__c=$LASTEXITCODE;Write-Output ('{}'+(Get-Location).Path);if($__c -ne $null){{exit $__c}}elseif(-not $__q){{exit 1}}",
                commande, MARQUEUR
            );
            let octets: Vec<u8> = script.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
            use base64::Engine;
            let b64 = base64::engine::general_purpose::STANDARD.encode(octets);
            let mut c = Command::new("powershell.exe");
            c.args(["-NoLogo", "-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-OutputFormat", "Text", "-EncodedCommand", &b64]);
            c.creation_flags(0x0800_0000);
            c
        }
    };
    #[cfg(not(windows))]
    let mut c = {
        let _ = shell;
        let mut c = Command::new("sh");
        c.args([
            "-c",
            &format!("{}\n__q=$?; printf '%s%s\\n' '{}' \"$(pwd)\"; exit $__q", commande, MARQUEUR),
        ]);
        c
    };
    c.current_dir(dossier).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    // « fadera » doit toujours être trouvé, même si Fadéra n'est pas dans le PATH
    if let Some(d) = std::env::current_exe().ok().and_then(|e| e.parent().map(|p| p.to_path_buf())) {
        let mut morceaux = vec![d];
        if let Some(p) = std::env::var_os("PATH") {
            morceaux.extend(std::env::split_paths(&p));
        }
        if let Ok(chemin) = std::env::join_paths(morceaux) {
            c.env("PATH", chemin);
        }
    }
    let mut enfant = match c.spawn() {
        Ok(e) => e,
        Err(e) => return (String::new(), format!("Impossible de lancer le terminal : {}", e), 127, None),
    };
    let sortie = enfant.stdout.take().map(lire_pipe);
    let erreur = enfant.stderr.take().map(lire_pipe);
    let debut = Instant::now();
    let mut code;
    let mut delai = false;
    loop {
        match enfant.try_wait() {
            Ok(Some(st)) => {
                code = st.code().unwrap_or(1);
                break;
            }
            Ok(None) => {
                if debut.elapsed() > DELAI_COMMANDE {
                    let _ = enfant.kill();
                    let _ = enfant.wait();
                    code = 124;
                    delai = true;
                    break;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(_) => {
                code = 1;
                break;
            }
        }
    }
    let mut so = sortie.map(|h| h.join().unwrap_or_default()).unwrap_or_default();
    let mut er = erreur.map(|h| h.join().unwrap_or_default()).unwrap_or_default();
    so.truncate(1024 * 1024);
    er.truncate(1024 * 1024);
    let mut so = String::from_utf8_lossy(&so).to_string();
    let mut er = String::from_utf8_lossy(&er).to_string().replace("#< CLIXML\r\n", "").replace("#< CLIXML\n", "");
    if er.contains("<Objs Version=") {
        er = String::new(); // reste de format interne de PowerShell, illisible
    }
    let mut nouveau = None;
    if let Some(i) = so.rfind(MARQUEUR) {
        let reste = so[i + MARQUEUR.len()..].lines().next().unwrap_or("").trim().to_string();
        if !reste.is_empty() {
            nouveau = Some(reste);
        }
        so.truncate(i);
    }
    if delai {
        er.push_str(&format!("\nCommande arrêtée : plus de {} secondes. (Ce terminal n'est pas interactif.)", DELAI_COMMANDE.as_secs()));
    }
    (so, er, code, nouveau)
}

/// Routes réservées à l'éditeur (jeton obligatoire) : terminal et dossier de travail.
fn route_privee(f: &mut TcpStream, methode: &str, chemin: &str, url: &str, corps: &str, espace: &Path) {
    let json = |f: &mut TcpStream, j: String| repondre(f, "200 OK", "application/json", j.as_bytes());
    match (methode, chemin) {
        ("POST", "/__terminal") => {
            let (Some(commande), shell) = (lire_json_chaine(corps, "commande"), lire_json_chaine(corps, "shell").unwrap_or_default()) else {
                return repondre(f, "400 Bad Request", "text/plain", b"json invalide");
            };
            if commande.len() > 20000 {
                return repondre(f, "413 Payload Too Large", "text/plain", b"commande trop longue");
            }
            let mut dossier = lire_json_chaine(corps, "dossier").map(PathBuf::from).unwrap_or_default();
            if !dossier.is_dir() {
                dossier = espace.to_path_buf();
            }
            let (so, er, code, nouveau) = executer(&shell, &commande, &dossier);
            let nouveau = nouveau.unwrap_or_else(|| dossier.to_string_lossy().to_string());
            json(f, format!("{{\"sortie\":{},\"erreur\":{},\"code\":{},\"dossier\":{}}}", echapper_json(&so), echapper_json(&er), code, echapper_json(&nouveau)));
        }
        ("GET", "/__espace") => {
            let mut v = Vec::new();
            lister_espace(espace, "", &mut v, 0);
            let liste: Vec<String> = v
                .iter()
                .map(|(n, mt, t)| format!("{{\"nom\":{},\"mt\":{},\"taille\":{}}}", echapper_json(n), mt, t))
                .collect();
            json(f, format!("{{\"dossier\":{},\"fichiers\":[{}]}}", echapper_json(&espace.to_string_lossy()), liste.join(",")));
        }
        ("GET", "/__espace/fichier") => {
            let nom = parametre_url(url, "nom").unwrap_or_default();
            if !nom_espace_valide(&nom) {
                return repondre(f, "400 Bad Request", "text/plain", b"nom refuse");
            }
            match std::fs::read_to_string(espace.join(&nom)) {
                Ok(t) if (t.len() as u64) <= TAILLE_MAX_FICHIER => json(f, format!("{{\"nom\":{},\"texte\":{}}}", echapper_json(&nom), echapper_json(&t))),
                _ => repondre(f, "404 Not Found", "text/plain", b"introuvable"),
            }
        }
        ("POST", "/__espace/ecrire") => {
            let (Some(nom), Some(texte)) = (lire_json_chaine(corps, "nom"), lire_json_chaine(corps, "texte")) else {
                return repondre(f, "400 Bad Request", "text/plain", b"json invalide");
            };
            if !nom_espace_valide(&nom) || (texte.len() as u64) > TAILLE_MAX_FICHIER {
                return repondre(f, "400 Bad Request", "text/plain", b"nom ou taille refuse");
            }
            let cible = espace.join(&nom);
            if let Some(p) = cible.parent() {
                let _ = std::fs::create_dir_all(p);
            }
            match std::fs::write(&cible, texte.as_bytes()) {
                Ok(_) => {
                    let mt = std::fs::metadata(&cible).and_then(|m| m.modified()).map(secondes).unwrap_or(0);
                    json(f, format!("{{\"mt\":{}}}", mt));
                }
                Err(e) => repondre(f, "500 Internal Server Error", "text/plain", e.to_string().as_bytes()),
            }
        }
        ("POST", "/__espace/supprimer") => {
            let Some(nom) = lire_json_chaine(corps, "nom") else {
                return repondre(f, "400 Bad Request", "text/plain", b"json invalide");
            };
            if !nom_espace_valide(&nom) {
                return repondre(f, "400 Bad Request", "text/plain", b"nom refuse");
            }
            let _ = std::fs::remove_file(espace.join(&nom));
            repondre(f, "200 OK", "text/plain", b"ok");
        }
        _ => repondre(f, "405 Method Not Allowed", "text/plain", b"methode refusee"),
    }
}

/// Dossier de travail partagé avec l'éditeur : --espace=DOSSIER, sinon FADERA_ESPACE, sinon Documents\Fadera.
fn dossier_espace(args: &[String]) -> PathBuf {
    let choisi = args
        .iter()
        .find_map(|a| a.strip_prefix("--espace=").map(|s| s.to_string()))
        .or_else(|| std::env::var("FADERA_ESPACE").ok())
        .filter(|s| !s.trim().is_empty());
    let d = match choisi {
        Some(s) => PathBuf::from(s),
        None => {
            let maison = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
            let docs = maison.join("Documents");
            if docs.is_dir() { docs.join("Fadera") } else { maison.join("Fadera") }
        }
    };
    let _ = std::fs::create_dir_all(&d);
    d
}

fn traiter(mut f: TcpStream, etat: &Arc<Mutex<Etat>>) {
    let _ = f.set_read_timeout(Some(Duration::from_secs(5)));
    let mut tampon = Vec::new();
    let mut bloc = [0u8; 8192];
    // lecture des en-têtes, puis du corps (Content-Length)
    let fin_entetes = loop {
        match f.read(&mut bloc) {
            Ok(0) | Err(_) => return,
            Ok(n) => tampon.extend_from_slice(&bloc[..n]),
        }
        if let Some(i) = tampon.windows(4).position(|w| w == b"\r\n\r\n") {
            break i + 4;
        }
        if tampon.len() > 65536 {
            return;
        }
    };
    let entetes = String::from_utf8_lossy(&tampon[..fin_entetes]).to_string();
    let longueur: usize = entetes
        .lines()
        .find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").map(|v| v.trim().parse().unwrap_or(0)))
        .unwrap_or(0);
    if longueur > 8 * 1024 * 1024 {
        repondre(&mut f, "413 Payload Too Large", "text/plain", b"trop gros");
        return;
    }
    while tampon.len() < fin_entetes + longueur {
        match f.read(&mut bloc) {
            Ok(0) | Err(_) => break,
            Ok(n) => tampon.extend_from_slice(&bloc[..n]),
        }
    }
    let corps = String::from_utf8_lossy(&tampon[fin_entetes.min(tampon.len())..]).to_string();
    let premiere = entetes.lines().next().unwrap_or("");
    let mut mots = premiere.split_whitespace();
    let methode = mots.next().unwrap_or("");
    let url = mots.next().unwrap_or("/");
    // protection contre le « DNS rebinding » : seul localhost est accepté comme hôte
    let hote_ok = entetes.lines().any(|l| {
        let m = l.to_ascii_lowercase();
        m.starts_with("host:") && (m.contains("127.0.0.1") || m.contains("localhost"))
    });
    if !hote_ok {
        repondre(&mut f, "403 Forbidden", "text/plain", b"hote refuse");
        return;
    }
    let chemin = url.split(['?', '#']).next().unwrap_or("/");
    if chemin == "/__terminal" || chemin.starts_with("/__espace") {
        let (jeton, espace) = {
            let e = etat.lock().unwrap();
            (e.jeton.clone(), e.espace.clone())
        };
        if !acces_prive(&entetes, &jeton) {
            repondre(&mut f, "403 Forbidden", "text/plain", b"jeton refuse");
        } else {
            route_privee(&mut f, methode, chemin, url, &corps, &espace);
        }
        return;
    }
    match (methode, chemin) {
        ("GET", "/__local.js") => repondre(&mut f, "200 OK", "text/javascript; charset=utf-8", include_str!("ide_local.js").as_bytes()),
        ("GET", "/__fadera") => repondre(&mut f, "200 OK", "text/plain", b"fadera-ide"),
        ("GET", "/__vivant") => {
            etat.lock().unwrap().dernier_signal = Some(Instant::now());
            repondre(&mut f, "200 OK", "text/plain", b"ok");
        }
        ("GET", "/__ouvrir") => {
            let pris = etat.lock().unwrap().en_attente.take();
            match pris {
                Some((nom, texte)) => {
                    let j = format!("{{\"nom\":{},\"texte\":{}}}", echapper_json(&nom), echapper_json(&texte));
                    repondre(&mut f, "200 OK", "application/json", j.as_bytes());
                }
                None => repondre(&mut f, "204 No Content", "text/plain", b""),
            }
        }
        ("POST", "/__ouvrir") => {
            if let (Some(nom), Some(texte)) = (lire_json_chaine(&corps, "nom"), lire_json_chaine(&corps, "texte")) {
                etat.lock().unwrap().en_attente = Some((nom, texte));
                repondre(&mut f, "200 OK", "text/plain", b"ok");
            } else {
                repondre(&mut f, "400 Bad Request", "text/plain", b"json invalide");
            }
        }
        ("GET", _) => {
            let (racine, jeton) = {
                let e = etat.lock().unwrap();
                (e.racine.clone(), e.jeton.clone())
            };
            match resoudre(&racine, url).and_then(|p| std::fs::read(&p).ok().map(|c| (p, c))) {
                Some((p, c)) if p.file_name().map(|n| n == "index.html").unwrap_or(false) => {
                    // l'éditeur reçoit le jeton secret et le module « dossier réel + PowerShell »
                    let mut page = String::from_utf8_lossy(&c).to_string();
                    let ajout = format!("<meta name=\"fadera-jeton\" content=\"{}\"><script src=\"/__local.js\"></script>", jeton);
                    match page.rfind("</body>") {
                        Some(i) => page.insert_str(i, &ajout),
                        None => page.push_str(&ajout),
                    }
                    repondre(&mut f, "200 OK", type_mime(&p), page.as_bytes())
                }
                Some((p, c)) => repondre(&mut f, "200 OK", type_mime(&p), &c),
                None => repondre(&mut f, "404 Not Found", "text/plain", b"introuvable"),
            }
        }
        _ => repondre(&mut f, "405 Method Not Allowed", "text/plain", b"methode refusee"),
    }
}

/// Dossier contenant index.html : `ide/` à côté de l'exécutable, sinon dossier de l'exécutable,
/// sinon dossier courant et son parent (utile en développement).
fn trouver_racine() -> Option<PathBuf> {
    let mut candidats = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(d) = exe.parent() {
            candidats.push(d.join("ide"));
            candidats.push(d.to_path_buf());
            if let Some(p) = d.parent() {
                candidats.push(p.to_path_buf());
                if let Some(pp) = p.parent() {
                    candidats.push(pp.to_path_buf());
                    if let Some(ppp) = pp.parent() {
                        candidats.push(ppp.to_path_buf());
                    }
                }
            }
        }
    }
    if let Ok(c) = std::env::current_dir() {
        candidats.push(c.join("ide"));
        candidats.push(c.clone());
        if let Some(p) = c.parent() {
            candidats.push(p.to_path_buf());
        }
    }
    candidats.into_iter().find(|d| d.join("index.html").is_file())
}

fn ouvrir_navigateur(url: &str) {
    use std::process::{Command, Stdio};
    let muet = |c: &mut Command| {
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            c.creation_flags(0x0800_0000); // CREATE_NO_WINDOW : pas de console qui clignote
        }
        c.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status().map(|s| s.success()).unwrap_or(false)
    };
    #[cfg(windows)]
    {
        // fenêtre d'application (sans barre d'adresse) avec Edge, sinon Chrome, sinon le navigateur par défaut
        for nav in ["msedge", "chrome"] {
            if muet(Command::new("cmd").args(["/C", "start", "", nav, &format!("--app={}", url)])) {
                return;
            }
        }
        muet(Command::new("cmd").args(["/C", "start", "", url]));
    }
    #[cfg(target_os = "macos")]
    {
        muet(Command::new("open").arg(url));
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        muet(Command::new("xdg-open").arg(url));
    }
}

fn serveur_deja_lance() -> bool {
    match TcpStream::connect_timeout(&format!("127.0.0.1:{}", PORT).parse().unwrap(), Duration::from_millis(500)) {
        Ok(mut s) => {
            let _ = s.set_read_timeout(Some(Duration::from_secs(2)));
            let _ = s.write_all(b"GET /__fadera HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
            let mut r = String::new();
            let _ = s.read_to_string(&mut r);
            r.contains("fadera-ide")
        }
        Err(_) => false,
    }
}

fn envoyer_fichier(nom: &str, texte: &str) {
    if let Ok(mut s) = TcpStream::connect_timeout(&format!("127.0.0.1:{}", PORT).parse().unwrap(), Duration::from_millis(500)) {
        let corps = format!("{{\"nom\":{},\"texte\":{}}}", echapper_json(nom), echapper_json(texte));
        let req = format!(
            "POST /__ouvrir HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            corps.len(),
            corps
        );
        let _ = s.write_all(req.as_bytes());
        let mut r = String::new();
        let _ = s.read_to_string(&mut r);
    }
}

#[cfg(windows)]
fn masquer_console() {
    extern "system" {
        fn FreeConsole() -> i32;
    }
    unsafe {
        FreeConsole();
    }
}
#[cfg(not(windows))]
fn masquer_console() {}

/// `fadera editer fichier.fa` (depuis le terminal de l'éditeur) : ouvre le fichier dans l'éditeur déjà
/// ouvert, sans nouvelle fenêtre. Si aucun éditeur n'est ouvert, se comporte comme `fadera ide fichier.fa`.
pub fn editer(args: &[String]) -> i32 {
    let Some(f) = args.iter().find(|a| !a.starts_with("--")) else {
        eprintln!("Usage : fadera editer fichier.fa   (ouvre le fichier dans l'éditeur déjà ouvert)");
        return 2;
    };
    if serveur_deja_lance() {
        match std::fs::read_to_string(f) {
            Ok(texte) => {
                let nom = Path::new(f).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| f.clone());
                envoyer_fichier(&nom, &texte);
                println!("« {} » est ouvert dans l'éditeur.", nom);
                0
            }
            Err(e) => {
                eprintln!("Impossible de lire « {} » : {}", f, e);
                2
            }
        }
    } else {
        commande(args)
    }
}

pub fn commande(args: &[String]) -> i32 {
    let fichier = args.iter().find(|a| !a.starts_with("--")).cloned();
    let sans_navigateur = args.iter().any(|a| a == "--sans-navigateur");
    let a_ouvrir = match &fichier {
        Some(f) => match std::fs::read_to_string(f) {
            Ok(t) => {
                let nom = Path::new(f).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| f.clone());
                Some((nom, t))
            }
            Err(e) => {
                eprintln!("Impossible de lire « {} » : {}", f, e);
                return 2;
            }
        },
        None => None,
    };
    let url = format!("http://127.0.0.1:{}/", PORT);

    if serveur_deja_lance() {
        if let Some((nom, texte)) = &a_ouvrir {
            envoyer_fichier(nom, texte);
        }
        if !sans_navigateur {
            ouvrir_navigateur(&url);
        }
        return 0;
    }
    let racine = match trouver_racine() {
        Some(r) => r,
        None => {
            eprintln!("Éditeur introuvable : le dossier « ide » (avec index.html) doit se trouver à côté de fadera.exe.");
            return 2;
        }
    };
    let ecoute = match TcpListener::bind(("127.0.0.1", PORT)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Impossible d'utiliser le port {} : {}", PORT, e);
            return 2;
        }
    };
    if args.iter().any(|a| a == "--discret") {
        // lancé depuis un raccourci ou un double-clic : pas de fenêtre noire (Windows)
        masquer_console();
    } else {
        println!("Éditeur Fadéra : {}   (dossier : {})", url, racine.display());
        println!("Fermez la fenêtre de l'éditeur pour arrêter ce serveur (ou appuyez sur Ctrl+C).");
    }
    let espace = dossier_espace(args);
    let etat = Arc::new(Mutex::new(Etat { racine, en_attente: a_ouvrir, dernier_signal: None, debut: Instant::now(), jeton: jeton_aleatoire(), espace }));
    {
        let etat = Arc::clone(&etat);
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_secs(5));
            let e = etat.lock().unwrap();
            let inactif = match e.dernier_signal {
                Some(t) => t.elapsed() > DELAI_SANS_SIGNAL,
                None => e.debut.elapsed() > Duration::from_secs(120),
            };
            if inactif {
                std::process::exit(0);
            }
        });
    }
    if !sans_navigateur {
        ouvrir_navigateur(&url);
    }
    for flux in ecoute.incoming().flatten() {
        let etat = Arc::clone(&etat);
        std::thread::spawn(move || traiter(flux, &etat));
    }
    0
}
