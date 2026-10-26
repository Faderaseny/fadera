//! Modules standard : dates (UTC), texte, aléatoire, listes, CSV, temps, HTTP.
//! Port du moteur de référence JavaScript (mêmes noms, mêmes messages d'erreur).

use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::erreur::*;
use crate::interp::{HoteErr, Interp};
use crate::natives::NatFn;
use crate::util::*;
use crate::valeur::*;

pub fn trouver(nom: &str) -> Option<NatFn> {
    Some(match nom {
        "date_creer" => date_creer,
        "date_composants" => date_composants,
        "date_ajouter" => date_ajouter,
        "date_difference" => date_difference,
        "date_analyser" => date_analyser,
        "date_formater" => date_formater,
        "est_bissextile" => est_bissextile,
        "date_debut" => date_debut,
        "date_jours_dans_mois" => date_jours_dans_mois,
        "date_ajouter_mois" => date_ajouter_mois,
        "date_semaine" => date_semaine,
        "date_age" => date_age,
        "texte_remplir_gauche" => texte_remplir_gauche,
        "texte_remplir_droite" => texte_remplir_droite,
        "texte_capitaliser" => texte_capitaliser,
        "texte_titre" => texte_titre,
        "texte_lignes" => texte_lignes,
        "texte_mots" => texte_mots,
        "texte_sans_accents" => texte_sans_accents,
        "texte_slug" => texte_slug,
        "texte_est_palindrome" => texte_est_palindrome,
        "texte_distance" => texte_distance,
        "url_encoder" => url_encoder,
        "url_decoder" => url_decoder,
        "alea_uuid" => alea_uuid,
        "alea_entier" => alea_entier,
        "alea_choisir" => alea_choisir,
        "alea_melanger" => alea_melanger,
        "liste_decouper" => liste_decouper,
        "liste_zip" => liste_zip,
        "liste_cumul" => liste_cumul,
        "liste_frequences" => liste_frequences,
        "csv_lire" => csv_lire,
        "csv_texte" => csv_texte,
        "pause" => pause,
        "chrono" => chrono,
        "parler" => parler,
        "http_obtenir" => http_obtenir,
        "http_json" => http_json,
        "http_envoyer" => http_envoyer,
        "pi" => m_pi,
        "sinus" => m_sinus,
        "cosinus" => m_cosinus,
        "tangente" => m_tangente,
        "arc_sinus" => m_arc_sinus,
        "arc_cosinus" => m_arc_cosinus,
        "arc_tangente" => m_arc_tangente,
        "arc_tangente2" => m_arc_tangente2,
        "exponentielle" => m_exponentielle,
        "logarithme" => m_logarithme,
        "logarithme10" => m_logarithme10,
        "radians" => m_radians,
        "degres" => m_degres,
        "signe" => m_signe,
        "modulo" => m_modulo,
        "copier" => m_copier,
        "texte_format" => m_texte_format,
        "en_base" => m_en_base,
        "depuis_base" => m_depuis_base,
        _ => return None,
    })
}

// ------------------------------------------------------------------ helpers
fn n(x: f64) -> Valeur {
    Valeur::Nombre(x)
}
fn t(s: impl AsRef<str>) -> Valeur {
    Valeur::texte(s)
}
fn tp(v: Option<&Valeur>) -> &'static str {
    v.map_or("rien", type_de)
}
fn txt(a: &[Valeur], i: usize, nom: &str) -> R<String> {
    match a.get(i) {
        Some(Valeur::Texte(s)) => Ok(s.to_string()),
        v => Err(fad(format!("{} : l'argument {} doit être un texte (reçu : {}).", nom, i + 1, tp(v)))),
    }
}
fn num(a: &[Valeur], i: usize, nom: &str) -> R<f64> {
    match a.get(i) {
        Some(Valeur::Nombre(x)) if !x.is_nan() => Ok(*x),
        v => Err(fad(format!("{} : l'argument {} doit être un nombre (reçu : {}).", nom, i + 1, tp(v)))),
    }
}
fn liste(a: &[Valeur], i: usize, nom: &str) -> R<Vec<Valeur>> {
    match a.get(i) {
        Some(Valeur::Liste(l)) => Ok(l.borrow().clone()),
        v => Err(fad(format!("{} : l'argument {} doit être une liste (reçu : {}).", nom, i + 1, tp(v)))),
    }
}
/// Aléa dans [0,1) : octets du système si disponibles (crypto), sinon générateur de l'interpréteur.
fn alea01(it: &mut Interp) -> f64 {
    if let Ok(h) = crate::crypto::aleatoire_hex(7) {
        if let Ok(v) = u64::from_str_radix(&h, 16) {
            return v as f64 / (1u64 << 56) as f64;
        }
    }
    it.alea()
}
fn regex(p: &str) -> Regex {
    Regex::new(p).expect("regex interne")
}

// ------------------------------------------------------------------ dates (UTC)
const JOURS: [&str; 7] = ["dimanche", "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi"];
const MOIS: [&str; 12] = [
    "janvier", "février", "mars", "avril", "mai", "juin", "juillet", "août", "septembre", "octobre", "novembre", "décembre",
];

