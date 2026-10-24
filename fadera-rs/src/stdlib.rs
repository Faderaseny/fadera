//! Bibliothèque standard étendue : texte, listes, structures, ordre supérieur, regex, JSON, dates, maths.

use regex::{Regex, RegexBuilder};

use crate::erreur::*;
use crate::interp::Interp;
use crate::natives::{arg, js_pow, vec_de, NatFn};
use crate::util::*;
use crate::valeur::*;

pub fn trouver(nom: &str) -> Option<NatFn> {
    Some(match nom {
        "majuscule" => majuscule,
        "minuscule" => minuscule,
        "nettoyer" => nettoyer,
        "decouper" => decouper,
        "joindre" => joindre,
        "remplacer" => remplacer,
        "contient" => contient,
        "commence_par" => commence_par,
        "termine_par" => termine_par,
        "sous_texte" => sous_texte,
        "position" => position,
        "repeter" => repeter,
        "inverser" => inverser,
        "caractere" => caractere,
        "code_caractere" => code_caractere,
        "depuis_code" => depuis_code,
        "est_nombre" => est_nombre,
        "type_de" => type_de_f,
        "plage" => plage,
        "tranche" => tranche,
        "unique" => unique,
        "inserer" => inserer,
        "retirer" => retirer,
        "concatener" => concatener,
        "aplatir" => aplatir,
        "compter" => compter,
        "cles" => cles,
        "valeurs" => valeurs,
        "paires" => paires,
        "a_cle" => a_cle,
        "supprimer_cle" => supprimer_cle,
        "fusionner" => fusionner,
        "structure_depuis_paires" => structure_depuis_paires,
        "filtrer" => filtrer,
        "transformer" => transformer,
        "reduire" => reduire,
        "trouver" => trouver_f,
        "tous" => tous,
        "existe" => existe_f,
        "trier_par" => trier_par,
        "trier" => trier,
        "appliquer" => appliquer,
        "regex_test" => regex_test,
        "regex_trouver" => regex_trouver,
        "regex_remplacer" => regex_remplacer,
        "regex_groupes" => regex_groupes,
        "regex_decouper" => regex_decouper,
        "regex_echapper" => regex_echapper,
        "json_valide" => json_valide,
        "json_chemin" => json_chemin,
        "json_fusionner" => json_fusionner,
        "json_texte" => json_texte_f,
        "json_lire" => json_lire_f,
        "maintenant" => maintenant,
        "date_iso" => date_iso,
        "formater_date" => formater_date,
        "plancher" => plancher,
        "plafond" => plafond,
        "entier" => entier,
        "minimum" => minimum,
        "maximum" => maximum,
        _ => return crate::modules::trouver(nom),
    })
}

fn n(x: f64) -> Valeur {
    Valeur::Nombre(x)
}
fn t(s: impl AsRef<str>) -> Valeur {
    Valeur::texte(s)
}
fn pos_txt(p: Option<usize>) -> usize {
    p.unwrap_or(1)
}

fn texte_req(a: &[Valeur], i: usize, nom: &str) -> R<String> {
    match a.get(i) {
        Some(Valeur::Texte(s)) => Ok(s.to_string()),
        v => Err(fad(format!(
            "{} : l'argument {} doit être un texte (reçu : {}).",
            nom,
            pos_txt(Some(i + 1)),
            v.map_or("rien", type_de)
        ))),
    }
}
fn liste_req(a: &[Valeur], i: usize, nom: &str) -> R<Vec<Valeur>> {
    match a.get(i) {
        Some(Valeur::Liste(l)) => Ok(l.borrow().clone()),
        v => Err(fad(format!(
            "{} : l'argument {} doit être une liste (reçu : {}).",
            nom,
            i + 1,
            v.map_or("rien", type_de)
        ))),
    }
}
fn liste_ref(a: &[Valeur], i: usize, nom: &str) -> R<std::rc::Rc<std::cell::RefCell<Vec<Valeur>>>> {
    match a.get(i) {
        Some(Valeur::Liste(l)) => Ok(l.clone()),
        v => Err(fad(format!(
            "{} : l'argument {} doit être une liste (reçu : {}).",
            nom,
            i + 1,
            v.map_or("rien", type_de)
        ))),
    }
}
fn struct_req(a: &[Valeur], i: usize, nom: &str) -> R<std::rc::Rc<std::cell::RefCell<Dict>>> {
    match a.get(i) {
        Some(Valeur::Struct(d)) => Ok(d.clone()),
        v => Err(fad(format!(
            "{} : l'argument {} doit être une structure (reçu : {}).",
            nom,
            i + 1,
            v.map_or("rien", type_de)
        ))),
    }
}
fn entier_req(a: &[Valeur], i: usize, nom: &str) -> R<f64> {
    match a.get(i) {
        Some(Valeur::Nombre(x)) if est_entier(*x) => Ok(*x),
        v => Err(fad(format!(
            "{} : l'argument {} doit être un nombre entier (reçu : {}).",
            nom,
            i + 1,
            v.map_or("undefined".to_string(), formater)
        ))),
    }
}
fn fonction_req(a: &[Valeur], i: usize, nom: &str) -> R<Valeur> {
    match a.get(i) {
        Some(f @ Valeur::Fonction(_)) => Ok(f.clone()),
        v => Err(fad(format!(
            "{} : l'argument {} doit être une fonction (reçu : {}). Exemple : (x) => x * 2",
            nom,
            i + 1,
            v.map_or("rien", type_de)
        ))),
    }
}

