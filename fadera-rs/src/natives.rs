//! Fonctions natives (statistiques, maths, réseau, IA, cryptographie, tests).
//! Règle d'or : toute nouvelle fonction se déclare ICI (ou dans stdlib.rs) et dans `trouver`,
//! jamais comme mot réservé avec une règle de grammaire.

use std::rc::Rc;

use crate::crypto;
use crate::erreur::*;
use crate::interp::{HoteErr, Interp};
use crate::stdlib;
use crate::util::*;
use crate::valeur::*;

pub type NatFn = fn(&mut Interp, &[Valeur], usize) -> R<Valeur>;

pub fn existe(nom: &str) -> bool {
    trouver(nom).is_some()
}

// ----------------------------------------------------------------- aides
pub fn arg(a: &[Valeur], i: usize) -> Option<&Valeur> {
    a.get(i)
}
pub fn est_liste(v: Option<&Valeur>) -> bool {
    matches!(v, Some(Valeur::Liste(_)))
}
pub fn vec_de(v: Option<&Valeur>) -> Option<Vec<Valeur>> {
    match v {
        Some(Valeur::Liste(l)) => Some(l.borrow().clone()),
        _ => None,
    }
}
pub fn nombres_de(v: &[Valeur]) -> Vec<f64> {
    v.iter().map(js_nombre).collect()
}
pub fn valides(v: &[Valeur]) -> Vec<f64> {
    v.iter().map(js_nombre).filter(|x| !x.is_nan()).collect()
}
pub fn num(v: Option<&Valeur>) -> f64 {
    match v {
        None => f64::NAN,
        Some(v) => js_nombre(v),
    }
}
/// Propriété d'une structure (équivalent de `obj.cle`, `undefined` → None).
pub fn prop(v: Option<&Valeur>, k: &str) -> Option<Valeur> {
    match v {
        Some(Valeur::Struct(d)) => d.borrow().get(k).cloned(),
        _ => None,
    }
}
fn vrai_opt(v: &Option<Valeur>) -> bool {
    v.as_ref().map_or(false, |x| x.vrai())
}
fn js_min(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.min(b)
    }
}
fn js_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}
fn n(x: f64) -> Valeur {
    Valeur::Nombre(x)
}
fn t(s: impl AsRef<str>) -> Valeur {
    Valeur::texte(s)
}
fn tri_nombres(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v
}
fn texte_arg(a: &[Valeur], i: usize) -> String {
    a.get(i).map(js_texte).unwrap_or_else(|| "undefined".into())
}

// ----------------------------------------------------------------- registre
pub fn trouver(nom: &str) -> Option<NatFn> {
    Some(match nom {
        "stats_somme" | "somme" => stats_somme,
        "stats_moyenne" | "moyenne" => stats_moyenne,
        "stats_min" => stats_min,
        "stats_max" => stats_max,
        "stats_mediane" => stats_mediane,
        "stats_variance" => stats_variance,
        "stats_ecart_type" => stats_ecart_type,
        "stats_mode" => stats_mode,
        "crypto_hacher_sha256" | "sha256" | "hacher_sha256" => crypto_sha256,
        "math_pgcd_ppcm" => math_pgcd_ppcm,
        "math_facteurs_premiers" => math_facteurs_premiers,
        "math_resoudre_second_degre" => math_second_degre,
        "telecom_calculer_sous_reseau" => telecom_sous_reseau,
        "nombre" => f_nombre,
        "texte" => f_texte,
        "arrondir" => f_arrondir,
        "racine" => f_racine,
        "absolu" => f_absolu,
        "puissance" => f_puissance,
        "crypto_chiffrer_symetrique" | "chiffrer" => c_chiffrer_sym,
        "crypto_dechiffrer_symetrique" | "dechiffrer" => c_dechiffrer_sym,
        "crypto_generer_cles_rsa" => c_cles_rsa,
        "crypto_chiffrer_asymetrique" => c_chiffrer_asym,
        "crypto_dechiffrer_asymetrique" => c_dechiffrer_asym,
        "crypto_hacher_sha512" => c_sha512,
        "crypto_hmac_sha256" => c_hmac,
        "crypto_aleatoire_hex" => c_alea_hex,
        "matrice_carree" => matrice_carree,
        "matrice_triangulaire" => matrice_triangulaire,
        "inverser_matrice" => inverser_matrice,
        "stats_analyse_distribution" => stats_distribution,
        "stats_pearson_correlation" => stats_pearson,
        "stats_regression_lineaire" => stats_regression,
        "stats_supprimer_doublons" => stats_doublons,
        "stats_nettoyer_valeurs_manquantes" => stats_nettoyer,
        "stats_normalisation_ia" => stats_normalisation,
        "stats_query_engine" => stats_query,
        "math_integrale_trapeze" => math_trapeze,
        "math_derivee_centree" => math_derivee,
        "math_resoudre_systeme_gauss" => math_gauss,
        "telecom_creer_topologie" => telecom_creer,
        "telecom_ajouter_liaison" => telecom_liaison,
        "telecom_matrice_adjacence" => telecom_adjacence,
        "telecom_verifier_connexite" => telecom_connexite,
        "telecom_dijkstra_routage" => telecom_dijkstra,
        "ia_creer_neurone" => ia_neurone,
        "ia_activation_perceptron" => ia_activation,
        "ia_descente_gradient" => ia_gradient,
        "ia_entrainer_reseau_xor" => ia_xor,
        "ia_reseau_multicouche" => ia_multicouche,
        "masqueReseau" | "masque_reseau" => masque_reseau,
        "plageReseau" | "plage_reseau" => plage_reseau,
        "creerReseau" | "creer_reseau" => creer_reseau,
        "iaPredire" | "ia_predire" => ia_predire,
        "iaEntrainer" | "ia_entrainer" => ia_entrainer,
        "chiffrer_base64" => chiffrer_b64,
        "dechiffrer_base64" => dechiffrer_b64,
        "creer_fichier" => creer_fichier,
        "ecrire_log" => ecrire_log,
        "verifier" => verifier,
        "verifier_egal" => verifier_egal,
        "verifier_erreur" => verifier_erreur,
        "lever" => lever,
        "__instance" => nat_instance,
        "classe_de" => nat_classe_de,
        "est_instance" => nat_est_instance,
        _ => return stdlib::trouver(nom),
    })
}

// ----------------------------------------------------------------- statistiques
fn stats_somme(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(match vec_de(arg(a, 0)) {
        None => 0.0,
        Some(v) => valides(&v).iter().sum(),
    }))
}
fn stats_moyenne(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(match vec_de(arg(a, 0)) {
        Some(v) if !v.is_empty() => {
            let x = valides(&v);
            if x.is_empty() {
                0.0
            } else {
                x.iter().sum::<f64>() / x.len() as f64
            }
        }
        _ => 0.0,
    }))
}
fn stats_min(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(match vec_de(arg(a, 0)) {
        Some(v) if !v.is_empty() => nombres_de(&v).into_iter().fold(f64::INFINITY, js_min),
        _ => 0.0,
    }))
}
fn stats_max(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(match vec_de(arg(a, 0)) {
        Some(v) if !v.is_empty() => nombres_de(&v).into_iter().fold(f64::NEG_INFINITY, js_max),
        _ => 0.0,
    }))
}
fn mediane(v: &[Valeur]) -> f64 {
    let trie = tri_nombres(valides(v));
    if trie.is_empty() {
        return 0.0;
    }
    let m = trie.len() / 2;
    if trie.len() % 2 != 0 {
        trie[m]
    } else {
        (trie[m - 1] + trie[m]) / 2.0
    }
}
fn stats_mediane(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(match vec_de(arg(a, 0)) {
        Some(v) if !v.is_empty() => mediane(&v),
        _ => 0.0,
    }))
}
fn variance(v: &[Valeur]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    let x = valides(v);
    if x.is_empty() {
        return 0.0;
    }
    let m = x.iter().sum::<f64>() / x.len() as f64;
    x.iter().fold(0.0, |a, v| a + (v - m).powi(2)) / x.len() as f64
}
fn stats_variance(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(vec_de(arg(a, 0)).map_or(0.0, |v| variance(&v))))
}
fn stats_ecart_type(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(vec_de(arg(a, 0)).map_or(0.0, |v| variance(&v)).sqrt()))
}
fn stats_mode(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let v = match vec_de(arg(a, 0)) {
        Some(v) if !v.is_empty() => v,
        _ => return Ok(Valeur::liste(vec![])),
    };
    let mut occ: Vec<(String, usize)> = Vec::new();
    let mut max = 0;
    for x in &v {
        let k = js_texte(x);
        let c = match occ.iter_mut().find(|e| e.0 == k) {
            Some(e) => {
                e.1 += 1;
                e.1
            }
            None => {
                occ.push((k, 1));
                1
            }
        };
        if c > max {
            max = c;
        }
    }
    let cles: Vec<String> = occ.iter().map(|e| e.0.clone()).collect();
    let ord = ordonner_cles(&cles);
    Ok(Valeur::liste(
        ord.into_iter()
            .filter(|k| occ.iter().any(|e| &e.0 == k && e.1 == max))
            .map(|k| n(js_nombre_texte(&k)))
            .collect(),
    ))
}