fn jours_depuis_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn unite(v: &Valeur, nom: &str) -> R<f64> {
    let u = js_texte(v).to_lowercase();
    Ok(match u.as_str() {
        "ms" | "milliseconde" | "millisecondes" => 1.0,
        "s" | "seconde" | "secondes" => 1000.0,
        "min" | "minute" | "minutes" => 60_000.0,
        "h" | "heure" | "heures" => 3_600_000.0,
        "j" | "jour" | "jours" => 86_400_000.0,
        "semaine" | "semaines" => 604_800_000.0,
        _ => {
            return Err(fad(format!(
                "{} : unité inconnue « {} » (ms, secondes, minutes, heures, jours, semaines).",
                nom,
                js_texte(v)
            )))
        }
    })
}

fn date_creer(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let an = num(a, 0, "date_creer")?;
    let arg = |i: usize, defaut: f64| -> f64 {
        match a.get(i) {
            None => defaut,
            Some(v) => js_nombre(v),
        }
    };
    let ou0 = |x: f64| if x.is_nan() { 0.0 } else { x }; // `h || 0`
    let m = arg(1, 1.0);
    let j = arg(2, 1.0);
    let h = ou0(arg(3, 0.0));
    let mi = ou0(arg(4, 0.0));
    let s = ou0(arg(5, 0.0));
    if [an, m, j, h, mi, s].iter().any(|x| !x.is_finite()) {
        return Ok(n(f64::NAN));
    }
    let (an, m, j) = (an.trunc() as i64, m.trunc() as i64 - 1, j.trunc() as i64);
    let an = if (0..=99).contains(&an) { 1900 + an } else { an };
    let tot = an * 12 + m;
    let (y, mo) = (tot.div_euclid(12), tot.rem_euclid(12));
    let jours = jours_depuis_civil(y, mo + 1, 1) + (j - 1);
    let ms = jours as f64 * 86_400_000.0 + h.trunc() * 3_600_000.0 + mi.trunc() * 60_000.0 + s.trunc() * 1000.0;
    Ok(n(if ms.abs() > 8.64e15 { f64::NAN } else { ms }))
}

fn date_composants(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ts = match a.get(0) {
        None => it.hote.maintenant_ms().floor(),
        Some(_) => num(a, 0, "date_composants")?,
    };
    let d = date_depuis_ms(ts).ok_or_else(|| nat("Invalid time value"))?;
    let jour_sem = ((ts.trunc() as i64).div_euclid(86_400_000) + 4).rem_euclid(7) as usize;
    Ok(Valeur::struct_de(vec![
        ("annee", n(d.an as f64)),
        ("mois", n(d.mois as f64)),
        ("jour", n(d.jour as f64)),
        ("heure", n(d.h as f64)),
        ("minute", n(d.mi as f64)),
        ("seconde", n(d.s as f64)),
        ("jour_semaine", t(JOURS[jour_sem])),
        ("nom_mois", t(MOIS[(d.mois - 1) as usize])),
        ("numero_semaine_jour", n(jour_sem as f64)),
    ]))
}

fn date_ajouter(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ts = num(a, 0, "date_ajouter")?;
    let k = num(a, 1, "date_ajouter")?;
    let u = match a.get(2) {
        None => unite(&t("jours"), "date_ajouter")?,
        Some(v) => unite(v, "date_ajouter")?,
    };
    Ok(n(ts + k * u))
}

fn date_difference(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a, 0, "date_difference")?;
    let y = num(a, 1, "date_difference")?;
    let u = match a.get(2) {
        None => unite(&t("jours"), "date_difference")?,
        Some(v) => unite(v, "date_difference")?,
    };
    Ok(n((y - x) / u))
}

/// Sous-ensemble ISO 8601 : AAAA-MM-JJ[(T| )hh:mm[:ss[.mmm]]][Z|±hh:mm] ; une date sans heure est en UTC.
fn analyser_iso(s: &str) -> Option<f64> {
    let re = regex(r"^(\d{4})-(\d{2})-(\d{2})(?:[T ](\d{2}):(\d{2})(?::(\d{2})(?:\.(\d{1,9}))?)?)?\s*(Z|[+-]\d{2}:?\d{2})?$");
    let c = re.captures(s.trim())?;
    let g = |i: usize| c.get(i).map_or(0i64, |m| m.as_str().parse::<i64>().unwrap_or(0));
    let (an, mo, j, h, mi, sec) = (g(1), g(2), g(3), g(4), g(5), g(6));
    if !(1..=12).contains(&mo) || !(1..=31).contains(&j) || h > 24 || mi > 59 || sec > 59 {
        return None;
    }
    let ms = c.get(7).map_or(0, |m| {
        let f = format!("{:0<3}", m.as_str());
        f[..3].parse::<i64>().unwrap_or(0)
    });
    let mut tot = jours_depuis_civil(an, mo, j) * 86_400_000 + h * 3_600_000 + mi * 60_000 + sec * 1000 + ms;
    if let Some(z) = c.get(8) {
        let z = z.as_str();
        if z != "Z" {
            let sg = if z.starts_with('-') { -1 } else { 1 };
            let d: String = z[1..].chars().filter(|x| x.is_ascii_digit()).collect();
            let (zh, zm) = (d[..2].parse::<i64>().ok()?, d[2..].parse::<i64>().ok()?);
            tot -= sg * (zh * 60 + zm) * 60_000;
        }
    }
    Some(tot as f64)
}
fn date_analyser(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = txt(a, 0, "date_analyser")?;
    match analyser_iso(&s) {
        Some(v) => Ok(n(v)),
        None => Err(fad(format!(
            "date_analyser : date illisible « {} » (format conseillé : 2026-10-14 ou 2026-10-14T10:30:00).",
            s
        ))),
    }
}

