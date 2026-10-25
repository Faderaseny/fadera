//! Gestionnaire de paquets : `fadera paquet installer | lister | retirer | creer`.
//!
//! Un paquet est un dossier de fichiers Fadéra avec un fichier `fadera-paquet.json` à sa racine :
//!   { "nom": "outils", "version": "1.0.0", "description": "…", "principal": "principal.fa", "fichiers": ["principal.fa", "aide.fa"] }
//! Il est installé dans `paquets/<nom>/` ; un programme l'utilise avec `importer "paquets/outils/principal" comme o`.
//! Aucun mot-clé, aucune fonction native : c'est l'importation de modules qui existait déjà.
//! Un projet liste ses paquets dans `fadera.json` : { "dependances": { "outils": "utilisateur/depot@v1.0.0" } }.
use fadera::util::{json_lire, json_texte};
use fadera::{Valeur};
use std::path::{Path, PathBuf};

const MANIFESTE: &str = "fadera-paquet.json";
const PROJET: &str = "fadera.json";
const DOSSIER: &str = "paquets";
const MAX_FICHIERS: usize = 200;
const MAX_OCTETS_FICHIER: usize = 1_000_000;
const EXTENSIONS: [&str; 5] = ["fa", "md", "txt", "json", "sortie"];

type R<T> = Result<T, String>;

pub fn commande(args: &[String]) -> i32 {
    let r = match args.first().map(|s| s.as_str()) {
        Some("installer") => installer(&args[1..]),
        Some("lister") => lister(),
        Some("retirer") => retirer(&args[1..]),
        Some("creer") => creer(&args[1..]),
        _ => {
            eprintln!(
                "Usage : fadera paquet installer [source]   installe un paquet (sans source : tout ce qu'indique fadera.json)\n\
                 \x20       fadera paquet lister               liste les paquets installés\n\
                 \x20       fadera paquet retirer nom          désinstalle un paquet\n\
                 \x20       fadera paquet creer nom            crée le squelette d'un nouveau paquet\n\
                 \n\
                 Source : utilisateur/depot[@version] (GitHub), une adresse https://… vers le dossier du paquet, ou un dossier local."
            );
            return 2;
        }
    };
    match r {
        Ok(()) => 0,
        Err(m) => {
            eprintln!("Erreur : {}", m);
            1
        }
    }
}

// ------------------------------------------------------------------ validations
fn nom_valide(n: &str) -> bool {
    !n.is_empty() && n.len() <= 40 && n.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-') && !n.starts_with('-')
}

/// Chemin relatif sûr : pas de « .. », pas de racine, pas de lecteur Windows, extension connue.
fn chemin_valide(p: &str) -> R<()> {
    if p.is_empty() || p.len() > 120 || p.starts_with('/') || p.starts_with('\\') || p.contains(':') || p.contains('\\') || p.contains('\0') {
        return Err(format!("chemin de fichier refusé : « {} ».", p));
    }
    for part in p.split('/') {
        if part.is_empty() || part == "." || part == ".." {
            return Err(format!("chemin de fichier refusé : « {} ».", p));
        }
    }
    let ext = p.rsplit('.').next().unwrap_or("");
    if !p.contains('.') || !EXTENSIONS.contains(&ext) {
        return Err(format!("« {} » : seuls les fichiers {} sont acceptés dans un paquet.", p, EXTENSIONS.iter().map(|e| format!(".{}", e)).collect::<Vec<_>>().join(", ")));
    }
    Ok(())
}

// ------------------------------------------------------------------ JSON
fn champ_texte(v: &Valeur, k: &str) -> Option<String> {
    match v {
        Valeur::Struct(d) => match d.borrow().get(k) {
            Some(Valeur::Texte(t)) => Some((**t).clone()),
            _ => None,
        },
        _ => None,
    }
}

struct Manifeste {
    nom: String,
    version: String,
    principal: String,
    fichiers: Vec<String>,
}