// ----------------------------------------------------------------- maths
fn crypto_sha256(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(t(crypto::sha256_hex(&a.get(0).map(formater).unwrap_or_else(|| "undefined".into()))))
}
fn math_pgcd_ppcm(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = a.get(0).map_or(f64::NAN, js_parse_int).abs();
    let y = a.get(1).map_or(f64::NAN, js_parse_int).abs();
    if x.is_nan() || y.is_nan() {
        return Err(nat("Deux entiers valides sont requis."));
    }
    let (mut p, mut q) = (x, y);
    while q != 0.0 {
        let tmp = q;
        q = p % q;
        p = tmp;
    }
    Ok(Valeur::struct_de(vec![("pgcd", n(p)), ("ppcm", n(if p == 0.0 { 0.0 } else { (x * y) / p }))]))
}
fn math_facteurs_premiers(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let mut m = a.get(0).map_or(f64::NAN, js_parse_int);
    if m.is_nan() || m < 2.0 {
        return Err(nat("Un entier supérieur ou égal à 2 est requis."));
    }
    let mut f = Vec::new();
    let mut d = 2.0f64;
    while m >= 2.0 {
        if d * d > m {
            f.push(m);
            break;
        }
        while m % d == 0.0 {
            f.push(d);
            m /= d;
        }
        d += 1.0;
    }
    Ok(Valeur::nombres(&f))
}
fn math_second_degre(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let g = |i: usize| a.get(i).map_or(f64::NAN, js_parse_float);
    let (a, b, c) = (g(0), g(1), g(2));
    if a == 0.0 {
        return Ok(if b != 0.0 { Valeur::nombres(&[-c / b]) } else { t("Équation impossible") });
    }
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return Ok(t("Aucune racine réelle (Delta négatif)"));
    }
    if disc == 0.0 {
        return Ok(Valeur::nombres(&[-b / (2.0 * a)]));
    }
    Ok(Valeur::nombres(&[(-b - disc.sqrt()) / (2.0 * a), (-b + disc.sqrt()) / (2.0 * a)]))
}
fn telecom_sous_reseau(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ip = texte_arg(a, 0);
    let cidr = a.get(1).map_or(f64::NAN, js_parse_int);
    if cidr.is_nan() || cidr < 0.0 || cidr > 32.0 {
        return Err(nat("CIDR invalide (0 à 32)."));
    }
    let cidr = cidr as u32;
    let o: Vec<f64> = ip.split('.').map(js_nombre_texte).collect();
    if o.len() != 4 || o.iter().any(|x| x.is_nan() || *x < 0.0 || *x > 255.0) {
        return Err(nat("Format d'adresse IPv4 incorrect."));
    }
    let bits: String = "1".repeat(cidr as usize) + &"0".repeat(32 - cidr as usize);
    let m: Vec<u32> = (0..4).map(|i| u32::from_str_radix(&bits[i * 8..i * 8 + 8], 2).unwrap()).collect();
    let w: Vec<u32> = m.iter().map(|x| 255 - x).collect();
    let reseau: Vec<u32> = o.iter().enumerate().map(|(i, x)| (*x as u32) & m[i]).collect();
    let diff: Vec<u32> = reseau.iter().enumerate().map(|(i, x)| x | w[i]).collect();
    let hotes = if 32 - cidr >= 2 { 2f64.powi(32 - cidr as i32) - 2.0 } else { 0.0 };
    let mut prem = reseau.clone();
    prem[3] += 1;
    let mut der = diff.clone();
    der[3] = der[3].wrapping_sub(1);
    let j = |v: &[u32]| v.iter().map(|x| (*x as i64).to_string()).collect::<Vec<_>>().join(".");
    let derj = if der[3] > 255 && diff[3] == 0 { // -1 comme en JS
        let mut s: Vec<String> = diff.iter().map(|x| x.to_string()).collect();
        s[3] = "-1".into();
        s.join(".")
    } else {
        j(&der)
    };
    Ok(Valeur::struct_de(vec![
        ("adresse_saisie", t(format!("{}/{}", ip, cidr))),
        ("masque_sous_reseau", t(j(&m))),
        ("masque_inverse_wildcard", t(j(&w))),
        ("adresse_reseau_exacte", t(j(&reseau))),
        ("adresse_diffusion_broadcast", t(j(&diff))),
        ("premiere_ip_disponible", t(if hotes > 0.0 { j(&prem) } else { "N/A".into() })),
        ("derniere_ip_disponible", t(if hotes > 0.0 { derj } else { "N/A".into() })),
        ("nombre_hotes_utilisables", n(hotes)),
    ]))
}
fn f_nombre(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = a.get(0).cloned().unwrap_or(Valeur::Rien);
    if let Valeur::Nombre(_) = x {
        return Ok(x);
    }
    let s = if a.is_empty() { "undefined".to_string() } else { js_texte(&x) };
    let tt = js_trim(&s).replacen(',', ".", 1);
    let v = js_nombre_texte(&tt);
    if tt.is_empty() || v.is_nan() {
        return Err(nat(format!("impossible de convertir \"{}\" en nombre.", formater(&x))));
    }
    Ok(n(v))
}
fn f_texte(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(t(a.get(0).map(formater).unwrap_or_else(|| "undefined".into())))
}
fn f_arrondir(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let d = match a.get(1) {
        None => 0.0,
        Some(v) => js_parse_int(v),
    };
    let f = if d.is_nan() { f64::NAN } else { crate::mathfn::pow(10.0, d) };
    Ok(n(js_round(num(a.get(0)) * f) / f))
}
fn f_racine(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a.get(0));
    if x < 0.0 {
        return Err(nat("racine d'un nombre négatif."));
    }
    Ok(n(x.sqrt()))
}
fn f_absolu(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(num(a.get(0)).abs()))
}
pub fn js_pow(a: f64, b: f64) -> f64 {
    if b.is_nan() || (b.is_infinite() && a.abs() == 1.0) {
        f64::NAN
    } else {
        crate::mathfn::pow(a, b)
    }
}
fn f_puissance(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(js_pow(num(a.get(0)), num(a.get(1)))))
}