fn date_formater(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ts = match a.get(0) {
        None => it.hote.maintenant_ms().floor(),
        Some(_) => num(a, 0, "date_formater")?,
    };
    let modele = a.get(1).map_or("AAAA-MM-JJ hh:mm:ss".to_string(), js_texte);
    let d = date_depuis_ms(ts).ok_or_else(|| nat("Invalid time value"))?;
    let jour_sem = ((ts.trunc() as i64).div_euclid(86_400_000) + 4).rem_euclid(7) as usize;
    let jetons: [(&str, String); 8] = [
        ("AAAA", d.an.to_string()),
        ("MM", format!("{:02}", d.mois)),
        ("JJ", format!("{:02}", d.jour)),
        ("hh", format!("{:02}", d.h)),
        ("mm", format!("{:02}", d.mi)),
        ("ss", format!("{:02}", d.s)),
        ("jour", JOURS[jour_sem].to_string()),
        ("mois", MOIS[(d.mois - 1) as usize].to_string()),
    ];
    let mut out = String::new();
    let mut reste = modele.as_str();
    'boucle: while !reste.is_empty() {
        for (k, v) in jetons.iter() {
            if reste.starts_with(k) {
                out.push_str(v);
                reste = &reste[k.len()..];
                continue 'boucle;
            }
        }
        let c = reste.chars().next().unwrap();
        out.push(c);
        reste = &reste[c.len_utf8()..];
    }
    Ok(t(out))
}

fn est_bissextile(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let an = num(a, 0, "est_bissextile")?;
    Ok(Valeur::Bool((an % 4.0 == 0.0 && an % 100.0 != 0.0) || an % 400.0 == 0.0))
}

fn bissextile(an: f64) -> bool {
    (an % 4.0 == 0.0 && an % 100.0 != 0.0) || an % 400.0 == 0.0
}
fn jours_du_mois(an: f64, m: usize) -> u32 {
    [31, if bissextile(an) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][m]
}
const JOUR_MS: f64 = 86_400_000.0;
/// Début du jour / de la semaine (lundi) / du mois / de l'année qui contient la date.
fn date_debut(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ts = num(a, 0, "date_debut")?;
    let k = match a.get(1) {
        None => "jour".to_string(),
        Some(v) => js_texte(v).to_lowercase(),
    };
    let d = match date_depuis_ms(ts) {
        Some(d) => d,
        None => return Ok(n(f64::NAN)),
    };
    let jours = |y: i64, m: i64, j: i64| jours_depuis_civil(y, m, j) as f64 * JOUR_MS;
    Ok(n(match k.as_str() {
        "jour" => jours(d.an, d.mois as i64, d.jour as i64),
        "semaine" => {
            let wd = ((ts.trunc() as i64).div_euclid(86_400_000) + 3).rem_euclid(7);
            jours(d.an, d.mois as i64, d.jour as i64) - wd as f64 * JOUR_MS
        }
        "mois" => jours(d.an, d.mois as i64, 1),
        "annee" | "année" | "an" => jours(d.an, 1, 1),
        _ => {
            return Err(fad(format!(
                "date_debut : unité inconnue « {} » (jour, semaine, mois, annee).",
                a.get(1).map(js_texte).unwrap_or_default()
            )))
        }
    }))
}
fn date_jours_dans_mois(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let an = num(a, 0, "date_jours_dans_mois")?;
    let m = num(a, 1, "date_jours_dans_mois")?;
    if an.fract() != 0.0 || m.fract() != 0.0 || !an.is_finite() || !(1.0..=12.0).contains(&m) {
        return Err(fad("date_jours_dans_mois : le mois doit être un entier de 1 à 12."));
    }
    Ok(n(jours_du_mois(an, m as usize - 1) as f64))
}
/// Ajoute des mois en gardant le jour (ramené à la fin du mois si besoin).
fn date_ajouter_mois(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ts = num(a, 0, "date_ajouter_mois")?;
    let nb = num(a, 1, "date_ajouter_mois")?;
    if nb.fract() != 0.0 || !nb.is_finite() {
        return Err(fad("date_ajouter_mois : le nombre de mois doit être un entier."));
    }
    let d = match date_depuis_ms(ts) {
        Some(d) => d,
        None => return Ok(n(f64::NAN)),
    };
    let tot = d.an * 12 + (d.mois as i64 - 1) + nb as i64;
    let (an, mo) = (tot.div_euclid(12), tot.rem_euclid(12));
    let jour = d.jour.min(jours_du_mois(an as f64, mo as usize)) as i64;
    let ms = jours_depuis_civil(an, mo + 1, jour) as f64 * JOUR_MS
        + d.h as f64 * 3_600_000.0
        + d.mi as f64 * 60_000.0
        + d.s as f64 * 1000.0
        + d.ms as f64;
    Ok(n(if ms.abs() > 8.64e15 { f64::NAN } else { ms }))
}
/// Numéro de semaine ISO 8601 (1 à 53).
fn date_semaine(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ts = num(a, 0, "date_semaine")?;
    let d = match date_depuis_ms(ts) {
        Some(d) => d,
        None => return Ok(n(f64::NAN)),
    };
    let nj = jours_depuis_civil(d.an, d.mois as i64, d.jour as i64);
    let wd = (nj + 3).rem_euclid(7);
    let jeudi = nj - wd + 3;
    let an = match date_depuis_ms(jeudi as f64 * JOUR_MS) {
        Some(x) => x.an,
        None => return Ok(n(f64::NAN)),
    };
    Ok(n(((jeudi - jours_depuis_civil(an, 1, 1)).div_euclid(7) + 1) as f64))
}
/// Âge en années entières à une date (aujourd'hui par défaut).
fn date_age(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let nais = num(a, 0, "date_age")?;
    let r = match a.get(1) {
        None => it.hote.maintenant_ms().floor(),
        Some(_) => num(a, 1, "date_age")?,
    };
    match (date_depuis_ms(nais), date_depuis_ms(r)) {
        (Some(x), Some(y)) => {
            let mut age = y.an - x.an;
            if y.mois < x.mois || (y.mois == x.mois && y.jour < x.jour) {
                age -= 1;
            }
            Ok(n(age as f64))
        }
        _ => Ok(n(f64::NAN)),
    }
}