// ------------------------------------------------------------------ texte
fn majuscule(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(t(texte_req(a, 0, "majuscule")?.to_uppercase()))
}
fn minuscule(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(t(texte_req(a, 0, "minuscule")?.to_lowercase()))
}
fn nettoyer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(t(js_trim(&texte_req(a, 0, "nettoyer")?)))
}
fn decouper(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = texte_req(a, 0, "decouper")?;
    let sep = if a.len() > 1 { texte_req(a, 1, "decouper")? } else { " ".to_string() };
    let parts: Vec<Valeur> = if sep.is_empty() { s.chars().map(|c| t(c.to_string())).collect() } else { s.split(sep.as_str()).map(t).collect() };
    Ok(Valeur::liste(parts))
}
fn joindre(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let l = liste_req(a, 0, "joindre")?;
    let sep = if a.len() > 1 { texte_req(a, 1, "joindre")? } else { String::new() };
    Ok(t(l.iter().map(formater).collect::<Vec<_>>().join(&sep)))
}
fn remplacer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = texte_req(a, 0, "remplacer")?;
    let x = texte_req(a, 1, "remplacer")?;
    let r = a.get(2).map_or("undefined".to_string(), formater);
    if x.is_empty() {
        return Ok(t(s.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(&r)));
    }
    Ok(t(s.split(x.as_str()).collect::<Vec<_>>().join(&r)))
}
fn contient(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let y = a.get(1).cloned().unwrap_or(Valeur::Rien);
    match a.get(0) {
        Some(Valeur::Texte(s)) => {
            let p = texte_req(a, 1, "contient")?;
            Ok(Valeur::Bool(s.contains(p.as_str())))
        }
        Some(Valeur::Liste(l)) => Ok(Valeur::Bool(l.borrow().iter().any(|e| egal(e, &y)))),
        Some(Valeur::Struct(d)) => Ok(Valeur::Bool(d.borrow().contient(&js_texte(&y)))),
        v => Err(fad(format!("contient : attend un texte, une liste ou une structure (reçu : {}).", v.map_or("rien", type_de)))),
    }
}
fn commence_par(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(Valeur::Bool(texte_req(a, 0, "commence_par")?.starts_with(&texte_req(a, 1, "commence_par")?)))
}
fn termine_par(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(Valeur::Bool(texte_req(a, 0, "termine_par")?.ends_with(&texte_req(a, 1, "termine_par")?)))
}
fn sous_texte(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s: Vec<char> = texte_req(a, 0, "sous_texte")?.chars().collect();
    let d = entier_req(a, 1, "sous_texte")?;
    let f = if a.len() > 2 { Some(entier_req(a, 2, "sous_texte")?) } else { None };
    let (i, j) = js_slice(s.len(), d, f);
    Ok(t(s[i..j].iter().collect::<String>()))
}
fn position(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    match a.get(0) {
        Some(Valeur::Texte(s)) => {
            let p = texte_req(a, 1, "position")?;
            Ok(n(match s.find(p.as_str()) {
                Some(b) => s[..b].chars().count() as f64,
                None => -1.0,
            }))
        }
        Some(Valeur::Liste(l)) => {
            let y = a.get(1).cloned().unwrap_or(Valeur::Rien);
            Ok(n(l.borrow().iter().position(|e| egal(e, &y)).map_or(-1.0, |i| i as f64)))
        }
        v => Err(fad(format!("position : attend un texte ou une liste (reçu : {}).", v.map_or("rien", type_de)))),
    }
}
fn repeter(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = texte_req(a, 0, "repeter")?;
    let k = entier_req(a, 1, "repeter")?.max(0.0);
    Ok(t(s.repeat(k as usize)))
}
fn inverser(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    match a.get(0) {
        Some(Valeur::Texte(s)) => Ok(t(s.chars().rev().collect::<String>())),
        Some(Valeur::Liste(l)) => Ok(Valeur::liste(l.borrow().iter().rev().cloned().collect())),
        v => Err(fad(format!("inverser : attend un texte ou une liste (reçu : {}).", v.map_or("rien", type_de)))),
    }
}
fn caractere(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s: Vec<char> = texte_req(a, 0, "caractere")?.chars().collect();
    let i = entier_req(a, 1, "caractere")?;
    if i < 0.0 || i >= s.len() as f64 {
        return Err(fad(format!("caractere : index {} hors limites (0 à {}).", js_num(i), s.len() as i64 - 1)));
    }
    Ok(t(s[i as usize].to_string()))
}
fn code_caractere(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = texte_req(a, 0, "code_caractere")?;
    Ok(s.chars().next().map_or(Valeur::Rien, |c| n(c as u32 as f64)))
}
fn depuis_code(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let k = entier_req(a, 0, "depuis_code")?;
    match if k >= 0.0 && k <= 0x10FFFF as f64 { char::from_u32(k as u32) } else { None } {
        Some(c) => Ok(t(c.to_string())),
        None => Err(nat(format!("Invalid code point {}", js_num(k)))),
    }
}
fn est_nombre(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(Valeur::Bool(match a.get(0) {
        Some(Valeur::Nombre(_)) => true,
        Some(Valeur::Texte(s)) => !js_trim(s).is_empty() && !js_nombre_texte(s).is_nan(),
        _ => false,
    }))
}
fn type_de_f(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(t(a.get(0).map_or("rien", type_de)))
}

