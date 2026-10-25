use fadera::*;
use std::io::{BufRead, Write};

mod paquets;
mod reseau;

struct HoteCli {
    base: std::path::PathBuf,
}

impl HoteCli {
    fn resoudre(&self, p: &str) -> std::path::PathBuf {
        let c = std::path::Path::new(p);
        if c.is_absolute() || c.exists() {
            c.to_path_buf()
        } else {
            let b = self.base.join(p);
            if b.exists() {
                b
            } else {
                c.to_path_buf()
            }
        }
    }
}

impl Hote for HoteCli {
    fn afficher(&mut self, t: &str) {
        let so = std::io::stdout();
        let mut o = so.lock();
        let _ = writeln!(o, "{}", t);
    }
    fn saisir(&mut self, invite: &str) -> Result<Option<String>, HoteErr> {
        print!("{}", invite);
        let _ = std::io::stdout().flush();
        let mut l = String::new();
        match std::io::stdin().lock().read_line(&mut l) {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(l.trim_end_matches(['\n', '\r']).to_string())),
            Err(e) => Err(HoteErr::Msg(e.to_string())),
        }
    }
    fn lire_fichier(&mut self, p: &str) -> Result<String, HoteErr> {
        std::fs::read_to_string(self.resoudre(p)).map_err(|e| HoteErr::Msg(e.to_string()))
    }
    fn ecrire_fichier(&mut self, p: &str, c: &str) -> Result<(), HoteErr> {
        std::fs::write(p, c).map_err(|e| HoteErr::Msg(e.to_string()))
    }
    fn http(&mut self, methode: &str, url: &str, corps: Option<&str>, entetes: &[(String, String)]) -> Result<(u16, String), HoteErr> {
        reseau::requete(methode, url, corps, entetes).map_err(HoteErr::Msg)
    }
}

/// Hôte de débogage en ligne de commande. `plan` : commandes successives (une par réponse ; « continuer » quand il est épuisé) ;
/// sans plan (None), les commandes sont lues sur l'entrée standard. `trace` : mode d'essai (sortie normalisée, comparée au moteur JS).
struct HoteDebug {
    base: HoteCli,
    plan: Option<Vec<String>>,
    trace: bool,
}

fn champ(v: &Valeur, k: &str) -> Valeur {
    match v {
        Valeur::Struct(d) => d.borrow().get(k).cloned().unwrap_or(Valeur::Rien),
        _ => Valeur::Rien,
    }
}
fn texte_de(v: &Valeur) -> String {
    match v {
        Valeur::Texte(t) => t.to_string(),
        autre => formater(autre),
    }
}
fn liste_de(v: &Valeur) -> Vec<Valeur> {
    match v {
        Valeur::Liste(l) => l.borrow().clone(),
        _ => vec![],
    }
}