// ------------------------------------------------------------------ texte
fn units(s: &str) -> usize {
    s.encode_utf16().count()
}
fn remplir(a: &[Valeur], nom: &str, gauche: bool) -> R<Valeur> {
    let s = txt(a, 0, nom)?;
    let long = num(a, 1, nom)?;
    let c = a.get(2).map_or(" ".to_string(), js_texte);
    let cible = if long.is_finite() && long > 0.0 { long.trunc() as usize } else { 0 };
    let act = units(&s);
    if cible <= act || c.is_empty() {
        return Ok(t(s));
    }
    let manque = cible - act;
    let cu: Vec<u16> = c.encode_utf16().collect();
    let pad: Vec<u16> = (0..manque).map(|i| cu[i % cu.len()]).collect();
    let pad = String::from_utf16_lossy(&pad);
    Ok(t(if gauche { pad + &s } else { s + &pad }))
}
fn texte_remplir_gauche(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    remplir(a, "texte_remplir_gauche", true)
}
fn texte_remplir_droite(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    remplir(a, "texte_remplir_droite", false)
}
fn texte_capitaliser(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = txt(a, 0, "texte_capitaliser")?;
    let mut ch = s.chars();
    Ok(t(match ch.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + &ch.as_str().to_lowercase(),
    }))
}
fn texte_titre(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = txt(a, 0, "texte_titre")?.to_lowercase();
    let re = regex(r"(^|[\s'’-])(\p{L})");
    Ok(t(re.replace_all(&s, |c: &regex::Captures| format!("{}{}", &c[1], c[2].to_uppercase())).into_owned()))
}
fn texte_lignes(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = txt(a, 0, "texte_lignes")?;
    Ok(Valeur::liste(regex(r"\r?\n").split(&s).map(t).collect()))
}
fn texte_mots(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = txt(a, 0, "texte_mots")?;
    Ok(Valeur::liste(regex(r"[^\p{L}\p{N}'’-]+").split(&s).filter(|x| !x.is_empty()).map(t).collect()))
}
fn sans_accents(s: &str) -> String {
    s.nfd().filter(|c| !('\u{0300}'..='\u{036f}').contains(c)).collect()
}
fn texte_sans_accents(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(t(sans_accents(&txt(a, 0, "texte_sans_accents")?)))
}
fn texte_slug(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = sans_accents(&txt(a, 0, "texte_slug")?).to_lowercase();
    let s = regex(r"[^a-z0-9]+").replace_all(&s, "-").into_owned();
    Ok(t(regex(r"^-+|-+$").replace_all(&s, "").into_owned()))
}
fn texte_est_palindrome(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s: Vec<char> = sans_accents(&txt(a, 0, "texte_est_palindrome")?)
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        .collect();
    Ok(Valeur::Bool(s.iter().eq(s.iter().rev())))
}
fn texte_distance(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x: Vec<u16> = txt(a, 0, "texte_distance")?.encode_utf16().collect();
    let y: Vec<u16> = txt(a, 1, "texte_distance")?.encode_utf16().collect();
    let mut p: Vec<usize> = (0..=y.len()).collect();
    for i in 1..=x.len() {
        let mut c = vec![i];
        for j in 1..=y.len() {
            let v = (p[j] + 1).min(c[j - 1] + 1).min(p[j - 1] + if x[i - 1] == y[j - 1] { 0 } else { 1 });
            c.push(v);
        }
        p = c;
    }
    Ok(n(p[y.len()] as f64))
}
fn url_encoder(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = txt(a, 0, "url_encoder")?;
    let mut o = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&b) {
            o.push(b as char);
        } else {
            o.push_str(&format!("%{:02X}", b));
        }
    }
    Ok(t(o))
}
fn url_decoder(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = txt(a, 0, "url_decoder")?;
    let b = s.as_bytes();
    let mut o: Vec<u8> = Vec::new();
    let mut i = 0;
    let invalide = || fad("url_decoder : séquence % invalide.");
    while i < b.len() {
        if b[i] == b'%' {
            if i + 2 >= b.len() || !b[i + 1].is_ascii_hexdigit() || !b[i + 2].is_ascii_hexdigit() {
                return Err(invalide());
            }
            let h = std::str::from_utf8(&b[i + 1..i + 3]).map_err(|_| invalide())?;
            o.push(u8::from_str_radix(h, 16).map_err(|_| invalide())?);
            i += 3;
        } else {
            o.push(b[i]);
            i += 1;
        }
    }
    String::from_utf8(o).map(t).map_err(|_| invalide())
}