// ----------------------------------------------------------------- crypto
fn txt(a: &[Valeur], i: usize) -> String {
    texte_arg(a, i)
}
fn ok_crypto(r: Result<String, String>) -> R<Valeur> {
    match r {
        Ok(s) => Ok(t(s)),
        Err(m) => Err(nat(m)),
    }
}
fn c_chiffrer_sym(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    match a.get(1) {
        None => Err(nat("une clé (mot de passe) non vide est requise.")),
        Some(c) => ok_crypto(crypto::chiffrer_sym(&txt(a, 0), &js_texte(c))),
    }
}
fn c_dechiffrer_sym(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    ok_crypto(crypto::dechiffrer_sym(&txt(a, 0), &txt(a, 1)))
}
fn c_cles_rsa(_: &mut Interp, _: &[Valeur], _: usize) -> R<Valeur> {
    match crypto::generer_cles_rsa() {
        Ok((p, s)) => Ok(Valeur::struct_de(vec![("cle_publique", t(p)), ("cle_privee", t(s))])),
        Err(m) => Err(nat(m)),
    }
}
fn c_chiffrer_asym(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    ok_crypto(crypto::chiffrer_asym(&txt(a, 0), &txt(a, 1)))
}
fn c_dechiffrer_asym(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    ok_crypto(crypto::dechiffrer_asym(&txt(a, 0), &txt(a, 1)))
}
fn c_sha512(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    ok_crypto(crypto::sha512_hex(&txt(a, 0)))
}
fn c_hmac(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    ok_crypto(crypto::hmac_sha256_hex(&txt(a, 0), &txt(a, 1)))
}
fn c_alea_hex(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let k = match a.get(0) {
        None => 16.0,
        Some(v) => js_parse_int(v),
    };
    if k.is_nan() || k < 1.0 || k > 1024.0 {
        return Err(nat("le nombre d'octets doit être entre 1 et 1024."));
    }
    ok_crypto(crypto::aleatoire_hex(k as usize))
}

// ----------------------------------------------------------------- matrices
fn longueur(v: &Valeur) -> Option<usize> {
    match v {
        Valeur::Liste(l) => Some(l.borrow().len()),
        Valeur::Texte(s) => Some(s.chars().count()),
        _ => None,
    }
}
fn matrice_carree(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let m = vec_de(arg(a, 0));
    match m {
        Some(m) if matches!(m.get(0), Some(Valeur::Liste(_))) => {
            Ok(Valeur::Bool(m.iter().all(|l| longueur(l) == Some(m.len()))))
        }
        _ => Err(nat("une matrice (liste de listes) est requise.")),
    }
}
fn matrice_triangulaire(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let m = match vec_de(arg(a, 0)) {
        Some(m) => m,
        None => return Ok(Valeur::Bool(false)),
    };
    let r0 = match m.get(0) {
        Some(Valeur::Liste(l)) => l.borrow().len(),
        _ => return Ok(Valeur::Bool(false)),
    };
    if m.len() != r0 {
        return Ok(Valeur::Bool(false));
    }
    let (mut sup, mut inf) = (true, true);
    for (i, ligne) in m.iter().enumerate() {
        if let Valeur::Liste(l) = ligne {
            for (j, x) in l.borrow().iter().enumerate() {
                let nul = matches!(x, Valeur::Nombre(z) if *z == 0.0);
                if i > j && !nul {
                    sup = false;
                }
                if i < j && !nul {
                    inf = false;
                }
            }
        }
    }
    Ok(Valeur::Bool(sup || inf))
}
fn inverser_matrice(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let err = || nat("l'inversion native ne gère que les matrices carrées 2x2 pour l'instant.");
    let m = vec_de(arg(a, 0)).ok_or_else(err)?;
    if m.len() != 2 {
        return Err(err());
    }
    let l0 = vec_de(m.get(0)).ok_or_else(err)?;
    if l0.len() != 2 {
        return Err(err());
    }
    let l1 = vec_de(m.get(1)).unwrap_or_default();
    let g = |l: &Vec<Valeur>, i: usize| l.get(i).map_or(f64::NAN, js_nombre);
    let (m00, m01, m10, m11) = (g(&l0, 0), g(&l0, 1), g(&l1, 0), g(&l1, 1));
    let det = m00 * m11 - m01 * m10;
    if det == 0.0 {
        return Err(nat("la matrice n'est pas inversible (déterminant nul)."));
    }
    Ok(Valeur::liste(vec![Valeur::nombres(&[m11 / det, -m01 / det]), Valeur::nombres(&[-m10 / det, m00 / det])]))
}