impl Hote for HoteDebug {
    fn afficher(&mut self, t: &str) {
        if self.trace {
            println!("out:{}", t);
        } else {
            self.base.afficher(t);
        }
    }
    fn lire_fichier(&mut self, p: &str) -> Result<String, HoteErr> {
        self.base.lire_fichier(p)
    }
    fn ecrire_fichier(&mut self, p: &str, c: &str) -> Result<(), HoteErr> {
        self.base.ecrire_fichier(p, c)
    }
    fn debogage(&mut self, json: &str) -> String {
        let v = fadera::util::json_lire(json).unwrap_or(Valeur::Rien);
        let res = champ(&v, "resultat");
        let vars = |k: &str| -> String {
            let mut x: Vec<String> = liste_de(&champ(&v, k)).iter().map(|e| format!("{}={}", texte_de(&champ(e, "nom")), texte_de(&champ(e, "valeur")))).collect();
            if k == "globales" {
                x.sort();
            }
            x.join(";")
        };
        if self.trace {
            if let Valeur::Rien = res {
                let pile: Vec<String> = liste_de(&champ(&v, "pile")).iter().map(|e| format!("{}:{}", texte_de(&champ(e, "nom")), texte_de(&champ(e, "ligne")))).collect();
                println!("L{} {} | {} | {} | {}", texte_de(&champ(&v, "ligne")), texte_de(&champ(&v, "raison")), vars("locales"), vars("globales"), pile.join(","));
            } else {
                println!("  eval {} : {}", if matches!(champ(&res, "ok"), Valeur::Bool(true)) { "ok" } else { "erreur" }, texte_de(&champ(&res, "texte")));
            }
            return match self.plan.as_mut() {
                Some(p) if !p.is_empty() => p.remove(0),
                _ => "continuer".to_string(),
            };
        }
        // mode interactif
        if let Valeur::Rien = res {
            let pile: Vec<String> = liste_de(&champ(&v, "pile")).iter().map(|e| format!("{} (ligne {})", texte_de(&champ(e, "nom")), texte_de(&champ(e, "ligne")))).collect();
            println!("⏸ ligne {} [{}]", texte_de(&champ(&v, "ligne")), texte_de(&champ(&v, "raison")));
            println!("  pile      : {}", pile.join("  ←  "));
            let l = vars("locales");
            if !l.is_empty() {
                println!("  locales   : {}", l.replace(';', ", "));
            }
            let g = vars("globales");
            if !g.is_empty() {
                println!("  globales  : {}", g.replace(';', ", "));
            }
        } else {
            println!("  = {}", texte_de(&champ(&res, "texte")));
        }
        loop {
            print!("(deboguer) c=continuer n=passer s=entrer o=sortir q=arrêter e <expr> > ");
            let _ = std::io::stdout().flush();
            let mut l = String::new();
            if std::io::stdin().lock().read_line(&mut l).unwrap_or(0) == 0 {
                return "stop".to_string();
            }
            let l = l.trim();
            let (c, reste) = l.split_once(' ').unwrap_or((l, ""));
            return match c {
                "c" | "continuer" => "continuer",
                "n" | "passer" => "passer",
                "s" | "entrer" => "entrer",
                "o" | "sortir" => "sortir",
                "q" | "stop" | "arreter" => "stop",
                "e" | "eval" if !reste.is_empty() => return format!("eval:{}", reste),
                _ => continue,
            }
            .to_string();
        }
    }
}

fn deboguer(code: String, base: std::path::PathBuf, plan: Option<Vec<String>>, points: Vec<usize>, trace: bool) -> i32 {
    let (_, err) = fadera::debogueur::executer_programme_debogue(&code, Box::new(HoteDebug { base: HoteCli { base }, plan, trace }), &points);
    match err {
        None => 0,
        Some(e) => {
            let arret = e.message.contains(fadera::debogueur::MSG_STOP);
            let (texte, _) = decrire_erreur(&e, &code);
            if trace {
                println!("ERR:{}", if arret { e.message.clone() } else { texte });
            } else if arret {
                eprintln!("{}", e.message);
            } else {
                eprintln!("Erreur : {}", texte);
            }
            1
        }
    }
}

fn lancer(code: String, base: std::path::PathBuf) -> i32 {
    match executer_programme(&code, Box::new(HoteCli { base })) {
        Ok(_) => 0,
        Err(e) => {
            let (texte, source) = decrire_erreur(&e, &code);
            eprintln!("Erreur : {}", texte);
            if !source.is_empty() {
                eprintln!("    {}", source);
            }
            1
        }
    }
}

fn repl() {
    println!("Fadéra {} — mode interactif (Ctrl+D pour quitter)", env!("CARGO_PKG_VERSION"));
    let mut it = Interp::new(Box::new(HoteCli { base: std::env::current_dir().unwrap_or_default() }));
    let stdin = std::io::stdin();
    loop {
        print!(">>> ");
        let _ = std::io::stdout().flush();
        let mut l = String::new();
        if stdin.lock().read_line(&mut l).unwrap_or(0) == 0 {
            println!();
            break;
        }
        let mut code = l;
        // bloc multi-lignes : continuer tant que l'analyse échoue faute de « fin »
        loop {
            match analyser_code(&code) {
                Err(e) if e.message.contains("jamais fermé") || e.message.starts_with("Fin de code") => {
                    print!("... ");
                    let _ = std::io::stdout().flush();
                    let mut s = String::new();
                    if stdin.lock().read_line(&mut s).unwrap_or(0) == 0 {
                        break;
                    }
                    code.push_str(&s);
                }
                _ => break,
            }
        }
        match analyser_code(&code) {
            Err(e) => {
                let (t, _) = decrire_erreur(&e, &code);
                eprintln!("Erreur : {}", t);
            }
            Ok(ast) => {
                let r = it.executer_programme_ast(&ast);
                if let Err(Flow::Err(e)) = r {
                    let (t, _) = decrire_erreur(&e, &code);
                    eprintln!("Erreur : {}", t);
                }
            }
        }
    }
}