fn lire_manifeste(texte: &str) -> R<Manifeste> {
    let v = json_lire(texte).map_err(|e| format!("{} n'est pas du JSON valide ({}).", MANIFESTE, e))?;
    let nom = champ_texte(&v, "nom").ok_or_else(|| format!("{} : le champ \"nom\" est obligatoire.", MANIFESTE))?;
    if !nom_valide(&nom) {
        return Err(format!("nom de paquet « {} » invalide : lettres minuscules sans accent, chiffres, _ et - seulement (40 signes au plus).", nom));
    }
    let version = champ_texte(&v, "version").unwrap_or_else(|| "0.0.0".into());
    let principal = champ_texte(&v, "principal").unwrap_or_else(|| "principal.fa".into());
    let mut fichiers: Vec<String> = Vec::new();
    if let Valeur::Struct(d) = &v {
        if let Some(Valeur::Liste(l)) = d.borrow().get("fichiers") {
            for x in l.borrow().iter() {
                match x {
                    Valeur::Texte(t) => fichiers.push((**t).clone()),
                    _ => return Err(format!("{} : \"fichiers\" doit contenir uniquement des noms de fichiers.", MANIFESTE)),
                }
            }
        }
    }
    if fichiers.is_empty() {
        return Err(format!("{} : la liste \"fichiers\" est vide ou absente.", MANIFESTE));
    }
    if fichiers.len() > MAX_FICHIERS {
        return Err(format!("{} : trop de fichiers ({} au plus).", MANIFESTE, MAX_FICHIERS));
    }
    for f in &fichiers {
        chemin_valide(f)?;
    }
    chemin_valide(&principal)?;
    if !fichiers.contains(&principal) {
        return Err(format!("{} : le fichier principal « {} » n'est pas dans la liste \"fichiers\".", MANIFESTE, principal));
    }
    Ok(Manifeste { nom, version, principal, fichiers })
}

fn lire_projet() -> R<Vec<(String, String)>> {
    let p = Path::new(PROJET);
    if !p.exists() {
        return Ok(vec![]);
    }
    let t = std::fs::read_to_string(p).map_err(|e| format!("lecture de {} : {}", PROJET, e))?;
    let v = json_lire(&t).map_err(|e| format!("{} n'est pas du JSON valide ({}).", PROJET, e))?;
    let mut out = vec![];
    if let Valeur::Struct(d) = &v {
        if let Some(Valeur::Struct(dep)) = d.borrow().get("dependances") {
            let dep = dep.borrow();
            for k in dep.cles_ordonnees() {
                if let Some(Valeur::Texte(s)) = dep.get(&k) {
                    out.push((k, (**s).clone()));
                }
            }
        }
    }
    Ok(out)
}

fn ecrire_projet(deps: &[(String, String)]) -> R<()> {
    let d = Valeur::struct_de(deps.iter().map(|(k, s)| (k.as_str(), Valeur::texte(s))).collect());
    let v = Valeur::struct_de(vec![("dependances", d)]);
    let t = json_texte(&v, "  ").map_err(|_| "écriture de fadera.json impossible".to_string())?;
    std::fs::write(PROJET, t + "\n").map_err(|e| format!("écriture de {} : {}", PROJET, e))
}

// ------------------------------------------------------------------ sources
enum Source {
    Local(PathBuf),
    Web(String),
}

fn analyser_source(s: &str) -> R<Source> {
    if s.starts_with("http://") || s.starts_with("https://") {
        return Ok(Source::Web(s.trim_end_matches('/').to_string()));
    }
    if Path::new(s).is_dir() {
        return Ok(Source::Local(PathBuf::from(s)));
    }
    // utilisateur/depot[@version]
    let (depot, version) = match s.split_once('@') {
        Some((d, v)) => (d, v),
        None => (s, "main"),
    };
    let ok_part = |p: &str| !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
    let parts: Vec<&str> = depot.split('/').collect();
    if parts.len() == 2 && ok_part(parts[0]) && ok_part(parts[1]) && !version.is_empty() && version.chars().all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c)) {
        return Ok(Source::Web(format!("https://raw.githubusercontent.com/{}/{}/{}", parts[0], parts[1], version)));
    }
    Err(format!("source « {} » non reconnue : attendu utilisateur/depot[@version], une adresse https://… ou un dossier existant.", s))
}