// ----------------------------------------------------------------- statistiques avancées
fn stats_distribution(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let v = match vec_de(arg(a, 0)) {
        Some(v) if !v.is_empty() => v,
        _ => return Err(nat("une liste non vide est requise.")),
    };
    let trie = tri_nombres(valides(&v));
    if trie.is_empty() {
        return Err(nat("aucune valeur numérique dans la liste."));
    }
    let cut = |p: f64| -> f64 {
        let pos = (trie.len() - 1) as f64 * p;
        let base = pos.floor();
        let reste = pos - base;
        let b = base as usize;
        if b + 1 < trie.len() {
            trie[b] + reste * (trie[b + 1] - trie[b])
        } else {
            trie[b]
        }
    };
    let (q1, q3) = (cut(0.25), cut(0.75));
    let iqr = q3 - q1;
    Ok(Valeur::struct_de(vec![
        ("premier_quartile_q1", n(q1)),
        ("troisieme_quartile_q3", n(q3)),
        ("intervalle_interquartile_iqr", n(iqr)),
        ("seuil_anomalie_bas", n(q1 - 1.5 * iqr)),
        ("seuil_anomalie_haut", n(q3 + 1.5 * iqr)),
    ]))
}
fn deux_listes(a: &[Valeur]) -> R<(Vec<Valeur>, Vec<Valeur>)> {
    match (vec_de(arg(a, 0)), vec_de(arg(a, 1))) {
        (Some(x), Some(y)) if x.len() == y.len() && !x.is_empty() => Ok((x, y)),
        _ => Err(nat("deux listes numériques de même taille sont requises.")),
    }
}
fn stats_pearson(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let (x, y) = deux_listes(a)?;
    let nn = x.len() as f64;
    let mx = x.iter().fold(0.0, |s, b| s + js_nombre(b)) / nn;
    let my = y.iter().fold(0.0, |s, b| s + js_nombre(b)) / nn;
    let (mut num_, mut dx_, mut dy_) = (0.0, 0.0, 0.0);
    for i in 0..x.len() {
        let dx = js_nombre(&x[i]) - mx;
        let dy = js_nombre(&y[i]) - my;
        num_ += dx * dy;
        dx_ += dx * dx;
        dy_ += dy * dy;
    }
    if dx_ == 0.0 || dy_ == 0.0 {
        return Ok(n(0.0));
    }
    Ok(n(num_ / (dx_ * dy_).sqrt()))
}
fn stats_regression(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let (x, y) = deux_listes(a)?;
    let nn = x.len() as f64;
    let (mut sx, mut sy, mut sxy, mut sxx) = (0.0, 0.0, 0.0, 0.0);
    for i in 0..x.len() {
        let p = js_nombre(&x[i]);
        let q = js_nombre(&y[i]);
        sx += p;
        sy += q;
        sxy += p * q;
        sxx += p * p;
    }
    let den = nn * sxx - sx * sx;
    if den == 0.0 {
        return Ok(Valeur::struct_de(vec![("formule", t("indéterminée")), ("pente", n(0.0)), ("ordonnee_origine", n(0.0))]));
    }
    let pente = (nn * sxy - sx * sy) / den;
    let ord = (sy - pente * sx) / nn;
    Ok(Valeur::struct_de(vec![
        ("formule", t(format!("y = {} * x + {}", to_fixed(pente, 4), to_fixed(ord, 4)))),
        ("pente", n(pente)),
        ("ordonnee_origine", n(ord)),
    ]))
}
fn stats_doublons(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let v = vec_de(arg(a, 0)).ok_or_else(|| nat("une liste est requise."))?;
    let mut r: Vec<Valeur> = Vec::new();
    for x in v {
        if !r.iter().any(|y| meme_valeur_zero(&x, y)) {
            r.push(x);
        }
    }
    Ok(Valeur::liste(r))
}
fn stats_nettoyer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let v = vec_de(arg(a, 0)).ok_or_else(|| nat("une liste est requise."))?;
    let strat = match a.get(1) {
        Some(s) if s.vrai() => js_texte(s),
        _ => "supprimer".to_string(),
    };
    let manquant = |x: &Valeur| matches!(x, Valeur::Rien) || matches!(x, Valeur::Texte(s) if s.is_empty()) || js_nombre(x).is_nan();
    if strat == "supprimer" {
        return Ok(Valeur::liste(v.into_iter().filter(|x| !manquant(x)).collect()));
    }
    let val = valides(&v);
    let mut rempl = 0.0;
    if strat == "moyenne" {
        rempl = val.iter().sum::<f64>() / val.len() as f64;
    } else if strat == "mediane" {
        let tr = tri_nombres(val.clone());
        let x = tr.get(tr.len() / 2).copied().unwrap_or(0.0);
        rempl = if x == 0.0 || x.is_nan() { 0.0 } else { x };
    }
    Ok(Valeur::liste(v.iter().map(|x| if manquant(x) { n(rempl) } else { n(js_nombre(x)) }).collect()))
}
fn stats_normalisation(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let v = match vec_de(arg(a, 0)) {
        Some(v) if !v.is_empty() => v,
        _ => return Ok(Valeur::liste(vec![])),
    };
    let x = valides(&v);
    let mn = x.iter().copied().fold(f64::INFINITY, js_min);
    let mx = x.iter().copied().fold(f64::NEG_INFINITY, js_max);
    if mx == mn {
        return Ok(Valeur::nombres(&x.iter().map(|_| 0.0).collect::<Vec<_>>()));
    }
    Ok(Valeur::nombres(&x.iter().map(|v| (v - mn) / (mx - mn)).collect::<Vec<_>>()))
}
fn egal_lache(a: &Valeur, b: &Valeur) -> bool {
    use Valeur::*;
    match (a, b) {
        (Rien, Rien) => true,
        (Rien, _) | (_, Rien) => false,
        (Nombre(_), Texte(_)) | (Texte(_), Nombre(_)) | (Bool(_), _) | (_, Bool(_)) => {
            if matches!((a, b), (Texte(_), Texte(_))) {
                egal(a, b)
            } else if matches!(a, Liste(_) | Struct(_)) || matches!(b, Liste(_) | Struct(_)) {
                false
            } else {
                js_nombre(a) == js_nombre(b)
            }
        }
        _ => egal(a, b),
    }
}
fn stats_query(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let data = vec_de(arg(a, 0)).ok_or_else(|| nat("le dataset doit être une liste de structures."))?;
    let opt = match a.get(1) {
        Some(o) if o.vrai() => o,
        _ => return Ok(a[0].clone()),
    };
    let filtres = match prop(Some(opt), "filtres") {
        Some(f) if f.vrai() => vec_de(Some(&f)).unwrap_or_default(),
        _ => vec![],
    };
    let tri = prop(Some(opt), "tri").filter(|x| x.vrai());
    let selection = match prop(Some(opt), "selection") {
        Some(s) if s.vrai() => s,
        _ => Valeur::liste(vec![]),
    };
    let mut res: Vec<Valeur> = Vec::new();
    for item in data {
        let mut garde = true;
        for f in &filtres {
            let champ = prop(Some(f), "champ");
            let v = champ.as_ref().and_then(|c| prop(Some(&item), &js_texte(c)));
            let fv = prop(Some(f), "valeur");
            let op = prop(Some(f), "operateur").map(|o| js_texte(&o)).unwrap_or_default();
            let vn = || v.as_ref().map_or(f64::NAN, js_nombre);
            let fn_ = || fv.as_ref().map_or(f64::NAN, js_nombre);
            let ok = match op.as_str() {
                "==" => egal_lache(v.as_ref().unwrap_or(&Valeur::Rien), fv.as_ref().unwrap_or(&Valeur::Rien)),
                "!=" => !egal_lache(v.as_ref().unwrap_or(&Valeur::Rien), fv.as_ref().unwrap_or(&Valeur::Rien)),
                ">" => vn() > fn_(),
                "<" => vn() < fn_(),
                ">=" => vn() >= fn_(),
                "<=" => vn() <= fn_(),
                "contient" => {
                    let s = v.as_ref().map_or("undefined".to_string(), js_texte).to_lowercase();
                    let p = fv.as_ref().map_or("undefined".to_string(), js_texte).to_lowercase();
                    s.contains(&p)
                }
                _ => true,
            };
            if !ok {
                garde = false;
                break;
            }
        }
        if garde {
            res.push(item);
        }
    }
    if let Some(tri) = tri {
        if let Some(champ) = prop(Some(&tri), "champ").filter(|c| c.vrai()) {
            let ck = js_texte(&champ);
            let ordre = if prop(Some(&tri), "ordre").map_or(false, |o| js_texte(&o) == "desc") { -1.0 } else { 1.0 };
            let cle = |it: &Valeur| prop(Some(it), &ck);
            res = tri_fusion(res, &mut |x, y| {
                let (va, vb) = (cle(x), cle(y));
                let (na, nb) = (va.as_ref().map_or(f64::NAN, js_nombre), vb.as_ref().map_or(f64::NAN, js_nombre));
                let r = if !na.is_nan() && !nb.is_nan() {
                    (na - nb) * ordre
                } else {
                    let (sa, sb) = (va.as_ref().map_or("undefined".into(), js_texte), vb.as_ref().map_or("undefined".into(), js_texte));
                    (match locale_compare(&sa, &sb) {
                        std::cmp::Ordering::Less => -1.0,
                        std::cmp::Ordering::Equal => 0.0,
                        _ => 1.0,
                    }) * ordre
                };
                Ok(if r.is_nan() { 0.0 } else { r })
            })?;
        }
    }
    if let Some(sel) = vec_de(Some(&selection)) {
        if !sel.is_empty() {
            res = res
                .into_iter()
                .map(|item| {
                    let mut d = Dict::nouveau();
                    for c in &sel {
                        let k = js_texte(c);
                        if let Valeur::Struct(s) = &item {
                            if let Some(v) = s.borrow().get(&k) {
                                d.set(&k, v.clone());
                            }
                        }
                    }
                    Valeur::structure(d)
                })
                .collect();
        }
    }
    Ok(Valeur::liste(res))
}

// ----------------------------------------------------------------- analyse numérique
fn math_trapeze(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let v = match vec_de(arg(a, 0)) {
        Some(v) if v.len() >= 2 => v,
        _ => return Ok(n(0.0)),
    };
    let pas = match a.get(1) {
        None => 1.0,
        Some(x) => js_parse_float(x),
    };
    let mut aire = 0.0;
    for i in 0..v.len() - 1 {
        aire += (js_parse_float(&v[i]) + js_parse_float(&v[i + 1])) / 2.0;
    }
    Ok(n(aire * pas))
}
fn math_derivee(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let v = match vec_de(arg(a, 0)) {
        Some(v) if v.len() >= 3 => v,
        _ => return Ok(n(0.0)),
    };
    let idx = a.get(1).map_or(f64::NAN, js_parse_int);
    if idx.is_nan() || idx <= 0.0 || idx >= (v.len() - 1) as f64 {
        return Err(nat("l'index doit être strictement entre le début et la fin de la liste."));
    }
    let i = idx as usize;
    Ok(n((js_parse_float(&v[i + 1]) - js_parse_float(&v[i - 1])) / 2.0))
}
fn math_gauss(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let rows = match vec_de(arg(a, 0)) {
        Some(v) if !v.is_empty() => v,
        _ => return Err(nat("une matrice augmentée valide est requise.")),
    };
    let nn = rows.len();
    let mut m: Vec<Vec<f64>> = rows
        .iter()
        .map(|l| {
            let mut r: Vec<f64> = vec_de(Some(l)).unwrap_or_default().iter().map(js_nombre).collect();
            while r.len() < nn + 1 {
                r.push(f64::NAN);
            }
            r
        })
        .collect();
    for i in 0..nn {
        let mut piv = i;
        for k in i + 1..nn {
            if m[k][i].abs() > m[piv][i].abs() {
                piv = k;
            }
        }
        m.swap(i, piv);
        if m[i][i].abs() < 1e-9 {
            return Err(nat("le système n'a pas de solution unique (déterminant nul)."));
        }
        for k in i + 1..nn {
            let f = m[k][i] / m[i][i];
            for j in i..=nn {
                m[k][j] -= f * m[i][j];
            }
        }
    }
    let mut sol = vec![0.0; nn];
    for i in (0..nn).rev() {
        let mut s = 0.0;
        for j in i + 1..nn {
            s += m[i][j] * sol[j];
        }
        sol[i] = (m[i][nn] - s) / m[i][i];
    }
    Ok(Valeur::nombres(&sol))
}

