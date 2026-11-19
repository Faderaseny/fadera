use crate::erreur::ErreurFadera;
use crate::valeur::js_num;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Type {
    MotCle,
    Identifiant,
    Numero,
    Chaine,
    Operateur,
    Symbole,
    Logique,
}

#[derive(Clone, Debug)]
pub struct Jeton {
    pub t: Type,
    pub valeur: String,
    pub num: f64,
    pub ligne: usize,
    /// Position (en caractères) du premier et du dernier caractère + 1 dans le texte source.
    pub deb: usize,
    pub fin: usize,
}

/// Commentaire (`# …` ou `// …`) : ligne de départ et étendue en caractères (fin de ligne exclue).
#[derive(Clone, Debug)]
pub struct Commentaire {
    pub ligne: usize,
    pub deb: usize,
    pub fin: usize,
}

pub const MOTS_CLES: &[&str] = &[
    "soit", "fnf", "si", "alors", "sinon", "fin", "afficher", "tantque", "faire", "fonction", "renvoyer",
    "saisir", "liste", "ajouter", "taille", "lireElement", "modifierElement", "modifierCle", "hasard",
    "structure", "lireCle", "pour", "allantde", "lireFichier", "ecrireFichier", "vrai", "faux", "non",
    "essayer", "attraper", "selon", "cas", "defaut", "rompre", "continuer", "importer",
];

/// Mots qui désignent une fonction du langage seulement quand une parenthèse ouvrante suit : ailleurs, ce sont des noms libres.
const MOTS_APPEL: &[&str] = &[
    "liste", "taille", "lireElement", "modifierElement", "modifierCle", "hasard", "lireCle", "lireFichier", "ecrireFichier", "ajouter", "saisir",
];

/// Écritures françaises (accents, snake_case) des fonctions du langage, ramenées à leur forme interne ; seulement dans un appel `nom(…)`.
const ALIAS_APPEL: &[(&str, &str)] = &[
    ("écrire_fichier", "ecrireFichier"),
    ("ecrire_fichier", "ecrireFichier"),
    ("lire_fichier", "lireFichier"),
    ("lire_élément", "lireElement"),
    ("lire_element", "lireElement"),
    ("modifier_élément", "modifierElement"),
    ("modifier_element", "modifierElement"),
    ("modifier_clé", "modifierCle"),
    ("modifier_cle", "modifierCle"),
    ("lire_clé", "lireCle"),
    ("lire_cle", "lireCle"),
    ("masque_réseau", "masque_reseau"),
    ("plage_réseau", "plage_reseau"),
    ("créer_réseau", "creer_reseau"),
    ("ia_entraîner", "ia_entrainer"),
    ("ia_prédire", "ia_predire"),
];

/// Vrai si le programme déclare lui-même une fonction `longueur` : le synonyme `longueur(x)` = `taille(x)` est alors désactivé.
fn definit_longueur(s: &[char]) -> bool {
    let mot: Vec<char> = "longueur".chars().collect();
    let est_car = |c: char| c.is_alphanumeric() || c == '_';
    let mut i = 0;
    while i + mot.len() <= s.len() {
        if s[i..i + mot.len()] == mot[..] && (i == 0 || !(est_car(s[i - 1]) || s[i - 1] == '.')) && !s.get(i + mot.len()).map_or(false, |c| est_car(*c) || *c == '.') {
            let mut k = i;
            while k > 0 && (s[k - 1] == ' ' || s[k - 1] == '\t') {
                k -= 1;
            }
            let f: Vec<char> = "fonction".chars().collect();
            if k > i.saturating_sub(64) && k >= f.len() && k < i && s[k - f.len()..k] == f[..] && (k == f.len() || !est_car(s[k - f.len() - 1])) {
                return true;
            }
        }
        i += 1;
    }
    false
}

/// Premier caractère non blanc (retours à la ligne compris) à partir de `i`.
fn prochain_non_blanc(s: &[char], mut i: usize) -> Option<char> {
    while i < s.len() && (s[i].is_whitespace() || s[i] == '\u{feff}') {
        i += 1;
    }
    s.get(i).copied()
}

/// Premier caractère qui n'est ni une espace ni une tabulation, sur la même ligne.
fn prochain_sur_la_ligne(s: &[char], mut i: usize) -> Option<char> {
    while i < s.len() && (s[i] == ' ' || s[i] == '\t') {
        i += 1;
    }
    s.get(i).copied().filter(|c| *c != '\n' && *c != '\r')
}