fn lire_source(src: &Source, fichier: &str) -> R<String> {
    match src {
        Source::Local(d) => {
            let p = d.join(fichier);
            let m = std::fs::metadata(&p).map_err(|e| format!("« {} » : {}", p.display(), e))?;
            if m.len() as usize > MAX_OCTETS_FICHIER {
                return Err(format!("« {} » dépasse {} Ko.", fichier, MAX_OCTETS_FICHIER / 1000));
            }
            std::fs::read_to_string(&p).map_err(|e| format!("« {} » : {}", p.display(), e))
        }
        Source::Web(base) => {
            let url = format!("{}/{}", base, fichier);
            let (statut, corps) = crate::reseau::requete("GET", &url, None, &[]).map_err(|e| format!("téléchargement impossible de {} ({}).", url, e))?;
            if statut != 200 {
                return Err(format!("{} : le serveur a répondu {}.", url, statut));
            }
            if corps.len() > MAX_OCTETS_FICHIER {
                return Err(format!("« {} » dépasse {} Ko.", fichier, MAX_OCTETS_FICHIER / 1000));
            }
            Ok(corps)
        }
    }
}

// ------------------------------------------------------------------ commandes
/// Installe un paquet depuis `source` ; renvoie son nom. Tout est lu et vérifié avant la première écriture.
fn installer_un(source: &str) -> R<(String, String)> {
    let src = analyser_source(source)?;
    let m = lire_manifeste(&lire_source(&src, MANIFESTE)?)?;
    let mut contenus: Vec<(String, String)> = Vec::new();
    for f in &m.fichiers {
        contenus.push((f.clone(), lire_source(&src, f)?));
    }
    let dest = Path::new(DOSSIER).join(&m.nom);
    if dest.exists() {
        std::fs::remove_dir_all(&dest).map_err(|e| format!("remplacement de {} : {}", dest.display(), e))?;
    }
    for (f, c) in &contenus {
        let p = dest.join(f);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("création de {} : {}", parent.display(), e))?;
        }
        std::fs::write(&p, c).map_err(|e| format!("écriture de {} : {}", p.display(), e))?;
    }
    let man = lire_source(&src, MANIFESTE)?;
    std::fs::write(dest.join(MANIFESTE), man).map_err(|e| e.to_string())?;
    println!("✓ {} {} installé ({} fichier(s)).", m.nom, m.version, contenus.len());
    println!("  Dans un programme :  importer \"{}/{}/{}\" comme {}", DOSSIER, m.nom, m.principal.trim_end_matches(".fa"), m.nom.replace('-', "_"));
    Ok((m.nom, source.to_string()))
}

fn installer(args: &[String]) -> R<()> {
    let mut deps = lire_projet()?;
    if args.is_empty() {
        if deps.is_empty() {
            return Err(format!("rien à installer : précisez une source (fadera paquet installer utilisateur/depot) ou créez un {}.", PROJET));
        }
        for (_, s) in &deps.clone() {
            installer_un(s)?;
        }
        return Ok(());
    }
    if args.len() > 1 {
        return Err("une seule source à la fois.".into());
    }
    if args[0].starts_with("http") || args[0].contains('/') && !Path::new(&args[0]).is_dir() {
        println!("Attention : un paquet est du code qui s'exécute avec vos droits (fichiers, réseau). N'installez que des paquets dont vous avez confiance en l'auteur.");
    }
    let (nom, source) = installer_un(&args[0])?;
    match deps.iter_mut().find(|(k, _)| *k == nom) {
        Some(e) => e.1 = source,
        None => deps.push((nom, source)),
    }
    ecrire_projet(&deps)
}