// ----------------------------------------------------------------- réseau / topologie
fn telecom_creer(_: &mut Interp, _: &[Valeur], _: usize) -> R<Valeur> {
    Ok(Valeur::struct_de(vec![("noeuds", Valeur::structure(Dict::nouveau())), ("listeNoeuds", Valeur::liste(vec![]))]))
}
fn telecom_liaison(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let topo = a.get(0).cloned();
    let liaison = a.get(1).cloned();
    let de = prop(liaison.as_ref(), "de");
    let vers = prop(liaison.as_ref(), "a");
    let topo_ok = topo.as_ref().map_or(false, |x| x.vrai());
    if !topo_ok || !liaison.as_ref().map_or(false, |x| x.vrai()) || !vrai_opt(&de) || !vrai_opt(&vers) {
        return Err(nat("la liaison (structure) doit contenir les clés de, a, et éventuellement cout."));
    }
    let (de, vers) = (de.unwrap(), vers.unwrap());
    let cout = match prop(liaison.as_ref(), "cout") {
        None => 1.0,
        Some(c) => js_parse_float(&c),
    };
    let noeuds = prop(topo.as_ref(), "noeuds");
    let liste = prop(topo.as_ref(), "listeNoeuds");
    let (noeuds, liste) = match (noeuds, liste) {
        (Some(Valeur::Struct(n)), Some(Valeur::Liste(l))) => (n, l),
        _ => return Err(nat("topologie invalide (voir telecom_creer_topologie).")),
    };
    for x in [&de, &vers] {
        let k = js_texte(x);
        let existe = noeuds.borrow().get(&k).map_or(false, |v| v.vrai());
        if !existe {
            noeuds.borrow_mut().set(&k, Valeur::structure(Dict::nouveau()));
            liste.borrow_mut().push(x.clone());
        }
    }
    let (kd, kv) = (js_texte(&de), js_texte(&vers));
    for (p, q) in [(&kd, &kv), (&kv, &kd)] {
        if let Some(Valeur::Struct(d)) = noeuds.borrow().get(p) {
            d.borrow_mut().set(q, n(cout));
        }
    }
    Ok(topo.unwrap())
}
fn topo_parts(a: &[Valeur]) -> Option<(Rc<std::cell::RefCell<Dict>>, Vec<Valeur>)> {
    match (prop(a.get(0), "noeuds"), prop(a.get(0), "listeNoeuds")) {
        (Some(Valeur::Struct(n)), Some(Valeur::Liste(l))) => Some((n, l.borrow().clone())),
        _ => None,
    }
}
fn telecom_adjacence(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let (noeuds, liste) = topo_parts(a).ok_or_else(|| nat("une topologie valide est requise (voir telecom_creer_topologie)."))?;
    let mut ks: Vec<Valeur> = liste;
    ks = {
        let mut v = ks;
        let mut tmp: Vec<(String, Valeur)> = v.drain(..).map(|x| (js_texte(&x), x)).collect();
        tmp.sort_by(|p, q| cmp_utf16(&p.0, &q.0));
        tmp.into_iter().map(|p| p.1).collect()
    };
    let nn = ks.len();
    let mut m = vec![vec![0.0; nn]; nn];
    for i in 0..nn {
        for j in 0..nn {
            let ki = js_texte(&ks[i]);
            let kj = js_texte(&ks[j]);
            let ligne = noeuds.borrow().get(&ki).cloned();
            if let Some(Valeur::Struct(d)) = ligne {
                if let Some(v) = d.borrow().get(&kj) {
                    m[i][j] = js_nombre(v);
                }
            }
        }
    }
    Ok(Valeur::struct_de(vec![
        ("index_noeuds", Valeur::liste(ks)),
        ("matrice", Valeur::liste(m.iter().map(|r| Valeur::nombres(r)).collect())),
    ]))
}
fn voisins(noeuds: &Rc<std::cell::RefCell<Dict>>, k: &str) -> Vec<(String, f64)> {
    match noeuds.borrow().get(k) {
        Some(Valeur::Struct(d)) => {
            let d = d.borrow();
            d.cles_ordonnees().into_iter().map(|c| { let v = js_nombre(d.get(&c).unwrap()); (c, v) }).collect()
        }
        _ => vec![],
    }
}
fn telecom_connexite(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let (noeuds, liste) = match topo_parts(a) {
        Some(x) if !x.1.is_empty() => x,
        _ => return Ok(Valeur::Bool(false)),
    };
    let depart = js_texte(&liste[0]);
    let mut vis = vec![depart.clone()];
    let mut file = std::collections::VecDeque::from(vec![depart]);
    while let Some(cur) = file.pop_front() {
        for (v, _) in voisins(&noeuds, &cur) {
            if !vis.contains(&v) {
                vis.push(v.clone());
                file.push_back(v);
            }
        }
    }
    Ok(Valeur::Bool(vis.len() == liste.len()))
}
fn telecom_dijkstra(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let req = a.get(1).cloned();
    let depart = prop(req.as_ref(), "depart");
    let arrivee = prop(req.as_ref(), "arrivee");
    if a.get(0).map_or(true, |x| !x.vrai()) || !req.as_ref().map_or(false, |x| x.vrai()) || !vrai_opt(&depart) || !vrai_opt(&arrivee) {
        return Err(nat("la requête (structure) doit contenir depart et arrivee."));
    }
    let (depart, arrivee) = (depart.unwrap(), arrivee.unwrap());
    let noeuds = match prop(a.get(0), "noeuds") {
        Some(Valeur::Struct(d)) => d,
        _ => return Err(nat("topologie invalide (voir telecom_creer_topologie).")),
    };
    let ordre = noeuds.borrow().cles_ordonnees();
    let mut dist: Vec<(String, f64)> = ordre.iter().map(|k| (k.clone(), f64::INFINITY)).collect();
    let kd = js_texte(&depart);
    match dist.iter_mut().find(|e| e.0 == kd) {
        None => return Err(nat(format!("le nœud de départ \"{}\" n'existe pas dans la topologie.", js_texte(&depart)))),
        Some(e) => e.1 = 0.0,
    }
    let mut visites: Vec<String> = Vec::new();
    let mut parents: Vec<(String, String)> = Vec::new();
    while visites.len() < ordre.len() {
        let mut actuel: Option<usize> = None;
        for (i, e) in dist.iter().enumerate() {
            if !visites.contains(&e.0) && (actuel.is_none() || e.1 < dist[actuel.unwrap()].1) {
                actuel = Some(i);
            }
        }
        let ai = match actuel {
            Some(i) if dist[i].1 != f64::INFINITY => i,
            _ => break,
        };
        let nom = dist[ai].0.clone();
        let da = dist[ai].1;
        visites.push(nom.clone());
        for (v, c) in voisins(&noeuds, &nom) {
            let alt = da + c;
            if let Some(e) = dist.iter_mut().find(|e| e.0 == v) {
                if alt < e.1 {
                    e.1 = alt;
                    parents.retain(|p| p.0 != v);
                    parents.push((v.clone(), nom.clone()));
                }
            }
        }
    }
    let ka = js_texte(&arrivee);
    let da = dist.iter().find(|e| e.0 == ka).map(|e| e.1);
    let mut chemin: Vec<Valeur> = Vec::new();
    if let Some(d) = da {
        if d != f64::INFINITY {
            let mut etape = Some(arrivee.clone());
            while let Some(e) = etape {
                if !e.vrai() {
                    break;
                }
                let ke = js_texte(&e);
                chemin.insert(0, e);
                etape = parents.iter().find(|p| p.0 == ke).map(|p| t(&p.1));
            }
        }
    }
    Ok(Valeur::struct_de(vec![("cout_total", n(da.unwrap_or(f64::INFINITY))), ("itineraire", Valeur::liste(chemin))]))
}