// ------------------------------------------------------------------ listes
fn plage(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let p = a.get(2).cloned().unwrap_or(Valeur::Nombre(1.0));
    match (a.get(0), a.get(1), &p) {
        (Some(Valeur::Nombre(d)), Some(Valeur::Nombre(f)), Valeur::Nombre(p)) if *p != 0.0 => {
            let mut r = Vec::new();
            let mut i = *d;
            while if *p > 0.0 { i <= *f } else { i >= *f } {
                r.push(n(i));
                if r.len() > 5_000_000 {
                    return Err(fad("plage : liste trop grande."));
                }
                i += *p;
            }
            Ok(Valeur::liste(r))
        }
        _ => Err(fad("plage(début, fin, pas) attend des nombres et un pas non nul.")),
    }
}
fn tranche(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let l = liste_req(a, 0, "tranche")?;
    let d = entier_req(a, 1, "tranche")?;
    let f = if a.len() > 2 { Some(entier_req(a, 2, "tranche")?) } else { None };
    let (i, j) = js_slice(l.len(), d, f);
    Ok(Valeur::liste(l[i..j].to_vec()))
}
fn unique(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    use std::collections::HashSet;
    // Valeurs simples (nombre, texte, booléen, rien) : table de hachage (O(n)). Listes, structures et autres :
    // comparaison deux à deux, uniquement entre elles (une valeur simple n'est jamais égale à une valeur composée).
    #[derive(Hash, PartialEq, Eq)]
    enum Cle {
        Nombre(u64),
        Texte(std::rc::Rc<String>),
        Bool(bool),
        Rien,
    }
    let mut r: Vec<Valeur> = Vec::new();
    let mut vus: HashSet<Cle> = HashSet::new();
    let mut composes: Vec<usize> = Vec::new();
    for x in liste_req(a, 0, "unique")? {
        let cle = match &x {
            // NaN n'est égal à rien (même pas à lui-même) : toujours conservé
            Valeur::Nombre(n) if n.is_nan() => {
                r.push(x);
                continue;
            }
            // 0 et -0 sont égaux
            Valeur::Nombre(n) => Some(Cle::Nombre(if *n == 0.0 { 0 } else { n.to_bits() })),
            Valeur::Texte(t) => Some(Cle::Texte(t.clone())),
            Valeur::Bool(b) => Some(Cle::Bool(*b)),
            Valeur::Rien => Some(Cle::Rien),
            _ => None,
        };
        match cle {
            Some(c) => {
                if vus.insert(c) {
                    r.push(x);
                }
            }
            None => {
                if !composes.iter().any(|&i| egal(&x, &r[i])) {
                    composes.push(r.len());
                    r.push(x);
                }
            }
        }
    }
    Ok(Valeur::liste(r))
}
fn inserer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let l = liste_ref(a, 0, "inserer")?;
    let i = entier_req(a, 1, "inserer")?;
    let len = l.borrow().len();
    if i < 0.0 || i > len as f64 {
        return Err(fad(format!("inserer : index {} hors limites (0 à {}).", js_num(i), len)));
    }
    l.borrow_mut().insert(i as usize, a.get(2).cloned().unwrap_or(Valeur::Rien));
    Ok(a[0].clone())
}
fn retirer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let l = liste_ref(a, 0, "retirer")?;
    let i = entier_req(a, 1, "retirer")?;
    let len = l.borrow().len();
    if i < 0.0 || i >= len as f64 {
        return Err(fad(format!("retirer : index {} hors limites.", js_num(i))));
    }
    let r = l.borrow_mut().remove(i as usize);
    Ok(r)
}
fn concatener(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let mut x = liste_req(a, 0, "concatener")?;
    x.extend(liste_req(a, 1, "concatener")?);
    Ok(Valeur::liste(x))
}
fn aplatir_rec(v: &[Valeur], out: &mut Vec<Valeur>) {
    for x in v {
        match x {
            Valeur::Liste(l) => aplatir_rec(&l.borrow().clone(), out),
            _ => out.push(x.clone()),
        }
    }
}
fn aplatir(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let l = liste_req(a, 0, "aplatir")?;
    let mut out = Vec::new();
    aplatir_rec(&l, &mut out);
    Ok(Valeur::liste(out))
}
fn compter(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let l = liste_req(a, 0, "compter")?;
    let v = a.get(1).cloned().unwrap_or(Valeur::Rien);
    Ok(n(l.iter().filter(|x| egal(x, &v)).count() as f64))
}

