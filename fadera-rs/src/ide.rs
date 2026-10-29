//! `fadera ide [fichier.fa]` : sert l'éditeur (index.html + moteur WebAssembly) sur
//! http://127.0.0.1:4747 et l'ouvre dans une fenêtre de navigateur. Sans dépendance :
//! un petit serveur HTTP sur la boucle locale (jamais accessible depuis le réseau).
//!
//! - le fichier donné en argument est passé à l'éditeur par `/__ouvrir` ;
//! - l'éditeur envoie un signal de vie (`/__vivant`) : sans signal pendant 3 minutes
//!   (fenêtre fermée), le serveur s'arrête tout seul ;
//! - `--discret` (raccourcis Windows) masque la fenêtre noire de la console ;
//! - une seconde commande `fadera ide` réutilise le serveur déjà lancé.

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
    match (methode, chemin) {
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
            let racine = etat.lock().unwrap().racine.clone();
            match resoudre(&racine, url).and_then(|p| std::fs::read(&p).ok().map(|c| (p, c))) {
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
    let etat = Arc::new(Mutex::new(Etat { racine, en_attente: a_ouvrir, dernier_signal: None, debut: Instant::now() }));
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