#[cfg(windows)]
fn console_utf8() {
    extern "system" {
        fn SetConsoleOutputCP(page: u32) -> i32;
    }
    unsafe {
        SetConsoleOutputCP(65001);
    }
}
#[cfg(not(windows))]
fn console_utf8() {}

fn main() {
    console_utf8();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = executer_avec_grande_pile(move || -> i32 {
        match args.first().map(|s| s.as_str()) {
            None => {
                repl();
                0
            }
            Some("--version") | Some("-v") => {
                println!("Fadéra {} (moteur Rust)", env!("CARGO_PKG_VERSION"));
                0
            }
            Some("--aide") | Some("--help") | Some("-h") => {
                println!("Usage : fadera run fichier.fa      exécute un programme (ou : fadera fichier.fa)\n        fadera fmt f.fa            met le code en forme\n        fadera deboguer f.fa [3,7] débogueur pas à pas (lignes d'arrêt facultatives)\n        fadera tester f.fa         lance les blocs tester \"nom\" ... fin\n        fadera paquet ...          gestionnaire de paquets (installer, lister, retirer, creer)\n        fadera -e \"code\"          exécute du code\n        fadera                     mode interactif");
                0
            }
            Some("--parite") => {
                // Mode de test : sortie identique au banc d'essai JS (fichiers virtuels, saisie vide).
                let code = std::fs::read_to_string(args.get(1).cloned().unwrap_or_default()).unwrap_or_default();
                // Fichiers virtuels : le contenu du dossier donné en 2e argument (chemins relatifs à ce dossier), sinon deux fichiers par défaut.
                let mut fichiers: Vec<(String, String)> = Vec::new();
                match args.get(2) {
                    Some(dossier) => {
                        fn parcourir(base: &std::path::Path, rel: &std::path::Path, sortie: &mut Vec<(String, String)>) {
                            if let Ok(entrees) = std::fs::read_dir(base.join(rel)) {
                                for e in entrees.flatten() {
                                    let r = rel.join(e.file_name());
                                    if e.path().is_dir() {
                                        parcourir(base, &r, sortie);
                                    } else if let Ok(c) = std::fs::read_to_string(e.path()) {
                                        sortie.push((r.to_string_lossy().replace('\\', "/"), c));
                                    }
                                }
                            }
                        }
                        parcourir(std::path::Path::new(dossier), std::path::Path::new(""), &mut fichiers);
                    }
                    None => {
                        fichiers.push(("outils.fa".into(), "fonction double(x)\n renvoyer x * 2\nfin\nsoit PI2 = 6.28".into()));
                        fichiers.push(("boucle.fa".into(), "importer \"boucle\"\nafficher 1".into()));
                    }
                }
                let refs: Vec<(&str, &str)> = fichiers.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
                let r = executer_capture(&code, vec![], &refs);
                print!("{}", r);
                0
            }
            Some("--tester-parite") => {
                let code = std::fs::read_to_string(args.get(1).cloned().unwrap_or_default()).unwrap_or_default();
                print!("{}", tester_capture(&code));
                0
            }
            Some("deboguer") | Some("--deboguer-trace") => {
                // fadera deboguer fichier.fa [lignes d'arrêt : 3,7]            (interactif)
                // fadera --deboguer-trace fichier.fa "plan" ["lignes"]         (essai : plan = commandes séparées par « ;; »)
                let trace = args[0] == "--deboguer-trace";
                let f = args.get(1).cloned().unwrap_or_default();
                let code = match std::fs::read_to_string(&f) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Impossible de lire « {} » : {}", f, e);
                        return 2;
                    }
                };
                let (plan, pts) = if trace { (Some(args.get(2).map(|p| p.split(";;").filter(|x| !x.is_empty()).map(String::from).collect()).unwrap_or_default()), args.get(3)) } else { (None, args.get(2)) };
                let points: Vec<usize> = pts.map(|p| p.split(',').filter_map(|x| x.trim().parse().ok()).collect()).unwrap_or_default();
                let base = std::path::Path::new(&f).parent().map(|p| p.to_path_buf()).unwrap_or_default();
                deboguer(code, base, plan, points, trace)
            }
            Some("paquet") | Some("paquets") => paquets::commande(&args[1..]),
            Some("fmt") => formater_cmd(&args[1..]),
            Some("tester") => match args.get(1) {
                None => {
                    eprintln!("Usage : fadera tester fichier.fa");
                    2
                }
                Some(f) => match std::fs::read_to_string(f) {
                    Ok(c) => {
                        let base = std::path::Path::new(f).parent().map(|p| p.to_path_buf()).unwrap_or_default();
                        match lancer_tests(&c, Box::new(HoteCli { base })) {
                            Ok(b) => (b.total != b.reussis) as i32,
                            Err(e) => {
                                let (texte, _) = decrire_erreur(&e, &c);
                                eprintln!("Erreur : {}", texte);
                                1
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Impossible de lire « {} » : {}", f, e);
                        2
                    }
                },
            },
            Some("run") | Some("executer") => match args.get(1) {
                None => {
                    eprintln!("Usage : fadera run fichier.fa");
                    2
                }
                Some(f) => match std::fs::read_to_string(f) {
                    Ok(c) => {
                        let base = std::path::Path::new(f).parent().map(|p| p.to_path_buf()).unwrap_or_default();
                        lancer(c, base)
                    }
                    Err(e) => {
                        eprintln!("Impossible de lire « {} » : {}", f, e);
                        2
                    }
                },
            },
            Some("-e") => lancer(args.get(1).cloned().unwrap_or_default(), std::env::current_dir().unwrap_or_default()),
            Some(f) => match std::fs::read_to_string(f) {
                Ok(c) => {
                    let base = std::path::Path::new(f).parent().map(|p| p.to_path_buf()).unwrap_or_default();
                    lancer(c, base)
                }
                Err(e) => {
                    eprintln!("Impossible de lire « {} » : {}", f, e);
                    2
                }
            },
        }
    });
    std::process::exit(code);
}