// ------------------------------------------------------------------ structures
fn cles(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let d = struct_req(a, 0, "cles")?;
    let r = d.borrow().cles_ordonnees().into_iter().map(t).collect();
    Ok(Valeur::liste(r))
}
fn valeurs(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let d = struct_req(a, 0, "valeurs")?;
    let d = d.borrow();
    Ok(Valeur::liste(d.cles_ordonnees().iter().map(|k| d.get(k).unwrap().clone()).collect()))
}
fn paires(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let d = struct_req(a, 0, "paires")?;
    let d = d.borrow();
    Ok(Valeur::liste(d.cles_ordonnees().iter().map(|k| Valeur::liste(vec![t(k), d.get(k).unwrap().clone()])).collect()))
}
fn a_cle(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let d = struct_req(a, 0, "a_cle")?;
    let k = a.get(1).map_or("undefined".to_string(), js_texte);
    let r = d.borrow().contient(&k);
    Ok(Valeur::Bool(r))
}
fn supprimer_cle(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let d = struct_req(a, 0, "supprimer_cle")?;
    let k = a.get(1).map_or("undefined".to_string(), js_texte);
    let r = d.borrow_mut().supprimer(&k);
    Ok(r.unwrap_or(Valeur::Rien))
}
fn fusionner(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = struct_req(a, 0, "fusionner")?;
    let y = struct_req(a, 1, "fusionner")?;
    let mut o = Dict::nouveau();
    for s in [x, y] {
        let s = s.borrow();
        for k in s.cles_ordonnees() {
            o.set(&k, s.get(&k).unwrap().clone());
        }
    }
    Ok(Valeur::structure(o))
}
fn structure_depuis_paires(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let l = liste_req(a, 0, "structure_depuis_paires")?;
    let mut o = Dict::nouveau();
    for p in l {
        match &p {
            Valeur::Liste(pl) if pl.borrow().len() == 2 => {
                let pl = pl.borrow();
                o.set(&js_texte(&pl[0]), pl[1].clone());
            }
            _ => return Err(fad("structure_depuis_paires : chaque élément doit être une paire [clé, valeur].")),
        }
    }
    Ok(Valeur::structure(o))
}

