//! Formateur de code Fadéra (`fadera fmt`).
//!
//! Il s'appuie sur le vrai lexeur (positions des jetons, commentaires) et sur l'analyseur : un code qui ne
//! s'analyse pas n'est jamais reformaté. Il ne déplace aucun jeton d'une ligne à l'autre (certaines
//! constructions dépendent des lignes : `sinon si`, `attraper e`, `importer … comme`), ne change que les
//! espaces, l'indentation et les lignes vides, et vérifie à la fin que le texte produit se découpe en
//! exactement les mêmes jetons que l'original. Le moteur JavaScript de l'IDE contient la même logique.
use crate::erreur::ErreurFadera;
use crate::lexer::{lexer_comm, Commentaire, Jeton, Type};

/// Mots-clés qui s'utilisent comme des fonctions : `taille(l)`, pas `taille (l)`.
const MOTS_APPEL: &[&str] = &[
    "taille", "ajouter", "liste", "saisir", "hasard", "lireElement", "lireCle", "modifierElement", "modifierCle",
    "ecrireFichier", "lireFichier", "fnf",
];
/// Mots-clés dont l'espace avant « ( » est laissé comme l'auteur l'a écrit.
const MOTS_LIBRES: &[&str] = &["afficher", "renvoyer"];

#[derive(Clone, Copy, PartialEq)]
enum Bloc {
    Normal,
    Cas,
}

struct J {
    t: Type,
    v: String,
    txt: String,
    ligne: usize,
    fin_ligne: usize,
    deb: usize,
    fin: usize,
    unaire: bool,
    bang: bool,
    /// « dans » ou « pas » d'une boucle `pour` : mot de syntaxe, pas une valeur.
    ctx: bool,
}

struct Ligne {
    toks: Vec<usize>,
    comm: Option<Commentaire>,
    debut: usize,
    fin: usize,
    // résultats du calcul d'indentation
    indent: usize,
    interieur: usize,
}

struct Rang {
    // une « rangée » : soit du code (toks non vide), soit un commentaire seul
    l: Ligne,
}

fn est_ouvrant(j: &J) -> bool {
    j.t == Type::Symbole && (j.v == "(" || j.v == "[" || j.v == "{")
}
fn est_fermant(j: &J) -> bool {
    j.t == Type::Symbole && (j.v == ")" || j.v == "]" || j.v == "}")
}
fn est_mot(j: &J, m: &str) -> bool {
    j.t == Type::MotCle && j.v == m
}
fn est_id(j: &J, m: &str) -> bool {
    j.t == Type::Identifiant && j.v == m
}