// ------------------------------------------------------------------ aléatoire
fn alea_uuid(it: &mut Interp, _: &[Valeur], _: usize) -> R<Valeur> {
    let mut o = [0u8; 16];
    match crate::crypto::aleatoire_hex(16) {
        Ok(h) => {
            for (i, x) in o.iter_mut().enumerate() {
                *x = u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).unwrap_or(0);
            }
        }
        Err(_) => {
            for x in o.iter_mut() {
                *x = (it.alea() * 256.0) as u8;
            }
        }
    }
    o[6] = (o[6] & 0x0f) | 0x40;
    o[8] = (o[8] & 0x3f) | 0x80;
    let h: String = o.iter().map(|x| format!("{:02x}", x)).collect();
    Ok(t(format!("{}-{}-{}-{}-{}", &h[0..8], &h[8..12], &h[12..16], &h[16..20], &h[20..32])))
}
fn alea_entier(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a, 0, "alea_entier")?;
    let y = num(a, 1, "alea_entier")?;
    if y < x {
        return Err(fad("alea_entier : le minimum doit être ≤ au maximum."));
    }
    Ok(n(x + (alea01(it) * (y - x + 1.0)).floor()))
}
fn alea_choisir(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let l = liste(a, 0, "alea_choisir")?;
    if l.is_empty() {
        return Err(fad("alea_choisir : la liste est vide."));
    }
    let i = (alea01(it) * l.len() as f64).floor() as usize;
    Ok(l[i.min(l.len() - 1)].clone())
}
fn alea_melanger(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let mut l = liste(a, 0, "alea_melanger")?;
    for i in (1..l.len()).rev() {
        let j = (alea01(it) * (i + 1) as f64).floor() as usize;
        l.swap(i, j.min(i));
    }
    Ok(Valeur::liste(l))
}

// ------------------------------------------------------------------ listes
fn liste_decouper(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let l = liste(a, 0, "liste_decouper")?;
    let k = num(a, 1, "liste_decouper")?;
    if k < 1.0 {
        return Err(fad("liste_decouper : la taille doit être ≥ 1."));
    }
    let mut r = Vec::new();
    let mut i = 0.0f64;
    while i < l.len() as f64 {
        let d = (i.trunc() as usize).min(l.len());
        let f = ((i + k).trunc() as usize).min(l.len());
        r.push(Valeur::liste(if d < f { l[d..f].to_vec() } else { vec![] }));
        i += k;
    }
    Ok(Valeur::liste(r))
}
fn liste_zip(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = liste(a, 0, "liste_zip")?;
    let y = liste(a, 1, "liste_zip")?;
    Ok(Valeur::liste(
        x.iter().zip(y.iter()).map(|(p, q)| Valeur::liste(vec![p.clone(), q.clone()])).collect(),
    ))
}
fn liste_cumul(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let mut s = 0.0;
    Ok(Valeur::liste(
        liste(a, 0, "liste_cumul")?
            .iter()
            .map(|v| {
                s += js_nombre(v);
                n(s)
            })
            .collect(),
    ))
}
fn liste_frequences(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let mut d = Dict::nouveau();
    for v in liste(a, 0, "liste_frequences")? {
        let k = match &v {
            Valeur::Texte(s) => s.to_string(),
            o => formater(o),
        };
        let c = match d.get(&k) {
            Some(Valeur::Nombre(x)) => *x,
            _ => 0.0,
        };
        d.set(&k, n(c + 1.0));
    }
    Ok(Valeur::structure(d))
}