// ------------------------------------------------------------------ ordre supérieur
fn filtrer(it: &mut Interp, a: &[Valeur], l: usize) -> R<Valeur> {
    let v = liste_req(a, 0, "filtrer")?;
    let f = fonction_req(a, 1, "filtrer")?;
    let mut r = Vec::new();
    for x in v {
        if it.appeler_valeur(&f, [x.clone()], l)?.vrai() {
            r.push(x);
        }
    }
    Ok(Valeur::liste(r))
}
fn transformer(it: &mut Interp, a: &[Valeur], l: usize) -> R<Valeur> {
    let v = liste_req(a, 0, "transformer")?;
    let f = fonction_req(a, 1, "transformer")?;
    let mut r = Vec::new();
    for x in v {
        r.push(it.appeler_valeur(&f, [x], l)?);
    }
    Ok(Valeur::liste(r))
}
fn reduire(it: &mut Interp, a: &[Valeur], l: usize) -> R<Valeur> {
    let v = liste_req(a, 0, "reduire")?;
    let f = fonction_req(a, 1, "reduire")?;
    let (mut acc, debut) = match a.get(2) {
        Some(i) => (i.clone(), 0),
        None => {
            if v.is_empty() {
                return Err(fad("reduire : liste vide sans valeur initiale."));
            }
            (v[0].clone(), 1)
        }
    };
    for x in &v[debut..] {
        acc = it.appeler_valeur(&f, [acc, x.clone()], l)?;
    }
    Ok(acc)
}
fn trouver_f(it: &mut Interp, a: &[Valeur], l: usize) -> R<Valeur> {
    let v = liste_req(a, 0, "trouver")?;
    let f = fonction_req(a, 1, "trouver")?;
    for x in v {
        if it.appeler_valeur(&f, [x.clone()], l)?.vrai() {
            return Ok(x);
        }
    }
    Ok(Valeur::Rien)
}
fn tous(it: &mut Interp, a: &[Valeur], l: usize) -> R<Valeur> {
    let v = liste_req(a, 0, "tous")?;
    let f = fonction_req(a, 1, "tous")?;
    for x in v {
        if !it.appeler_valeur(&f, [x], l)?.vrai() {
            return Ok(Valeur::Bool(false));
        }
    }
    Ok(Valeur::Bool(true))
}
fn existe_f(it: &mut Interp, a: &[Valeur], l: usize) -> R<Valeur> {
    let v = liste_req(a, 0, "existe")?;
    let f = fonction_req(a, 1, "existe")?;
    for x in v {
        if it.appeler_valeur(&f, [x], l)?.vrai() {
            return Ok(Valeur::Bool(true));
        }
    }
    Ok(Valeur::Bool(false))
}
fn trier_par(it: &mut Interp, a: &[Valeur], l: usize) -> R<Valeur> {
    let v = liste_req(a, 0, "trier_par")?;
    let f = fonction_req(a, 1, "trier_par")?;
    let mut paires = Vec::new();
    for x in v {
        let k = it.appeler_valeur(&f, [x.clone()], l)?;
        paires.push((x, k));
    }
    let r = tri_fusion(paires, &mut |p, q| Ok(cmp_defaut(&p.1, &q.1)))?;
    Ok(Valeur::liste(r.into_iter().map(|p| p.0).collect()))
}
fn trier(it: &mut Interp, a: &[Valeur], l: usize) -> R<Valeur> {
    let v = liste_req(a, 0, "trier")?;
    if a.len() < 2 {
        let r = tri_fusion_defaut(v);
        return Ok(Valeur::liste(r));
    }
    let f = fonction_req(a, 1, "trier")?;
    let r = tri_fusion(v, &mut |x, y| {
        match it.appeler_valeur(&f, [x.clone(), y.clone()], l)? {
            Valeur::Nombre(z) => Ok(z),
            _ => Err(fad("trier : la fonction de comparaison doit renvoyer un nombre (négatif, 0 ou positif).")),
        }
    })?;
    Ok(Valeur::liste(r))
}
fn tri_fusion_defaut(v: Vec<Valeur>) -> Vec<Valeur> {
    tri_fusion(v, &mut |x, y| Ok(cmp_defaut(x, y))).unwrap_or_default()
}
fn appliquer(it: &mut Interp, a: &[Valeur], l: usize) -> R<Valeur> {
    let f = fonction_req(a, 0, "appliquer")?;
    let args = liste_req(a, 1, "appliquer")?;
    it.appeler_valeur(&f, args, l)
}