fn lister() -> R<()> {
    let d = Path::new(DOSSIER);
    if !d.is_dir() {
        println!("Aucun paquet installé.");
        return Ok(());
    }
    let mut noms: Vec<String> = std::fs::read_dir(d).map_err(|e| e.to_string())?.filter_map(|e| e.ok()).filter(|e| e.path().is_dir()).map(|e| e.file_name().to_string_lossy().to_string()).collect();
    noms.sort();
    if noms.is_empty() {
        println!("Aucun paquet installé.");
    }
    for n in noms {
        let v = std::fs::read_to_string(d.join(&n).join(MANIFESTE)).ok().and_then(|t| lire_manifeste(&t).ok()).map(|m| m.version).unwrap_or_else(|| "?".into());
        println!("{} {}", n, v);
    }
    Ok(())
}

fn retirer(args: &[String]) -> R<()> {
    let nom = args.first().ok_or("usage : fadera paquet retirer nom")?;
    if !nom_valide(nom) {
        return Err(format!("nom de paquet « {} » invalide.", nom));
    }
    let d = Path::new(DOSSIER).join(nom);
    let mut deps = lire_projet()?;
    let avant = deps.len();
    deps.retain(|(k, _)| k != nom);
    if !d.is_dir() && deps.len() == avant {
        return Err(format!("le paquet « {} » n'est pas installé.", nom));
    }
    if d.is_dir() {
        std::fs::remove_dir_all(&d).map_err(|e| format!("suppression de {} : {}", d.display(), e))?;
    }
    if deps.len() != avant {
        ecrire_projet(&deps)?;
    }
    println!("✓ {} retiré.", nom);
    Ok(())
}

fn creer(args: &[String]) -> R<()> {
    let nom = args.first().ok_or("usage : fadera paquet creer nom")?;
    if !nom_valide(nom) {
        return Err(format!("nom de paquet « {} » invalide : lettres minuscules sans accent, chiffres, _ et -.", nom));
    }
    let d = Path::new(nom);
    if d.exists() {
        return Err(format!("« {} » existe déjà.", nom));
    }
    std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    let man = Valeur::struct_de(vec![
        ("nom", Valeur::texte(nom)),
        ("version", Valeur::texte("0.1.0")),
        ("description", Valeur::texte("Décrivez votre paquet en une phrase.")),
        ("principal", Valeur::texte("principal.fa")),
        ("fichiers", Valeur::liste(vec![Valeur::texte("principal.fa"), Valeur::texte("tests.fa"), Valeur::texte("LISEZMOI.md")])),
    ]);
    let ecrire = |f: &str, c: &str| std::fs::write(d.join(f), c).map_err(|e| format!("écriture de {} : {}", f, e));
    ecrire(MANIFESTE, &(json_texte(&man, "  ").map_err(|_| "JSON".to_string())? + "\n"))?;
    ecrire("principal.fa", "# Paquet « NOM » : les fonctions publiques sont celles que ce fichier déclare.\n\nfonction saluer(prenom)\n  renvoyer \"Bonjour \" + prenom + \" !\"\nfin\n".replace("NOM", nom).as_str())?;
    ecrire("tests.fa", "importer \"principal\" comme p\n\ntester \"saluer\"\n  verifier_egal(p.saluer(\"Awa\"), \"Bonjour Awa !\")\nfin\n")?;
    ecrire("LISEZMOI.md", &format!("# {}\n\nDécrivez ici ce que fait votre paquet.\n\n## Installation\n\n```\nfadera paquet installer votre-nom/{}\n```\n\n## Utilisation\n\n```\nimporter \"paquets/{}/principal\" comme p\nafficher p.saluer(\"Awa\")\n```\n", nom, nom, nom))?;
    println!("✓ Paquet créé dans {}/ : modifiez principal.fa, testez avec « fadera tester {}/tests.fa »,", nom, nom);
    println!("  puis publiez le dossier sur GitHub : vos utilisateurs feront « fadera paquet installer votre-nom/{} ».", nom);
    Ok(())
}