// ------------------------------------------------------------------ CSV
fn csv_ligne(l: &str, sep: &str) -> Vec<String> {
    let mut r = Vec::new();
    let mut c = String::new();
    let mut q = false;
    let ch: Vec<char> = l.chars().collect();
    let mut i = 0;
    while i < ch.len() {
        let x = ch[i];
        if q {
            if x == '"' {
                if ch.get(i + 1) == Some(&'"') {
                    c.push('"');
                    i += 1;
                } else {
                    q = false;
                }
            } else {
                c.push(x);
            }
        } else if x == '"' {
            q = true;
        } else if x.to_string() == sep {
            r.push(std::mem::take(&mut c));
        } else {
            c.push(x);
        }
        i += 1;
    }
    r.push(c);
    r
}
fn csv_lire(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = txt(a, 0, "csv_lire")?;
    let sep = a.get(1).map_or(",".to_string(), js_texte);
    let lignes = regex(r"\r?\n")
        .split(&s)
        .filter(|l| !l.is_empty())
        .map(|l| {
            Valeur::liste(
                csv_ligne(l, &sep)
                    .into_iter()
                    .map(|c| {
                        if !js_trim(&c).is_empty() && !js_nombre_texte(&c).is_nan() {
                            n(js_nombre_texte(&c))
                        } else {
                            t(c)
                        }
                    })
                    .collect(),
            )
        })
        .collect();
    Ok(Valeur::liste(lignes))
}
fn csv_texte(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let sep = a.get(1).map_or(",".to_string(), js_texte);
    let mut lignes = Vec::new();
    for r in liste(a, 0, "csv_texte")? {
        let cellules = liste(std::slice::from_ref(&r), 0, "csv_texte")?;
        let cs: Vec<String> = cellules
            .iter()
            .map(|c| {
                let x = if matches!(c, Valeur::Rien) { "undefined".to_string() } else { js_texte(c) };
                if x.contains(sep.as_str()) || x.contains('"') || x.contains('\n') {
                    format!("\"{}\"", x.replace('"', "\"\""))
                } else {
                    x
                }
            })
            .collect();
        lignes.push(cs.join(&sep));
    }
    Ok(t(lignes.join("\n")))
}

// ------------------------------------------------------------------ temps
fn pause(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let ms = num(a, 0, "pause")?;
    it.hote.pause(ms.max(0.0).min(10_000.0));
    Ok(n(ms))
}
fn parler(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let texte = txt(a, 0, "parler")?;
    it.hote.parler(&texte);
    Ok(t(texte))
}
fn chrono(it: &mut Interp, _: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(it.hote.maintenant_ms()))
}

// ------------------------------------------------------------------ HTTP (via l'hôte)
fn http(it: &mut Interp, nom: &str, url: &str, methode: &str, corps: Option<String>, entetes: Vec<(String, String)>) -> R<(u16, String)> {
    let url = if url.starts_with("http://") || url.starts_with("https://") { url.to_string() } else { format!("https://{}", url) };
    it.hote.http(methode, &url, corps.as_deref(), &entetes).map_err(|e| match e {
        HoteErr::Indisponible => fad(format!("{} : réseau indisponible dans cet environnement.", nom)),
        HoteErr::Msg(_) => fad(format!(
            "{} : requête impossible vers {} (réseau coupé ou site qui refuse l'accès depuis le navigateur — règle CORS).",
            nom, url
        )),
    })
}
/// En-têtes facultatifs : une structure { "Authorization": "Bearer …" } à la position `i`.
fn entetes_http(a: &[Valeur], i: usize, nom: &str) -> R<Vec<(String, String)>> {
    let mut h: Vec<(String, String)> = Vec::new();
    match a.get(i) {
        None => {}
        Some(Valeur::Struct(d)) => {
            let d = d.borrow();
            for k in d.cles_ordonnees() {
                h.push((k.clone(), js_texte(d.get(&k).unwrap_or(&Valeur::Rien))));
            }
        }
        Some(_) => return Err(fad(format!("{} : les en-têtes doivent être une structure.", nom))),
    }
    Ok(h)
}
fn http_obtenir(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let u = txt(a, 0, "http_obtenir")?;
    let h = entetes_http(a, 1, "http_obtenir")?;
    Ok(t(http(it, "http_obtenir", &u, "GET", None, h)?.1))
}
fn http_json(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let u = txt(a, 0, "http_json")?;
    let h = entetes_http(a, 1, "http_json")?;
    let (_, corps) = http(it, "http_json", &u, "GET", None, h)?;
    json_lire(&corps).map_err(|_| fad(format!("http_json : la réponse de {} n'est pas du JSON valide.", u)))
}
fn http_envoyer(it: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let u = txt(a, 0, "http_envoyer")?;
    let mut h = entetes_http(a, 2, "http_envoyer")?;
    let est_obj = matches!(a.get(1), Some(Valeur::Liste(_)) | Some(Valeur::Struct(_)));
    if est_obj && !h.iter().any(|(k, _)| k == "Content-Type") {
        h.push(("Content-Type".into(), "application/json".into()));
    }
    let corps = match a.get(1) {
        None => None,
        Some(v) if est_obj => Some(json_texte(v, "")?),
        Some(v) => Some(js_texte(v)),
    };
    let (statut, texte) = http(it, "http_envoyer", &u, "POST", corps, h)?;
    Ok(Valeur::struct_de(vec![
        ("statut", n(statut as f64)),
        ("ok", Valeur::Bool((200..300).contains(&statut))),
        ("corps", t(texte)),
    ]))
}