// ------------------------------------------------------------------ regex
fn traduire_motif(m: &str) -> String {
    let c: Vec<char> = m.chars().collect();
    let mut o = String::new();
    let mut i = 0;
    let mut classe = false;
    while i < c.len() {
        let ch = c[i];
        if ch == '\\' && i + 1 < c.len() {
            let s = c[i + 1];
            match s {
                'd' => o.push_str("[0-9]"),
                'D' => o.push_str("[^0-9]"),
                'w' => o.push_str("[A-Za-z0-9_]"),
                'W' => o.push_str("[^A-Za-z0-9_]"),
                'b' if !classe => o.push_str("(?-u:\\b)"),
                'B' if !classe => o.push_str("(?-u:\\B)"),
                '/' => o.push('/'),
                _ => {
                    o.push('\\');
                    o.push(s);
                }
            }
            i += 2;
            continue;
        }
        if ch == '[' {
            classe = true;
        } else if ch == ']' {
            classe = false;
        }
        o.push(ch);
        i += 1;
    }
    o
}
fn construire_regex(a: &[Valeur], nom: &str) -> R<Regex> {
    let motif = texte_req(a, 1, nom)?;
    let dr: String = match a.get(2) {
        Some(x) if x.vrai() => js_texte(x),
        _ => String::new(),
    };
    let mut b = RegexBuilder::new(&traduire_motif(&motif));
    b.case_insensitive(dr.contains('i')).multi_line(dr.contains('m')).dot_matches_new_line(dr.contains('s'));
    b.build().map_err(|e| {
        let msg = e.to_string();
        let court = msg.lines().filter_map(|l| l.strip_prefix("error: ")).last().unwrap_or(&msg).to_string();
        fad(format!("{} : expression régulière invalide ({}).", nom, court))
    })
}
fn regex_test(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let re = construire_regex(a, "regex_test")?;
    Ok(Valeur::Bool(re.is_match(&texte_req(a, 0, "regex_test")?)))
}
fn regex_trouver(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = texte_req(a, 0, "regex_trouver")?;
    let re = construire_regex(a, "regex_trouver")?;
    Ok(Valeur::liste(re.find_iter(&s).map(|m| t(m.as_str())).collect()))
}
fn regex_remplacer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = texte_req(a, 0, "regex_remplacer")?;
    let mut b: Vec<Valeur> = a.to_vec();
    // décalage : (t, motif, rempl, dr) → motif en 1, drapeaux en 2 pour construire_regex
    let rempl = a.get(2).map_or("undefined".to_string(), formater);
    b[2] = a.get(3).cloned().unwrap_or(Valeur::Rien);
    b.truncate(3);
    let re = construire_regex(&b, "regex_remplacer")?;
    let mut out = String::new();
    let mut dernier = 0;
    for caps in re.captures_iter(&s) {
        let m = caps.get(0).unwrap();
        out.push_str(&s[dernier..m.start()]);
        let rc: Vec<char> = rempl.chars().collect();
        let mut i = 0;
        while i < rc.len() {
            if rc[i] == '$' && i + 1 < rc.len() {
                let x = rc[i + 1];
                if x == '$' {
                    out.push('$');
                    i += 2;
                    continue;
                } else if x == '&' {
                    out.push_str(m.as_str());
                    i += 2;
                    continue;
                } else if x == '`' {
                    out.push_str(&s[..m.start()]);
                    i += 2;
                    continue;
                } else if x == '\'' {
                    out.push_str(&s[m.end()..]);
                    i += 2;
                    continue;
                } else if x.is_ascii_digit() {
                    let mut num_ = x.to_digit(10).unwrap() as usize;
                    let mut len = 2;
                    if i + 2 < rc.len() && rc[i + 2].is_ascii_digit() {
                        let n2 = num_ * 10 + rc[i + 2].to_digit(10).unwrap() as usize;
                        if n2 >= 1 && n2 < caps.len() {
                            num_ = n2;
                            len = 3;
                        }
                    }
                    if num_ >= 1 && num_ < caps.len() {
                        out.push_str(caps.get(num_).map_or("", |g| g.as_str()));
                        i += len;
                        continue;
                    }
                } else if x == '<' {
                    if let Some(fin) = rc[i + 2..].iter().position(|c| *c == '>') {
                        let nom: String = rc[i + 2..i + 2 + fin].iter().collect();
                        if let Some(g) = caps.name(&nom) {
                            out.push_str(g.as_str());
                        }
                        i += 3 + fin;
                        continue;
                    }
                }
            }
            out.push(rc[i]);
            i += 1;
        }
        dernier = m.end();
    }
    out.push_str(&s[dernier..]);
    Ok(t(out))
}