/// Formate `src` ; `indentation` = nombre d'espaces par niveau (4 par défaut côté commande).
pub fn formater_source(src: &str, indentation: usize) -> Result<String, ErreurFadera> {
    let chars: Vec<char> = src.chars().collect();
    let mut comms: Vec<Commentaire> = Vec::new();
    let jetons: Vec<Jeton> = lexer_comm(src, Some(&mut comms))?;
    // le code doit s'analyser : on ne reformate pas un code invalide
    crate::interp::analyser_code(src)?;
    let eol = if src.contains("\r\n") { "\r\n" } else { "\n" };

    // ---- jetons enrichis
    let mut js: Vec<J> = Vec::with_capacity(jetons.len());
    for (i, t) in jetons.iter().enumerate() {
        let txt: String = chars[t.deb..t.fin].iter().collect();
        let fin_ligne = t.ligne + txt.chars().filter(|&c| c == '\n').count();
        let bang = t.t == Type::MotCle && t.valeur == "non" && txt == "!";
        let _ = i;
        js.push(J { t: t.t, v: t.valeur.clone(), txt, ligne: t.ligne, fin_ligne, deb: t.deb, fin: t.fin, unaire: false, bang, ctx: false });
    }
    // « dans » / « pas » d'une ligne `pour …`
    for i in 0..js.len() {
        if js[i].t == Type::Identifiant && (js[i].v == "dans" || js[i].v == "pas") {
            let mut k = i;
            while k > 0 && js[k - 1].ligne == js[i].ligne {
                k -= 1;
            }
            js[i].ctx = js[k].t == Type::MotCle && js[k].v == "pour";
        }
    }
    // opérateur unaire : - ou + qui ne suit pas une valeur
    for i in 0..js.len() {
        if js[i].t == Type::Operateur && (js[i].v == "-" || js[i].v == "+") {
            let valeur_avant = if i == 0 {
                false
            } else {
                let p = &js[i - 1];
                match p.t {
                    Type::Numero | Type::Chaine => true,
                    Type::Identifiant => !p.ctx,
                    Type::Symbole => p.v == ")" || p.v == "]" || p.v == "}",
                    Type::MotCle => p.v == "vrai" || p.v == "faux",
                    _ => false,
                }
            };
            js[i].unaire = !valeur_avant;
        }
    }

    // ---- rangées : lignes de code (avec commentaire de fin éventuel) et commentaires seuls
    let mut rangs: Vec<Rang> = Vec::new();
    let (mut it, mut ic) = (0usize, 0usize);
    while it < js.len() || ic < comms.len() {
        let prend_jeton = ic >= comms.len() || (it < js.len() && js[it].deb < comms[ic].deb);
        if prend_jeton {
            let j = &js[it];
            let suite = match rangs.last() {
                Some(r) => !r.l.toks.is_empty() && r.l.comm.is_none() && j.ligne <= r.l.fin,
                None => false,
            };
            if suite {
                let r = rangs.last_mut().unwrap();
                r.l.toks.push(it);
                if j.fin_ligne > r.l.fin {
                    r.l.fin = j.fin_ligne;
                }
            } else {
                rangs.push(Rang { l: Ligne { toks: vec![it], comm: None, debut: j.ligne, fin: j.fin_ligne, indent: 0, interieur: 0 } });
            }
            it += 1;
        } else {
            let c = comms[ic].clone();
            let attache = match rangs.last() {
                Some(r) => !r.l.toks.is_empty() && r.l.comm.is_none() && r.l.fin == c.ligne,
                None => false,
            };
            if attache {
                rangs.last_mut().unwrap().l.comm = Some(c);
            } else {
                rangs.push(Rang { l: Ligne { toks: vec![], debut: c.ligne, fin: c.ligne, comm: Some(c), indent: 0, interieur: 0 } });
            }
            ic += 1;
        }
    }

    // ---- indentation
    let mut pile: Vec<Bloc> = Vec::new();
    let mut crochets: Vec<usize> = Vec::new(); // numéro de rangée où chaque crochet a été ouvert
    let mut derniere: Option<usize> = None; // dernier jeton vu (hors commentaires)
    for (ri, r) in rangs.iter_mut().enumerate() {
        if r.l.toks.is_empty() {
            continue;
        }
        let premier = &js[r.l.toks[0]];
        let n = pile.len();
        let sommet_cas = matches!(pile.last(), Some(Bloc::Cas));
        let niveau = if est_mot(premier, "fin") {
            n.saturating_sub(if sommet_cas { 2 } else { 1 })
        } else if est_mot(premier, "sinon") || est_mot(premier, "attraper") {
            n.saturating_sub(1)
        } else if est_mot(premier, "cas") || est_mot(premier, "defaut") {
            if sommet_cas {
                n - 1
            } else {
                n
            }
        } else {
            n
        };
        // niveaux de crochets, après les fermants placés en tête de rangée
        let mut k = 0;
        while k < r.l.toks.len() && est_fermant(&js[r.l.toks[k]]) {
            k += 1;
        }
        let restants = crochets.len().saturating_sub(k);
        let mut distincts: Vec<usize> = crochets[..restants].to_vec();
        distincts.dedup();
        let extra = distincts.len();
        let continuation = crochets.is_empty()
            && match derniere {
                Some(d) => {
                    let p = &js[d];
                    p.t == Type::Operateur || p.t == Type::Logique || (p.t == Type::Symbole && p.v == "=>")
                }
                None => false,
            }
            || (crochets.is_empty() && premier.t == Type::Logique && derniere.is_some());
        r.l.indent = niveau + extra + if continuation { 1 } else { 0 };
        r.l.interieur = n + extra + if continuation { 1 } else { 0 };
        // mise à jour de l'état, jeton par jeton
        for (pos, &ti) in r.l.toks.iter().enumerate() {
            let j = &js[ti];
            let precedent = if pos > 0 { Some(&js[r.l.toks[pos - 1]]) } else { None };
            let suivant = r.l.toks.get(pos + 1).map(|&x| &js[x]);
            if est_ouvrant(j) {
                crochets.push(ri);
            } else if est_fermant(j) {
                crochets.pop();
            } else if j.t == Type::MotCle {
                match j.v.as_str() {
                    "si" => {
                        let apres_sinon = matches!(precedent, Some(p) if est_mot(p, "sinon"));
                        if !apres_sinon {
                            pile.push(Bloc::Normal);
                        }
                    }
                    "tantque" | "pour" | "fonction" | "selon" | "essayer" => pile.push(Bloc::Normal),
                    "fin" => {
                        if matches!(pile.last(), Some(Bloc::Cas)) {
                            pile.pop();
                        }
                        pile.pop();
                    }
                    "cas" | "defaut" => {
                        if matches!(pile.last(), Some(Bloc::Cas)) {
                            pile.pop();
                        }
                        pile.push(Bloc::Cas);
                    }
                    _ => {}
                }
            } else if est_id(j, "classe") && matches!(suivant, Some(s) if s.t == Type::Identifiant) {
                pile.push(Bloc::Normal);
            } else if est_id(j, "tester") && matches!(suivant, Some(s) if s.t == Type::Chaine) {
                pile.push(Bloc::Normal);
            }
            derniere = Some(ti);
        }
    }

    // ---- espacement entre deux jetons d'une même rangée
    let espace_avant = |p: Option<&J>, c: &J, gap_vide: bool| -> bool {
        let p = match p {
            None => return false,
            Some(p) => p,
        };
        let c_ferme = c.t == Type::Symbole && matches!(c.v.as_str(), "," | ")" | "]" | "." | ":");
        if c_ferme {
            return false;
        }
        if p.t == Type::Symbole && (p.v == "(" || p.v == "[" || p.v == ".") {
            return false;
        }
        if p.bang || (p.t == Type::Operateur && p.unaire) {
            return false;
        }
        if p.t == Type::Symbole && p.v == "{" {
            return !(c.t == Type::Symbole && c.v == "}");
        }
        if c.t == Type::Symbole && c.v == "}" {
            return true;
        }
        if p.t == Type::Symbole && (p.v == "," || p.v == ":") {
            return true;
        }
        if c.t == Type::Symbole && c.v == "(" {
            return match p.t {
                Type::Identifiant => false,
                Type::Symbole => !(p.v == ")" || p.v == "]"),
                Type::MotCle => {
                    if MOTS_APPEL.contains(&p.v.as_str()) {
                        false
                    } else if MOTS_LIBRES.contains(&p.v.as_str()) {
                        !gap_vide
                    } else {
                        true
                    }
                }
                _ => true,
            };
        }
        if c.t == Type::Symbole && c.v == "[" {
            return !((p.t == Type::Identifiant && !p.ctx) || (p.t == Type::Symbole && (p.v == ")" || p.v == "]")));
        }
        true
    };

    // ---- production du texte
    let mut sortie = String::new();
    let mut precedent_fin: Option<usize> = None; // dernière ligne source écrite
    let nb = rangs.len();
    for ri in 0..nb {
        let (indent_lig, texte_code, commentaire): (usize, String, Option<String>) = {
            let r = &rangs[ri];
            let commentaire = r.l.comm.as_ref().map(|c| chars[c.deb..c.fin].iter().collect::<String>().trim_end().to_string());
            if r.l.toks.is_empty() {
                // commentaire seul : indenté comme la rangée de code suivante
                let mut ind = 0;
                for r2 in rangs.iter().skip(ri + 1) {
                    if !r2.l.toks.is_empty() {
                        let f = &js[r2.l.toks[0]];
                        ind = if est_mot(f, "fin") || est_mot(f, "sinon") || est_mot(f, "attraper") { r2.l.interieur } else { r2.l.indent };
                        break;
                    }
                }
                (ind, String::new(), commentaire)
            } else {
                let mut t = String::new();
                for (pos, &ti) in r.l.toks.iter().enumerate() {
                    let j = &js[ti];
                    if pos > 0 {
                        let pj = &js[r.l.toks[pos - 1]];
                        let gap: String = chars[pj.fin..j.deb].iter().collect();
                        let point_virgule = gap.contains(';');
                        if point_virgule {
                            t.push(';');
                            t.push(' ');
                        } else if espace_avant(Some(pj), j, gap.is_empty()) {
                            t.push(' ');
                        }
                    }
                    t.push_str(&j.txt);
                }
                (r.l.indent, t, commentaire)
            }
        };
        let r = &rangs[ri];
        // lignes vides : au plus une, jamais en tête de fichier
        if let Some(pf) = precedent_fin {
            if r.l.debut > pf + 1 {
                sortie.push_str(eol);
            }
        }
        sortie.push_str(&" ".repeat(indent_lig * indentation));
        sortie.push_str(&texte_code);
        if let Some(c) = commentaire {
            if !texte_code.is_empty() {
                // écart conservé s'il était d'au moins 2 espaces
                let dernier = &js[*r.l.toks.last().unwrap()];
                let debut_c = r.l.comm.as_ref().unwrap().deb;
                let gap: String = chars[dernier.fin..debut_c].iter().collect();
                let n = gap.chars().filter(|&x| x == ' ').count();
                let n = if gap.chars().all(|x| x == ' ') && n >= 2 { n } else { 2 };
                sortie.push_str(&" ".repeat(n));
            }
            sortie.push_str(&c);
        }
        sortie.push_str(eol);
        precedent_fin = Some(r.l.fin);
    }

    // ---- garde-fou : mêmes jetons avant et après
    let apres = lexer_comm(&sortie, None).map_err(|e| ErreurFadera::new(format!("Formatage annulé (le texte produit ne se lit plus : {}).", e.message), None))?;
    let meme = apres.len() == jetons.len()
        && apres.iter().zip(jetons.iter()).all(|(a, b)| a.t == b.t && a.valeur == b.valeur && (a.num == b.num || (a.num.is_nan() && b.num.is_nan())));
    if !meme {
        return Err(ErreurFadera::new("Formatage annulé : le texte produit ne correspond plus aux jetons de l'original.", None));
    }
    Ok(sortie)
}