/// `fadera fmt [--ecrire] [--verifier] [--indentation N] [fichier.fa …]` ; sans fichier (ou « - ») : entrée standard.
fn formater_cmd(args: &[String]) -> i32 {
    let (mut ecrire, mut verifier, mut indent) = (false, false, 4usize);
    let mut fichiers: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--ecrire" | "-w" => ecrire = true,
            "--verifier" | "--check" => verifier = true,
            "--indentation" => {
                i += 1;
                match args.get(i).and_then(|x| x.parse::<usize>().ok()) {
                    Some(n) if (1..=8).contains(&n) => indent = n,
                    _ => {
                        eprintln!("--indentation attend un nombre de 1 à 8.");
                        return 2;
                    }
                }
            }
            "--aide" | "-h" => {
                println!("Usage : fadera fmt [--ecrire] [--verifier] [--indentation N] [fichier.fa …]\n  (sans option : affiche le code formaté ; sans fichier : lit l'entrée standard)");
                return 0;
            }
            autre => fichiers.push(autre.to_string()),
        }
        i += 1;
    }
    if fichiers.is_empty() {
        fichiers.push("-".into());
    }
    let mut statut = 0;
    for f in &fichiers {
        let src = if f == "-" {
            let mut s = String::new();
            let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut s);
            Ok(s)
        } else {
            std::fs::read_to_string(f).map_err(|e| e.to_string())
        };
        let src = match src {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Impossible de lire « {} » : {}", f, e);
                statut = 2;
                continue;
            }
        };
        match fadera::fmt::formater_source(&src, indent) {
            Ok(res) => {
                if verifier {
                    if res != src {
                        println!("{}", f);
                        if statut == 0 {
                            statut = 1;
                        }
                    }
                } else if ecrire && f != "-" {
                    if res != src {
                        if let Err(e) = std::fs::write(f, &res) {
                            eprintln!("Impossible d'écrire « {} » : {}", f, e);
                            statut = 2;
                        } else {
                            eprintln!("formaté : {}", f);
                        }
                    }
                } else {
                    print!("{}", res);
                }
            }
            Err(e) => {
                let (texte, _) = decrire_erreur(&e, &src);
                eprintln!("{} : {}", f, texte);
                statut = 2;
            }
        }
    }
    statut
}
