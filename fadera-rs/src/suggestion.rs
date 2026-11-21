//! « Tu voulais écrire … ? » : suggestion affichée sous une erreur de nom (variable, fonction ou mot-clé mal écrit).
//! Même règle que l'IDE (index.html, `detaillerErreur`) : distance d'édition au plus égale au tiers de la longueur du mot,
//! accents et majuscules ignorés, candidats = noms déclarés dans le programme + fonctions connues + mots-clés.

use regex::Regex;
use unicode_normalization::UnicodeNormalization;

const MOTS_CLES: &[&str] = &[
    "soit", "si", "alors", "sinon", "fin", "afficher", "tantque", "faire", "fonction", "renvoyer", "saisir", "liste", "ajouter", "taille",
    "lireElement", "modifierElement", "modifierCle", "hasard", "structure", "lireCle", "pour", "allantde", "lireFichier", "ecrireFichier", "vrai",
    "faux", "non", "essayer", "attraper", "selon", "cas", "defaut", "rompre", "continuer", "importer",
];

/// Noms des fonctions documentées (docs/reference/fonctions.txt, intégré à la compilation).
fn natives() -> Vec<String> {
    let re = Regex::new(r"^([\p{L}_][\p{L}\p{N}_]*)").unwrap();
    include_str!("../../docs/reference/fonctions.txt")
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| re.captures(l).map(|c| c[1].to_string()))
        .collect()
}

fn sans_accent(t: &str) -> Vec<char> {
    t.nfd().filter(|c| !('\u{300}'..='\u{36f}').contains(c)).collect::<String>().to_lowercase().chars().collect()
}

/// Distance d'édition (insertion, suppression, remplacement, échange de deux lettres voisines).
fn distance(a: &[char], b: &[char]) -> usize {
    let (m, n) = (a.len(), b.len());
    let mut d = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m {
        d[i][0] = i;
    }
    for j in 0..=n {
        d[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            d[i][j] = (d[i - 1][j] + 1).min(d[i][j - 1] + 1).min(d[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1]));
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }
    d[m][n]
}

fn plus_proche(mot: &str, candidats: &[String]) -> Option<String> {
    let m = sans_accent(mot);
    let longueur = mot.chars().count();
    let seuil = (longueur / 3).max(1);
    let mut meilleur: Option<&String> = None;
    let mut d_min = usize::MAX;
    for c in candidats {
        if c == mot {
            continue;
        }
        let cs = sans_accent(c);
        let d = if cs == m {
            0
        } else if longueur < 3 {
            usize::MAX
        } else {
            distance(&m, &cs)
        };
        if d <= seuil && d < d_min {
            d_min = d;
            meilleur = Some(c);
        }
    }
    meilleur.cloned()
}

/// Remplace le contenu des textes par des espaces et coupe au commentaire.
fn sans_texte(l: &str) -> String {
    let c: Vec<char> = l.chars().collect();
    let mut o = String::new();
    let mut dans = false;
    let mut i = 0;
    while i < c.len() {
        let x = c[i];
        if dans {
            if x == '\\' {
                o.push_str("  ");
                i += 1;
            } else if x == '"' {
                dans = false;
                o.push('"');
            } else {
                o.push(' ');
            }
            i += 1;
            continue;
        }
        if x == '"' {
            dans = true;
            o.push('"');
            i += 1;
            continue;
        }
        if x == '#' || (x == '/' && c.get(i + 1) == Some(&'/')) {
            break;
        }
        o.push(x);
        i += 1;
    }
    o
}

/// Noms déclarés dans le programme : (variables, fonctions).
fn noms_declares(code: &str) -> (Vec<String>, Vec<String>) {
    let propre = code.split('\n').map(sans_texte).collect::<Vec<_>>().join("\n");
    let id = r"[\p{L}_][\p{L}\p{N}_]*";
    let rv = Regex::new(&format!(r"(?:^|[^\p{{L}}\p{{N}}_])(?:soit|structure|liste|pour|chaque|attraper)\s+({})", id)).unwrap();
    let rf = Regex::new(&format!(r"(?:^|[^\p{{L}}\p{{N}}_])fonction\s+({})\s*\(([^)]*)\)", id)).unwrap();
    let mut vars: Vec<String> = Vec::new();
    let mut fns: Vec<String> = Vec::new();
    for c in rv.captures_iter(&propre) {
        vars.push(c[1].to_string());
    }
    for c in rf.captures_iter(&propre) {
        fns.push(c[1].to_string());
        for p in c[2].split(',').map(|x| x.trim()).filter(|x| !x.is_empty()) {
            vars.push(p.to_string());
        }
    }
    (vars, fns)
}

/// Ligne (à partir de 1) où `mot` est déclaré plus bas que `ligne`, sinon 0.
fn declaree_apres(lignes: &[&str], ligne: usize, mot: &str) -> usize {
    let re = Regex::new(&format!(r"(?:^|[^\p{{L}}\p{{N}}_])(?:soit|structure|liste)\s+{}(?:[^\p{{L}}\p{{N}}_]|$)", regex::escape(mot))).unwrap();
    for i in ligne..lignes.len() {
        if re.is_match(&sans_texte(lignes[i])) {
            return i + 1;
        }
    }
    0
}

/// Aides à afficher sous le message d'erreur (liste vide si rien à suggérer). `message` : sans le préfixe « Ligne N : ».
pub fn aides(message: &str, ligne: Option<usize>, code: &str) -> Vec<String> {
    let mut genre: Option<(&str, String)> = None;
    let cherche = |motif: &str| Regex::new(motif).unwrap().captures(message).map(|c| c[1].to_string());
    if let Some(m) = cherche(r#"^La variable "(.+?)" n'existe pas"#) {
        genre = Some(("variable", m));
    } else if let Some(m) = cherche(r#"^Fonction "(.+?)" inconnue"#) {
        genre = Some(("fonction", m));
    } else if let Some(m) = cherche(r#"^Instruction inconnue : "(.+?)"\."#) {
        genre = Some(("instruction", m));
    }
    let mut aide: Vec<String> = Vec::new();
    if let Some((g, mot)) = genre {
        let (vars, fns) = noms_declares(code);
        let nat = natives();
        let cles: Vec<String> = MOTS_CLES.iter().map(|s| s.to_string()).collect();
        let lignes: Vec<&str> = code.split('\n').collect();
        match g {
            "variable" => {
                let apres = ligne.map_or(0, |l| declaree_apres(&lignes, l, &mot));
                let cand: Vec<String> = vars.iter().chain(fns.iter()).chain(nat.iter()).cloned().collect();
                if apres > 0 {
                    aide.push(format!("« {} » n'est déclarée qu'à la ligne {} : place cette déclaration avant son utilisation.", mot, apres));
                } else if let Some(p) = plus_proche(&mot, &cand) {
                    aide.push(format!("Tu voulais écrire « {} » ?", p));
                }
            }
            "fonction" => {
                let cand: Vec<String> = fns.iter().chain(nat.iter()).chain(cles.iter()).cloned().collect();
                if let Some(p) = plus_proche(&mot, &cand) {
                    aide.push(format!("Tu voulais écrire « {} » ?", p));
                }
            }
            _ => {
                let cand: Vec<String> = cles.iter().chain(fns.iter()).cloned().collect();
                if let Some(p) = plus_proche(&mot, &cand) {
                    aide.push(format!("Tu voulais écrire « {} » ?", p));
                }
            }
        }
    }
    aide
}