// ------------------------------------------------------------------ maths étendues, copie, texte à trous, bases
fn m_pi(_: &mut Interp, _: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(std::f64::consts::PI))
}
fn m_sinus(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(crate::mathfn::sin(num(a, 0, "sinus")?)))
}
fn m_cosinus(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(crate::mathfn::cos(num(a, 0, "cosinus")?)))
}
fn m_tangente(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(crate::mathfn::tan(num(a, 0, "tangente")?)))
}
fn m_arc_sinus(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a, 0, "arc_sinus")?;
    if !(-1.0..=1.0).contains(&x) {
        return Err(fad("arc_sinus : l'argument doit être entre -1 et 1."));
    }
    Ok(n(crate::mathfn::asin(x)))
}
fn m_arc_cosinus(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a, 0, "arc_cosinus")?;
    if !(-1.0..=1.0).contains(&x) {
        return Err(fad("arc_cosinus : l'argument doit être entre -1 et 1."));
    }
    Ok(n(crate::mathfn::acos(x)))
}
fn m_arc_tangente(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(crate::mathfn::atan(num(a, 0, "arc_tangente")?)))
}
fn m_arc_tangente2(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let y = num(a, 0, "arc_tangente2")?;
    let x = num(a, 1, "arc_tangente2")?;
    Ok(n(crate::mathfn::atan2(y, x)))
}
fn m_exponentielle(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(crate::mathfn::exp(num(a, 0, "exponentielle")?)))
}
fn m_logarithme(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a, 0, "logarithme")?;
    if !(x > 0.0) {
        return Err(fad("logarithme : l'argument doit être strictement positif."));
    }
    if matches!(a.get(1), None) {
        return Ok(n(crate::mathfn::log(x)));
    }
    let b = num(a, 1, "logarithme")?;
    if !(b > 0.0) || b == 1.0 {
        return Err(fad("logarithme : la base doit être positive et différente de 1."));
    }
    Ok(n(if b == 2.0 {
        crate::mathfn::log2(x)
    } else if b == 10.0 {
        crate::mathfn::log10(x)
    } else {
        crate::mathfn::log(x) / crate::mathfn::log(b)
    }))
}
fn m_logarithme10(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a, 0, "logarithme10")?;
    if !(x > 0.0) {
        return Err(fad("logarithme10 : l'argument doit être strictement positif."));
    }
    Ok(n(crate::mathfn::log10(x)))
}
fn m_radians(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(num(a, 0, "radians")? * std::f64::consts::PI / 180.0))
}
fn m_degres(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    Ok(n(num(a, 0, "degres")? * 180.0 / std::f64::consts::PI))
}
fn m_signe(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a, 0, "signe")?;
    Ok(n(if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }))
}
fn m_modulo(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a, 0, "modulo")?;
    let b = num(a, 1, "modulo")?;
    if b == 0.0 {
        return Err(fad("modulo : division par zéro."));
    }
    Ok(n(((x % b) + b) % b))
}

fn copie_profonde(v: &Valeur, vus: &mut std::collections::HashMap<usize, Valeur>) -> Valeur {
    match v {
        Valeur::Liste(l) => {
            let cle = std::rc::Rc::as_ptr(l) as *const u8 as usize;
            if let Some(d) = vus.get(&cle) {
                return d.clone();
            }
            let neuve = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            let res = Valeur::Liste(neuve.clone());
            vus.insert(cle, res.clone());
            let src: Vec<Valeur> = l.borrow().clone();
            for e in &src {
                let c = copie_profonde(e, vus);
                neuve.borrow_mut().push(c);
            }
            res
        }
        Valeur::Struct(d) => {
            let cle = std::rc::Rc::as_ptr(d) as *const u8 as usize;
            if let Some(x) = vus.get(&cle) {
                return x.clone();
            }
            let mut vide = Dict::nouveau();
            vide.classe = d.borrow().classe.clone();
            let neuve = std::rc::Rc::new(std::cell::RefCell::new(vide));
            let res = Valeur::Struct(neuve.clone());
            vus.insert(cle, res.clone());
            let paires: Vec<(String, Valeur)> = {
                let b = d.borrow();
                b.cles_ordonnees().into_iter().filter_map(|k| b.get(&k).cloned().map(|v| (k, v))).collect()
            };
            for (k, e) in &paires {
                let c = copie_profonde(e, vus);
                neuve.borrow_mut().set(k, c);
            }
            res
        }
        autre => autre.clone(),
    }
}
fn m_copier(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let v = a.get(0).cloned().unwrap_or(Valeur::Rien);
    Ok(copie_profonde(&v, &mut std::collections::HashMap::new()))
}