// ----------------------------------------------------------------- IA
fn ia_neurone(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let k = a.get(0).map_or(f64::NAN, js_parse_int);
    let k = if k.is_nan() || k == 0.0 { 2 } else { k.max(0.0) as usize };
    let poids: Vec<f64> = (0..k).map(|_| it.alea() * 2.0 - 1.0).collect();
    let biais = it.alea() * 2.0 - 1.0;
    let act = match a.get(1) {
        Some(x) if x.vrai() => js_texte(x),
        _ => "heaviside".to_string(),
    };
    Ok(Valeur::struct_de(vec![
        ("poids", Valeur::nombres(&poids)),
        ("biais", n(biais)),
        ("taux_apprentissage", n(0.1)),
        ("activation", t(act)),
        ("sortie", n(0.0)),
        ("gradient", n(0.0)),
    ]))
}
fn set_prop(v: &Valeur, k: &str, x: Valeur) {
    if let Valeur::Struct(d) = v {
        d.borrow_mut().set(k, x);
    }
}
fn ia_activation(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let neurone = a.get(0);
    let poids = prop(neurone, "poids").and_then(|p| vec_de(Some(&p)));
    let entrees = vec_de(a.get(1));
    let err = || nat("le nombre d'entrées doit correspondre au nombre de poids du neurone.");
    let (poids, entrees) = match (neurone.filter(|x| x.vrai()), poids, entrees) {
        (Some(_), Some(p), Some(e)) if e.len() == p.len() => (p, e),
        _ => return Err(err()),
    };
    let mut somme = num(prop(neurone, "biais").as_ref());
    for i in 0..entrees.len() {
        somme += js_nombre(&entrees[i]) * js_nombre(&poids[i]);
    }
    let act = prop(neurone, "activation").map(|x| js_texte(&x)).unwrap_or_default();
    let sortie = match act.as_str() {
        "sigmoid" => 1.0 / (1.0 + crate::mathfn::exp(-somme)),
        "relu" => js_max(0.0, somme),
        _ => {
            if somme > 0.0 {
                1.0
            } else {
                0.0
            }
        }
    };
    set_prop(neurone.unwrap(), "sortie", n(sortie));
    Ok(n(sortie))
}
fn ia_gradient(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let neurone = a.get(0);
    let config = a.get(1);
    let err = || nat("la configuration d'entraînement ne correspond pas à la structure du neurone.");
    let poids = prop(neurone, "poids").and_then(|p| vec_de(Some(&p)));
    let entrees = prop(config, "entrees").and_then(|p| vec_de(Some(&p)));
    let (poids, entrees) = match (neurone.filter(|x| x.vrai()), config.filter(|x| x.vrai()), poids, entrees) {
        (Some(_), Some(_), Some(p), Some(e)) if e.len() == p.len() => (p, e),
        _ => return Err(err()),
    };
    let sortie_obt = match prop(config, "sortieObtenue") {
        Some(x) => js_nombre(&x),
        None => num(prop(neurone, "sortie").as_ref()),
    };
    let act = prop(neurone, "activation").map(|x| js_texte(&x)).unwrap_or_default();
    let derivee = match act.as_str() {
        "sigmoid" => sortie_obt * (1.0 - sortie_obt),
        "relu" => {
            if sortie_obt > 0.0 {
                1.0
            } else {
                0.0
            }
        }
        _ => 1.0,
    };
    let gradient = num(prop(config, "erreur").as_ref()) * derivee;
    let taux = num(prop(neurone, "taux_apprentissage").as_ref());
    let nouveaux: Vec<f64> = poids.iter().enumerate().map(|(i, p)| js_nombre(p) + taux * gradient * js_nombre(&entrees[i])).collect();
    let nv = neurone.unwrap();
    set_prop(nv, "poids", Valeur::nombres(&nouveaux));
    let biais = num(prop(neurone, "biais").as_ref()) + taux * gradient;
    set_prop(nv, "biais", n(biais));
    set_prop(nv, "gradient", n(gradient));
    Ok(nv.clone())
}
fn sig(x: f64) -> f64 {
    1.0 / (1.0 + crate::mathfn::exp(-x))
}
#[derive(Clone)]
struct Ne {
    poids: Vec<f64>,
    biais: f64,
    sortie: f64,
    gradient: f64,
}
fn ia_xor(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ep = a.get(0).map_or(f64::NAN, js_parse_int);
    let ep = if ep.is_nan() || ep == 0.0 { 10000.0 } else { ep };
    let data: [([f64; 2], f64); 4] = [([0., 0.], 0.), ([0., 1.], 1.), ([1., 0.], 1.), ([1., 1.], 0.)];
    let creer = |it: &mut Interp| Ne { poids: vec![it.alea() * 2.0 - 1.0, it.alea() * 2.0 - 1.0], biais: it.alea() * 2.0 - 1.0, sortie: 0.0, gradient: 0.0 };
    let mut cachee = vec![creer(it), creer(it)];
    let mut sortie = creer(it);
    let taux = 0.5;
    let propager = |cachee: &mut Vec<Ne>, sortie: &mut Ne, x: f64, y: f64| -> [f64; 2] {
        for ne in cachee.iter_mut() {
            ne.sortie = sig(ne.biais + x * ne.poids[0] + y * ne.poids[1]);
        }
        let es = [cachee[0].sortie, cachee[1].sortie];
        sortie.sortie = sig(sortie.biais + es[0] * sortie.poids[0] + es[1] * sortie.poids[1]);
        es
    };
    let mut e = 0.0;
    while e < ep {
        for (ent, att) in data.iter() {
            let es = propager(&mut cachee, &mut sortie, ent[0], ent[1]);
            sortie.gradient = (att - sortie.sortie) * sortie.sortie * (1.0 - sortie.sortie);
            for (i, nc) in cachee.iter_mut().enumerate() {
                let err = sortie.gradient * sortie.poids[i];
                nc.gradient = err * nc.sortie * (1.0 - nc.sortie);
            }
            for i in 0..2 {
                sortie.poids[i] += taux * sortie.gradient * es[i];
            }
            sortie.biais += taux * sortie.gradient;
            for ne in cachee.iter_mut() {
                for i in 0..2 {
                    ne.poids[i] += taux * ne.gradient * ent[i];
                }
                ne.biais += taux * ne.gradient;
            }
        }
        e += 1.0;
    }
    let arr = |v: f64| js_round(v * 1000.0) / 1000.0;
    let mut res = Dict::nouveau();
    for (k, x, y) in [("0,0", 0., 0.), ("0,1", 0., 1.), ("1,0", 1., 0.), ("1,1", 1., 1.)] {
        propager(&mut cachee, &mut sortie, x, y);
        res.set(k, n(arr(sortie.sortie)));
    }
    Ok(Valeur::struct_de(vec![("epoques", n(ep)), ("resultats", Valeur::structure(res))]))
}
fn ia_multicouche(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let err = || nat("l'architecture (liste) et un jeu de données (structure avec dataset) sont requis.");
    let arch = vec_de(a.get(0)).ok_or_else(err)?;
    let config = a.get(1).filter(|x| x.vrai()).ok_or_else(err)?;
    let dataset = prop(Some(config), "dataset").and_then(|d| vec_de(Some(&d))).ok_or_else(err)?;
    let taux = match prop(Some(config), "tauxApprentissage") {
        Some(x) if x.vrai() => js_nombre(&x),
        _ => 0.3,
    };
    let epoques = match prop(Some(config), "epoques") {
        Some(x) if x.vrai() => js_nombre(&x),
        _ => 5000.0,
    };
    let tailles: Vec<usize> = arch.iter().map(|x| js_nombre(x).max(0.0) as usize).collect();
    let mut couches: Vec<Vec<Ne>> = Vec::new();
    for i in 1..tailles.len() {
        let (nin, nout) = (tailles[i - 1], tailles[i]);
        let c: Vec<Ne> = (0..nout)
            .map(|_| Ne { poids: (0..nin).map(|_| it.alea() * 2.0 - 1.0).collect(), biais: it.alea() * 2.0 - 1.0, sortie: 0.0, gradient: 0.0 })
            .collect();
        couches.push(c);
    }
    let ech: Vec<(Vec<f64>, Vec<f64>)> = dataset
        .iter()
        .map(|e| {
            let g = |k: &str| prop(Some(e), k).and_then(|v| vec_de(Some(&v))).unwrap_or_default().iter().map(js_nombre).collect::<Vec<f64>>();
            (g("entrees"), g("attendu"))
        })
        .collect();
    fn predire(couches: &mut Vec<Vec<Ne>>, entrees: &[f64]) -> Vec<f64> {
        let mut cour = entrees.to_vec();
        for c in couches.iter_mut() {
            cour = c
                .iter_mut()
                .map(|ne| {
                    let s = cour.iter().enumerate().fold(0.0, |a, (i, v)| a + v * ne.poids.get(i).copied().unwrap_or(f64::NAN)) + ne.biais;
                    ne.sortie = sig(s);
                    ne.sortie
                })
                .collect();
        }
        cour
    }
    let mut e = 0.0;
    while e < epoques && !couches.is_empty() {
        for (ent, att) in &ech {
            predire(&mut couches, ent);
            let nc = couches.len();
            for (i, ne) in couches[nc - 1].iter_mut().enumerate() {
                let at = att.get(i).copied().unwrap_or(f64::NAN);
                ne.gradient = (at - ne.sortie) * ne.sortie * (1.0 - ne.sortie);
            }
            for c in (0..nc.saturating_sub(1)).rev() {
                let suivante = couches[c + 1].clone();
                for (i, ne) in couches[c].iter_mut().enumerate() {
                    let mut er = 0.0;
                    for ns in &suivante {
                        er += ns.gradient * ns.poids[i];
                    }
                    ne.gradient = er * ne.sortie * (1.0 - ne.sortie);
                }
            }
            let mut ent_c = ent.clone();
            for c in couches.iter_mut() {
                for ne in c.iter_mut() {
                    for i in 0..ne.poids.len() {
                        ne.poids[i] += taux * ne.gradient * ent_c.get(i).copied().unwrap_or(f64::NAN);
                    }
                    ne.biais += taux * ne.gradient;
                }
                ent_c = c.iter().map(|ne| ne.sortie).collect();
            }
        }
        e += 1.0;
    }
    let mut preds = Vec::new();
    for (i, (ent, _)) in ech.iter().enumerate() {
        let s = predire(&mut couches, ent);
        let brut = prop(Some(&dataset[i]), "entrees").unwrap_or(Valeur::Rien);
        preds.push(Valeur::struct_de(vec![
            ("entrees", brut),
            ("sortie", Valeur::nombres(&s.iter().map(|v| js_round(v * 10000.0) / 10000.0).collect::<Vec<_>>())),
        ]));
    }
    Ok(Valeur::struct_de(vec![("architecture", a[0].clone()), ("predictions", Valeur::liste(preds))]))
}