/// Si le mot `mot` suit, après au moins une espace sur la même ligne, renvoie la position qui suit ce mot.
fn suit_mot(s: &[char], i: usize, mot: &str) -> Option<usize> {
    let mut k = i;
    while k < s.len() && (s[k] == ' ' || s[k] == '\t') {
        k += 1;
    }
    if k == i {
        return None;
    }
    let m: Vec<char> = mot.chars().collect();
    if k + m.len() > s.len() || s[k..k + m.len()] != m[..] {
        return None;
    }
    let f = k + m.len();
    match s.get(f) {
        Some(c) if c.is_alphanumeric() || *c == '_' || *c == '.' => None,
        _ => Some(f),
    }
}

pub fn decrire(j: Option<&Jeton>) -> String {
    match j {
        None => "la fin du code".into(),
        Some(j) => match j.t {
            Type::Chaine => format!("le texte \"{}\"", j.valeur),
            Type::Numero => format!("le nombre {}", js_num(j.num)),
            _ => format!("\"{}\"", j.valeur),
        },
    }
}

pub fn lexer(src: &str) -> Result<Vec<Jeton>, ErreurFadera> {
    lexer_comm(src, None)
}

/// Comme `lexer`, et range en plus les commentaires rencontrés dans `comm` (utilisé par le formateur et la coloration).
pub fn lexer_comm(src: &str, mut comm: Option<&mut Vec<Commentaire>>) -> Result<Vec<Jeton>, ErreurFadera> {
    let s: Vec<char> = src.chars().collect();
    let n = s.len();
    let mut toks: Vec<Jeton> = Vec::new();
    let mut i = 0;
    let mut ligne = 1usize;
    let est_debut = |c: char| c.is_alphabetic() || c == '_';
    let est_car = |c: char| c.is_alphanumeric() || c == '_';
    let sans_longueur = definit_longueur(&s);
    let mut deb_tour = 0usize;
    let mut nb_avant = 0usize;
    while i < n {
        // le jeton éventuellement posé au tour précédent s'étend de `deb_tour` à `i`
        if toks.len() > nb_avant {
            let k = toks.len() - 1;
            toks[k].deb = deb_tour;
            toks[k].fin = i;
        }
        deb_tour = i;
        nb_avant = toks.len();
        let c = s[i];
        if c == '\n' {
            ligne += 1;
            i += 1;
            continue;
        }
        if c.is_whitespace() || c == '\u{feff}' {
            i += 1;
            continue;
        }
        if c == '#' || (c == '/' && i + 1 < n && s[i + 1] == '/') {
            while i < n && s[i] != '\n' {
                i += 1;
            }
            if let Some(cv) = comm.as_deref_mut() {
                cv.push(Commentaire { ligne, deb: deb_tour, fin: i });
            }
            continue;
        }
        if c == '"' || c == '\'' {
            let l0 = ligne;
            let mut v = String::new();
            i += 1;
            while i < n && s[i] != c {
                if s[i] == '\\' && i + 1 < n {
                    let r = match s[i + 1] {
                        'n' => Some('\n'),
                        't' => Some('\t'),
                        '"' => Some('"'),
                        '\'' => Some('\''),
                        '\\' => Some('\\'),
                        _ => None,
                    };
                    if let Some(r) = r {
                        v.push(r);
                        i += 2;
                        continue;
                    }
                }
                if s[i] == '\n' {
                    ligne += 1;
                }
                v.push(s[i]);
                i += 1;
            }
            if i >= n {
                return Err(ErreurFadera::new("Texte non fermé : il manque le guillemet de fermeture.", Some(l0)));
            }
            i += 1;
            toks.push(Jeton { t: Type::Chaine, valeur: v, num: 0.0, ligne: l0, deb: 0, fin: 0 });
            continue;
        }
        if c.is_ascii_digit() {
            let mut j = i;
            while j < n && s[j].is_ascii_digit() {
                j += 1;
            }
            if j < n && s[j] == '.' && j + 1 < n && s[j + 1].is_ascii_digit() {
                j += 1;
                while j < n && s[j].is_ascii_digit() {
                    j += 1;
                }
            }
            // notation scientifique : 1e7, 2.5E-3, 4e+2 (seulement si un chiffre suit)
            if j < n && (s[j] == 'e' || s[j] == 'E') {
                let a = s.get(j + 1).copied().unwrap_or(' ');
                let b = s.get(j + 2).copied().unwrap_or(' ');
                if a.is_ascii_digit() || ((a == '+' || a == '-') && b.is_ascii_digit()) {
                    j += 2;
                    while j < n && s[j].is_ascii_digit() {
                        j += 1;
                    }
                }
            }
            let txt: String = s[i..j].iter().collect();
            toks.push(Jeton { t: Type::Numero, valeur: txt.clone(), num: txt.parse().unwrap_or(f64::NAN), ligne, deb: 0, fin: 0 });
            i = j;
            continue;
        }
        if est_debut(c) {
            let mut j = i + 1;
            // « o.double » : un identifiant peut contenir des points (espace de nom d'un module, champ d'une structure)
            while j < n && (est_car(s[j]) || (s[j] == '.' && j + 1 < n && est_debut(s[j + 1]))) {
                j += 1;
            }
            let mut v: String = s[i..j].iter().collect();
            // Formes françaises correctes, ramenées à la forme interne : « défaut », « tant que », « allant de ».
            if v == "défaut" {
                v = "defaut".into();
            } else if v == "tant" {
                if let Some(f) = suit_mot(&s, j, "que") {
                    v = "tantque".into();
                    j = f;
                }
            } else if v == "allant" {
                if let Some(f) = suit_mot(&s, j, "de") {
                    v = "allantde".into();
                    j = f;
                }
            }
            if let Some((_, canon)) = ALIAS_APPEL.iter().find(|(a, _)| *a == v) {
                if prochain_non_blanc(&s, j) == Some('(') {
                    v = (*canon).to_string();
                }
            }
            if v == "longueur" && !sans_longueur && prochain_non_blanc(&s, j) == Some('(') && !matches!(toks.last(), Some(Jeton { t: Type::Symbole, valeur, .. }) if valeur == ".") {
                v = "taille".into(); // synonyme français : longueur(x) = taille(x)
            }
            let t = if v == "et" || v == "ou" {
                Type::Logique
            } else if MOTS_APPEL.contains(&v.as_str()) {
                // « liste », « taille », « hasard »… ne sont des mots du langage que dans un appel `taille(x)` : ailleurs ce sont des noms ordinaires.
                if prochain_non_blanc(&s, j) == Some('(') { Type::MotCle } else { Type::Identifiant }
            } else if v == "structure" {
                // `structure nom = …` déclare ; un `structure` seul est un nom ordinaire.
                if prochain_sur_la_ligne(&s, j).map_or(false, est_debut) { Type::MotCle } else { Type::Identifiant }
            } else if MOTS_CLES.contains(&v.as_str()) {
                Type::MotCle
            } else {
                Type::Identifiant
            };
            toks.push(Jeton { t, valeur: v, num: 0.0, ligne, deb: 0, fin: 0 });
            i = j;
            continue;
        }
        let deux: String = s[i..(i + 2).min(n)].iter().collect();
        if deux == "=>" {
            toks.push(Jeton { t: Type::Symbole, valeur: deux, num: 0.0, ligne, deb: 0, fin: 0 });
            i += 2;
            continue;
        }
        if deux == "==" || deux == "!=" || deux == ">=" || deux == "<=" {
            toks.push(Jeton { t: Type::Operateur, valeur: deux, num: 0.0, ligne, deb: 0, fin: 0 });
            i += 2;
            continue;
        }
        if deux == "**" {
            toks.push(Jeton { t: Type::Operateur, valeur: deux, num: 0.0, ligne, deb: 0, fin: 0 });
            i += 2;
            continue;
        }
        if "=+-*/%<>".contains(c) {
            toks.push(Jeton { t: Type::Operateur, valeur: c.to_string(), num: 0.0, ligne, deb: 0, fin: 0 });
            i += 1;
            continue;
        }
        if c == '!' {
            toks.push(Jeton { t: Type::MotCle, valeur: "non".into(), num: 0.0, ligne, deb: 0, fin: 0 });
            i += 1;
            continue;
        }
        // « . » collé à la fin d'un appel ou d'un index : enchaînement `f(x).champ` / `f(x).methode(y)`
        if c == '.' && matches!(toks.last(), Some(Jeton { t: Type::Symbole, valeur, .. }) if valeur == ")" || valeur == "]") {
            toks.push(Jeton { t: Type::Symbole, valeur: ".".to_string(), num: 0.0, ligne, deb: 0, fin: 0 });
            i += 1;
            continue;
        }
        if "(),{}:[]".contains(c) {
            toks.push(Jeton { t: Type::Symbole, valeur: c.to_string(), num: 0.0, ligne, deb: 0, fin: 0 });
            i += 1;
            continue;
        }
        if c == ';' {
            i += 1;
            continue;
        }
        return Err(ErreurFadera::new(format!("Caractère inattendu \"{}\".", c), Some(ligne)));
    }
    if toks.len() > nb_avant {
        let k = toks.len() - 1;
        toks[k].deb = deb_tour;
        toks[k].fin = n;
    }
    Ok(toks)
}