/// Groupes de la première correspondance : [texte trouvé, groupe 1, …] ; liste vide si rien ne correspond.
fn regex_groupes(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let re = construire_regex(a, "regex_groupes")?;
    let s = texte_req(a, 0, "regex_groupes")?;
    Ok(match re.captures(&s) {
        Some(c) => Valeur::liste((0..c.len()).map(|i| t(c.get(i).map_or("", |m| m.as_str()))).collect()),
        None => Valeur::liste(vec![]),
    })
}
/// Découpe autour des correspondances (les correspondances vides sont ignorées).
fn regex_decouper(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = texte_req(a, 0, "regex_decouper")?;
    let re = construire_regex(a, "regex_decouper")?;
    let mut r = Vec::new();
    let mut d = 0;
    for m in re.find_iter(&s) {
        if m.as_str().is_empty() {
            continue;
        }
        r.push(t(&s[d..m.start()]));
        d = m.end();
    }
    r.push(t(&s[d..]));
    Ok(Valeur::liste(r))
}
fn regex_echapper(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = texte_req(a, 0, "regex_echapper")?;
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        if ".*+?^${}()|[]\\".contains(c) {
            o.push('\\');
        }
        o.push(c);
    }
    Ok(t(o))
}

// ------------------------------------------------------------------ JSON
fn json_valide(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(Valeur::Bool(match a.get(0) {
        Some(Valeur::Texte(s)) => json_lire(s).is_ok(),
        _ => false,
    }))
}
enum Seg {
    I(f64),
    K(String),
}
/// Découpe « a.b[0].c » en segments (même découpage que l'expression /\[(\d+)\]|[^.[\]]+/g du moteur JavaScript).
fn segments_chemin(ch: &str) -> Vec<Seg> {
    let c: Vec<char> = ch.chars().collect();
    let mut segs = Vec::new();
    let mut i = 0;
    while i < c.len() {
        if c[i] == '[' {
            let mut j = i + 1;
            while j < c.len() && c[j].is_ascii_digit() {
                j += 1;
            }
            if j > i + 1 && j < c.len() && c[j] == ']' {
                let nb: String = c[i + 1..j].iter().collect();
                segs.push(Seg::I(nb.parse::<f64>().unwrap_or(f64::INFINITY)));
                i = j + 1;
            } else {
                i += 1;
            }
        } else if c[i] == '.' || c[i] == ']' {
            i += 1;
        } else {
            let mut j = i;
            while j < c.len() && !".[]".contains(c[j]) {
                j += 1;
            }
            segs.push(Seg::K(c[i..j].iter().collect()));
            i = j;
        }
    }
    segs
}
fn json_chemin(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ch = texte_req(a, 1, "json_chemin")?;
    let mut cur = a.get(0).cloned().unwrap_or(Valeur::Rien);
    let mut fait = String::new();
    for p in segments_chemin(&ch) {
        let mut trouve = false;
        let courant = cur.clone();
        match &courant {
            Valeur::Liste(l) => {
                let idx = match &p {
                    Seg::I(i) => Some(*i),
                    Seg::K(k) if !k.is_empty() && k.chars().all(|c| c.is_ascii_digit()) => k.parse::<f64>().ok(),
                    _ => None,
                };
                if let Some(i) = idx {
                    let l = l.borrow();
                    if i < l.len() as f64 {
                        cur = l[i as usize].clone();
                        trouve = true;
                    }
                }
            }
            Valeur::Struct(d) => {
                if let Seg::K(k) = &p {
                    if let Some(v) = d.borrow().get(k) {
                        cur = v.clone();
                        trouve = true;
                    }
                }
            }
            _ => {}
        }
        let nom = match &p {
            Seg::I(i) => format!("[{}]", js_num(*i)),
            Seg::K(k) => k.clone(),
        };
        if !trouve {
            return match a.get(2) {
                Some(d) if !matches!(d, Valeur::Rien) => Ok(d.clone()),
                _ => Err(fad(format!(
                    "json_chemin : chemin « {} » introuvable (rien à « {} »{}).",
                    ch,
                    nom,
                    if fait.is_empty() { String::new() } else { format!(" après « {} »", fait) }
                ))),
            };
        }
        match &p {
            Seg::I(_) => fait.push_str(&nom),
            Seg::K(k) => {
                if !fait.is_empty() {
                    fait.push('.');
                }
                fait.push_str(k);
            }
        }
    }
    Ok(cur)
}
fn fusion_profonde(x: &Dict, y: &Dict) -> Dict {
    let mut o = Dict::nouveau();
    for k in x.cles_ordonnees() {
        o.set(&k, x.get(&k).unwrap().clone());
    }
    for k in y.cles_ordonnees() {
        let v = y.get(&k).unwrap();
        let nv = match (x.get(&k), v) {
            (Some(Valeur::Struct(xs)), Valeur::Struct(ys)) => Valeur::structure(fusion_profonde(&xs.borrow(), &ys.borrow())),
            _ => v.clone(),
        };
        o.set(&k, nv);
    }
    o
}
fn json_fusionner(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    match (a.get(0), a.get(1)) {
        (Some(Valeur::Struct(x)), Some(Valeur::Struct(y))) => Ok(Valeur::structure(fusion_profonde(&x.borrow(), &y.borrow()))),
        _ => Err(fad("json_fusionner : deux structures sont attendues.")),
    }
}
fn json_texte_f(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ind = match a.get(1) {
        Some(Valeur::Nombre(x)) => " ".repeat(x.max(0.0).min(10.0) as usize),
        Some(Valeur::Texte(s)) => s.chars().take(10).collect(),
        _ => String::new(),
    };
    Ok(t(json_texte(a.get(0).unwrap_or(&Valeur::Rien), &ind)?))
}
fn json_lire_f(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = texte_req(a, 0, "json_lire")?;
    json_lire(&s).map_err(|e| fad(format!("json_lire : JSON invalide ({}).", e)))
}