// ----------------------------------------------------------------- réseau IP et réseaux de neurones
fn cidr_entier(v: Option<&Valeur>, nom: &str) -> R<u32> {
    let c = match v {
        Some(Valeur::Nombre(x)) => *x,
        Some(x) => js_parse_int(x),
        None => f64::NAN,
    };
    if !est_entier(c) || c < 0.0 || c > 32.0 {
        return Err(fad(format!("{} : le préfixe CIDR doit être un entier compris entre 0 et 32.", nom)));
    }
    Ok(c as u32)
}
fn masque(c: u32) -> u32 {
    if c == 0 {
        0
    } else {
        !0u32 << (32 - c)
    }
}
fn ip_vers_num(ip: &Valeur) -> R<u32> {
    let s = js_texte(ip);
    let tr = js_trim(&s);
    let parts: Vec<&str> = tr.split('.').collect();
    let ok = parts.len() == 4 && parts.iter().all(|p| !p.is_empty() && p.len() <= 3 && p.bytes().all(|b| b.is_ascii_digit()));
    if ok {
        let o: Vec<u32> = parts.iter().map(|p| p.parse().unwrap()).collect();
        if o.iter().all(|x| *x <= 255) {
            return Ok((o[0] << 24) | (o[1] << 16) | (o[2] << 8) | o[3]);
        }
    }
    Err(fad(format!("Adresse IPv4 invalide : \"{}\".", s)))
}
fn num_vers_ip(x: u32) -> String {
    format!("{}.{}.{}.{}", x >> 24, (x >> 16) & 255, (x >> 8) & 255, x & 255)
}
fn masque_reseau(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let c = cidr_entier(a.get(0), "masqueReseau")?;
    Ok(t(num_vers_ip(masque(c))))
}
fn plage_reseau(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let c = cidr_entier(a.get(1), "plageReseau")?;
    let m = masque(c);
    let r = ip_vers_num(a.get(0).unwrap_or(&Valeur::Rien))? & m;
    Ok(t(format!("{} - {}", num_vers_ip(r), num_vers_ip(r | !m))))
}
fn creer_reseau(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let mut v = [0usize; 3];
    for (i, nom) in ["entrées", "neurones cachés", "sorties"].iter().enumerate() {
        match a.get(i) {
            Some(Valeur::Nombre(x)) if est_entier(*x) && *x >= 1.0 && *x <= 1000.0 => v[i] = *x as usize,
            _ => return Err(fad(format!("creerReseau : le nombre de {} doit être un entier entre 1 et 1000.", nom))),
        }
    }
    let (e, c, s) = (v[0], v[1], v[2]);
    let al = |it: &mut Interp| it.alea() * 2.0 - 1.0;
    let w1: Vec<Vec<f64>> = (0..c).map(|_| (0..e).map(|_| al(it)).collect()).collect();
    let b1: Vec<f64> = (0..c).map(|_| al(it)).collect();
    let w2: Vec<Vec<f64>> = (0..s).map(|_| (0..c).map(|_| al(it)).collect()).collect();
    let b2: Vec<f64> = (0..s).map(|_| al(it)).collect();
    Ok(Valeur::Reseau(Rc::new(std::cell::RefCell::new(Reseau { n_entrees: e, n_cache: c, n_sorties: s, w1, b1, w2, b2 }))))
}
fn reseau_ia(v: Option<&Valeur>, nom: &str) -> R<Rc<std::cell::RefCell<Reseau>>> {
    match v {
        Some(Valeur::Reseau(r)) => Ok(r.clone()),
        _ => Err(fad(format!("{} : le premier argument doit être un réseau créé avec creerReseau(entrées, cachés, sorties).", nom))),
    }
}
fn liste_ia(v: Option<&Valeur>, nom: &str, quoi: &str) -> R<Vec<Valeur>> {
    vec_de(v).ok_or_else(|| fad(format!("{} : {} doivent être une liste.", nom, quoi)))
}
fn avant(r: &Reseau, entrees: &[Valeur]) -> R<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if entrees.len() != r.n_entrees {
        return Err(fad(format!("Le réseau attend {} entrée(s), {} reçue(s).", r.n_entrees, entrees.len())));
    }
    let x: Vec<f64> = entrees.iter().map(js_nombre).collect();
    if x.iter().any(|v| v.is_nan()) {
        return Err(fad("Les entrées du réseau doivent être des nombres."));
    }
    let h: Vec<f64> = r.w1.iter().enumerate().map(|(i, l)| sig(l.iter().enumerate().fold(r.b1[i], |a, (j, w)| a + w * x[j]))).collect();
    let o: Vec<f64> = r.w2.iter().enumerate().map(|(i, l)| sig(l.iter().enumerate().fold(r.b2[i], |a, (j, w)| a + w * h[j]))).collect();
    Ok((x, h, o))
}
fn ia_predire(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let r = reseau_ia(a.get(0), "iaPredire")?;
    let e = liste_ia(a.get(1), "iaPredire", "les entrées")?;
    let (_, _, o) = avant(&r.borrow(), &e)?;
    Ok(Valeur::nombres(&o))
}
fn ia_entrainer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let rr = reseau_ia(a.get(0), "iaEntrainer")?;
    let taux = match a.get(3) {
        None => 0.5,
        Some(v) => js_nombre(v),
    };
    if taux.is_nan() || taux <= 0.0 {
        return Err(fad("iaEntrainer : le taux d'apprentissage doit être un nombre positif."));
    }
    let e = liste_ia(a.get(1), "iaEntrainer", "les entrées")?;
    let c = liste_ia(a.get(2), "iaEntrainer", "les cibles")?;
    let mut r = rr.borrow_mut();
    if c.len() != r.n_sorties {
        return Err(fad(format!("Le réseau a {} sortie(s) : {} cible(s) fournie(s).", r.n_sorties, c.len())));
    }
    let tg: Vec<f64> = c.iter().map(js_nombre).collect();
    if tg.iter().any(|v| v.is_nan()) {
        return Err(fad("Les cibles du réseau doivent être des nombres."));
    }
    let (x, h, o) = avant(&r, &e)?;
    let d_o: Vec<f64> = o.iter().enumerate().map(|(k, v)| (v - tg[k]) * v * (1.0 - v)).collect();
    let d_h: Vec<f64> = h.iter().enumerate().map(|(j, v)| d_o.iter().enumerate().fold(0.0, |a, (k, d)| a + d * r.w2[k][j]) * v * (1.0 - v)).collect();
    for k in 0..r.n_sorties {
        for j in 0..r.n_cache {
            r.w2[k][j] -= taux * d_o[k] * h[j];
        }
        r.b2[k] -= taux * d_o[k];
    }
    for j in 0..r.n_cache {
        for i in 0..r.n_entrees {
            r.w1[j][i] -= taux * d_h[j] * x[i];
        }
        r.b1[j] -= taux * d_h[j];
    }
    Ok(n(o.iter().enumerate().fold(0.0, |a, (k, v)| a + (v - tg[k]).powi(2)) / o.len() as f64))
}