fn m_texte_format(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let modele = txt(a, 0, "texte_format")?;
    let (cles, liste): (Option<Valeur>, Vec<Valeur>) = match a.get(1) {
        Some(v @ Valeur::Struct(_)) => (Some(v.clone()), Vec::new()),
        Some(Valeur::Liste(l)) => (None, l.borrow().clone()),
        None | Some(Valeur::Rien) => (None, Vec::new()),
        Some(v) => (None, vec![v.clone()]),
    };
    let car: Vec<char> = modele.chars().collect();
    let mut out = String::new();
    let mut suiv = 0usize;
    let mut i = 0usize;
    while i < car.len() {
        let c = car[i];
        if c == '{' {
            if car.get(i + 1) == Some(&'{') {
                out.push('{');
                i += 2;
                continue;
            }
            let j = match car[i + 1..].iter().position(|&x| x == '}') {
                Some(p) => i + 1 + p,
                None => return Err(fad("texte_format : accolade « { » non fermée (écrivez {{ pour un « { » littéral).")),
            };
            let nom: String = car[i + 1..j].iter().collect();
            i = j + 1;
            if let Some(Valeur::Struct(d)) = &cles {
                if nom.is_empty() {
                    return Err(fad("texte_format : « {} » demande une liste de valeurs ; avec une structure, écrivez {nom}."));
                }
                match d.borrow().get(&nom) {
                    Some(v) => out.push_str(&formater(v)),
                    None => return Err(fad(format!("texte_format : la clé « {} » n'existe pas dans la structure.", nom))),
                }
            } else {
                let idx: usize = if nom.is_empty() {
                    suiv += 1;
                    suiv - 1
                } else if nom.chars().all(|x| x.is_ascii_digit()) {
                    nom.parse::<usize>().unwrap_or(usize::MAX)
                } else {
                    return Err(fad(format!("texte_format : « {{{}}} » demande une structure comme 2e argument.", nom)));
                };
                match liste.get(idx) {
                    Some(v) => out.push_str(&formater(v)),
                    None => {
                        let shown = if nom.is_empty() || nom.parse::<usize>().is_ok() { idx.to_string() } else { nom.clone() };
                        return Err(fad(format!("texte_format : aucune valeur pour {{{}}} ({} valeur(s) fournie(s)).", shown, liste.len())));
                    }
                }
            }
        } else if c == '}' {
            if car.get(i + 1) == Some(&'}') {
                out.push('}');
                i += 2;
            } else {
                return Err(fad("texte_format : accolade « } » isolée (écrivez }} pour un « } » littéral)."));
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    Ok(t(out))
}

const MAX_ENTIER_SUR: f64 = 9007199254740991.0;
fn m_en_base(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let x = num(a, 0, "en_base")?;
    let b = num(a, 1, "en_base")?;
    if !x.is_finite() || x.fract() != 0.0 || x.abs() > MAX_ENTIER_SUR {
        return Err(fad("en_base : le nombre doit être un entier (valeur absolue d'au plus 9007199254740991)."));
    }
    if b.fract() != 0.0 || !(2.0..=36.0).contains(&b) {
        return Err(fad("en_base : la base doit être un entier de 2 à 36."));
    }
    let base = b as u64;
    let mut m = x.abs() as u64;
    if m == 0 {
        return Ok(t("0"));
    }
    let chiffres = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut v: Vec<u8> = Vec::new();
    while m > 0 {
        v.push(chiffres[(m % base) as usize]);
        m /= base;
    }
    if x < 0.0 {
        v.push(b'-');
    }
    v.reverse();
    Ok(t(String::from_utf8(v).unwrap_or_default()))
}
fn m_depuis_base(_: &mut Interp, a: &[Valeur], _: usize) -> R<Valeur> {
    let s = txt(a, 0, "depuis_base")?;
    let b = num(a, 1, "depuis_base")?;
    if b.fract() != 0.0 || !(2.0..=36.0).contains(&b) {
        return Err(fad("depuis_base : la base doit être un entier de 2 à 36."));
    }
    let neg = s.starts_with('-');
    let ch = if neg { &s[1..] } else { &s[..] };
    let invalide = || fad(format!("depuis_base : « {} » n'est pas un nombre valide en base {}.", s, b));
    if ch.is_empty() {
        return Err(invalide());
    }
    let mut v = 0.0f64;
    for c in ch.chars() {
        let d = match c.to_digit(36) {
            Some(d) if c.is_ascii_alphanumeric() => d as f64,
            _ => return Err(invalide()),
        };
        if d >= b {
            return Err(invalide());
        }
        v = v * b + d;
        if v > MAX_ENTIER_SUR {
            return Err(fad("depuis_base : valeur trop grande (au plus 9007199254740991)."));
        }
    }
    Ok(n(if neg && v != 0.0 { -v } else { v }))
}