// ------------------------------------------------------------------ dates (UTC)
fn maintenant(it: &mut Interp, _: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(it.hote.maintenant_ms().floor()))
}
fn ts_arg(it: &Interp, a: &[Valeur]) -> f64 {
    match a.get(0) {
        None => it.hote.maintenant_ms().floor(),
        Some(v) => js_nombre(v),
    }
}
fn date_iso(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    match date_depuis_ms(ts_arg(it, a)) {
        Some(d) => Ok(t(iso(&d))),
        None => Err(nat("Invalid time value")),
    }
}
fn formater_date(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    match date_depuis_ms(ts_arg(it, a)) {
        Some(d) => Ok(t(format!("{:02}/{:02}/{:04} {:02}:{:02}:{:02}", d.jour, d.mois, d.an, d.h, d.mi, d.s))),
        None => Ok(t("Invalid Date")),
    }
}

// ------------------------------------------------------------------ maths
fn num0(a: &[Valeur], i: usize) -> f64 {
    a.get(i).map_or(f64::NAN, js_nombre)
}
fn plancher(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(num0(a, 0).floor()))
}
fn plafond(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(num0(a, 0).ceil()))
}
fn entier(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(num0(a, 0).trunc()))
}
fn minimum(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let f = |x: f64, y: f64| if x.is_nan() || y.is_nan() { f64::NAN } else { x.min(y) };
    if let Some(v) = vec_de(arg(a, 0)) {
        return Ok(n(v.iter().map(js_nombre).fold(f64::INFINITY, f)));
    }
    Ok(n(f(num0(a, 0), num0(a, 1))))
}
fn maximum(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let f = |x: f64, y: f64| if x.is_nan() || y.is_nan() { f64::NAN } else { x.max(y) };
    if let Some(v) = vec_de(arg(a, 0)) {
        return Ok(n(v.iter().map(js_nombre).fold(f64::NEG_INFINITY, f)));
    }
    Ok(n(f(num0(a, 0), num0(a, 1))))
}
#[allow(dead_code)]
fn _inutilise() -> f64 {
    js_pow(1.0, 1.0)
}