// ----------------------------------------------------------------- base64, fichiers, tests
fn chiffrer_b64(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    use base64::Engine;
    let s = a.get(0).map(formater).unwrap_or_else(|| "undefined".into());
    Ok(t(base64::engine::general_purpose::STANDARD.encode(s.as_bytes())))
}
fn dechiffrer_b64(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    use base64::Engine;
    let s = texte_arg(a, 0);
    let c: String = s.chars().filter(|c| !c.is_whitespace() && *c != '=').collect();
    match base64::engine::general_purpose::STANDARD_NO_PAD.decode(c) {
        Ok(b) => Ok(t(String::from_utf8_lossy(&b))),
        Err(_) => Err(fad("dechiffrer_base64 : texte Base64 invalide.")),
    }
}
fn ecrire_vfs(it: &mut Interp, nom: &Valeur, contenu: &Valeur, ajouter: bool) -> R<Valeur> {
    let chemin = js_texte(nom);
    let texte = formater(contenu);
    let existant = it.hote.lire_fichier(&chemin).ok();
    let final_ = match (&existant, ajouter) {
        (Some(e), true) => format!("{}{}{}\n", e, if e.ends_with('\n') || e.is_empty() { "" } else { "\n" }, texte),
        (None, true) => format!("{}\n", texte),
        _ => texte,
    };
    match it.hote.ecrire_fichier(&chemin, &final_) {
        Ok(()) => Ok(t(chemin)),
        Err(HoteErr::Indisponible) => Err(fad("Le système de fichiers n'est pas disponible dans cet environnement.")),
        Err(HoteErr::Msg(m)) => Err(fad(format!("\"{}\" : {}", chemin, m))),
    }
}
fn creer_fichier(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let vide = t("");
    ecrire_vfs(it, a.get(0).unwrap_or(&Valeur::Rien), a.get(1).unwrap_or(&vide), false)
}
fn ecrire_log(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let vide = t("");
    ecrire_vfs(it, a.get(0).unwrap_or(&Valeur::Rien), a.get(1).unwrap_or(&vide), true)
}
fn verifier(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    if !a.get(0).map_or(false, |v| v.vrai()) {
        return Err(fad(format!(
            "Échec du test{}.",
            match a.get(1) {
                Some(m) => format!(" : {}", formater(m)),
                None => String::new(),
            }
        )));
    }
    Ok(Valeur::Bool(true))
}
fn verifier_egal(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let (o, at) = (a.get(0).cloned().unwrap_or(Valeur::Rien), a.get(1).cloned().unwrap_or(Valeur::Rien));
    if !egal(&o, &at) {
        let f = |v: &Valeur| match v {
            Valeur::Texte(s) => json_guillemets(s),
            _ => formater(v),
        };
        return Err(fad(format!(
            "Échec du test{} : obtenu {}, attendu {}.",
            match a.get(2) {
                Some(m) => format!(" ({})", formater(m)),
                None => String::new(),
            },
            f(&o),
            f(&at)
        )));
    }
    Ok(Valeur::Bool(true))
}

// ------------------------------------------------------------------ classes (instances) et tests
/// interne : structure vide étiquetée (le constructeur généré par « classe » l'appelle)
fn nat_instance(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let mut d = Dict::nouveau();
    d.classe = Some(std::rc::Rc::from(formater(a.get(0).unwrap_or(&Valeur::Rien)).as_str()));
    Ok(Valeur::structure(d))
}
fn classe_de(v: &Valeur) -> Option<String> {
    match v {
        Valeur::Struct(d) => d.borrow().classe.as_ref().map(|c| c.to_string()),
        _ => None,
    }
}
fn nat_classe_de(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(match classe_de(a.get(0).unwrap_or(&Valeur::Rien)) {
        Some(c) => Valeur::texte(c),
        None => Valeur::Rien,
    })
}
/// vrai si la classe de x est C ou en hérite (remonte les variables « X.#parent »)
fn nat_est_instance(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let cible = formater(a.get(1).unwrap_or(&Valeur::Rien));
    let mut cour = classe_de(a.get(0).unwrap_or(&Valeur::Rien));
    let mut prof = 0;
    while let Some(c) = cour {
        if prof >= 100 {
            break;
        }
        prof += 1;
        if c == cible {
            return Ok(Valeur::Bool(true));
        }
        cour = match it.chercher(&crate::ast::Nom::nouveau(&format!("{}.#parent", c))) {
            Some(Valeur::Texte(p)) => Some(p.to_string()),
            _ => None,
        };
    }
    Ok(Valeur::Bool(false))
}
fn verifier_erreur(it: &mut Interp, a: &[Valeur], ligne: usize) -> R<Valeur> {
    let f = a.get(0).cloned().unwrap_or(Valeur::Rien);
    let msg = match it.appeler_valeur(&f, Vec::<Valeur>::new(), ligne) {
        Ok(_) => None,
        Err(Flow::Err(e)) => Some(e.message.clone()),
        Err(Flow::Nat(m)) => Some(m),
        Err(autre) => return Err(autre),
    };
    match msg {
        None => Err(fad("Échec du test : aucune erreur levée.")),
        Some(m) => {
            if let Some(t) = a.get(1) {
                let t = formater(t);
                if !m.contains(&t) {
                    return Err(fad(format!("Échec du test : l'erreur « {} » ne contient pas « {} ».", m, t)));
                }
            }
            Ok(Valeur::Rien)
        }
    }
}

/// lever(message) : signale une erreur (rattrapable par essayer/attraper) ; utile aux bibliothèques.
fn lever(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Err(fad(match a.get(0) {
        Some(v) => formater(v),
        None => "Erreur levée.".to_string(),
    }))
}
