use crate::erreur::*;
use crate::valeur::*;

/// `Number.prototype.toFixed` (arrondi des moitiés vers le haut sur la valeur exacte).
pub fn to_fixed(v: f64, d: usize) -> String {
    if !v.is_finite() {
        return js_num(v);
    }
    if v == 0.0 {
        return format!("{:.*}", d, 0.0);
    }
    let neg = v < 0.0;
    let s = format!("{:.*}", d + 30, v.abs());
    let (ent, frac) = s.split_once('.').unwrap();
    let mut chiffres: Vec<u8> = ent.bytes().chain(frac.bytes().take(d)).collect();
    let suivant = frac.as_bytes()[d];
    if suivant >= b'5' {
        let mut i = chiffres.len();
        loop {
            if i == 0 {
                chiffres.insert(0, b'1');
                break;
            }
            i -= 1;
            if chiffres[i] == b'9' {
                chiffres[i] = b'0';
            } else {
                chiffres[i] += 1;
                break;
            }
        }
    }
    let lg = chiffres.len() - d;
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    out.push_str(std::str::from_utf8(&chiffres[..lg]).unwrap());
    if d > 0 {
        out.push('.');
        out.push_str(std::str::from_utf8(&chiffres[lg..]).unwrap());
    }
    out
}

/// Tri fusion stable avec comparaison faillible (même règle que le moteur JS : `<= 0` garde l'élément de gauche).
pub fn tri_fusion<T: Clone>(a: Vec<T>, cmp: &mut dyn FnMut(&T, &T) -> R<f64>) -> R<Vec<T>> {
    if a.len() < 2 {
        return Ok(a);
    }
    let m = a.len() / 2;
    let g = tri_fusion(a[..m].to_vec(), cmp)?;
    let d = tri_fusion(a[m..].to_vec(), cmp)?;
    let mut r = Vec::with_capacity(g.len() + d.len());
    let (mut i, mut j) = (0, 0);
    while i < g.len() && j < d.len() {
        if cmp(&g[i], &d[j])? <= 0.0 {
            r.push(g[i].clone());
            i += 1;
        } else {
            r.push(d[j].clone());
            j += 1;
        }
    }
    r.extend_from_slice(&g[i..]);
    r.extend_from_slice(&d[j..]);
    Ok(r)
}

pub fn cmp_defaut(a: &Valeur, b: &Valeur) -> f64 {
    match (a, b) {
        (Valeur::Nombre(x), Valeur::Nombre(y)) => {
            let r = x - y;
            if r.is_nan() {
                0.0
            } else {
                r
            }
        }
        // Listes : élément par élément ; à égalité de début, la plus courte d'abord.
        (Valeur::Liste(x), Valeur::Liste(y)) => {
            let (x, y) = (x.borrow(), y.borrow());
            for (p, q) in x.iter().zip(y.iter()) {
                let c = cmp_defaut(p, q);
                if c != 0.0 && !c.is_nan() {
                    return c;
                }
            }
            x.len() as f64 - y.len() as f64
        }
        _ => match locale_compare(&formater(a), &formater(b)) {
            std::cmp::Ordering::Less => -1.0,
            std::cmp::Ordering::Equal => 0.0,
            std::cmp::Ordering::Greater => 1.0,
        },
    }
}

/// Bornes d'un `slice(debut, fin)` JavaScript.
pub fn js_slice(len: usize, debut: f64, fin: Option<f64>) -> (usize, usize) {
    let l = len as f64;
    let rel = |x: f64| -> usize {
        let x = if x.is_nan() { 0.0 } else { x.trunc() };
        if x < 0.0 {
            (l + x).max(0.0) as usize
        } else {
            x.min(l) as usize
        }
    };
    let s = rel(debut);
    let e = match fin {
        None => len,
        Some(f) => rel(f),
    };
    (s, e.max(s))
}

// ---------------------------------------------------------------- JSON
pub fn json_texte(v: &Valeur, indent: &str) -> R<String> {
    let mut out = String::new();
    json_ecrire(v, indent, 0, &mut out)?;
    Ok(out)
}

fn json_ecrire(v: &Valeur, ind: &str, niv: usize, out: &mut String) -> R<()> {
    let nl = |out: &mut String, n: usize| {
        if !ind.is_empty() {
            out.push('\n');
            for _ in 0..n {
                out.push_str(ind);
            }
        }
    };
    match v {
        Valeur::Fonction(_) | Valeur::Reseau(_) => {
            return Err(fad("json_texte : une fonction ou un réseau ne peut pas être converti en JSON."))
        }
        Valeur::Rien => out.push_str("null"),
        Valeur::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Valeur::Nombre(n) => out.push_str(&if n.is_finite() { js_num(*n) } else { "null".to_string() }),
        Valeur::Texte(t) => out.push_str(&json_guillemets(t)),
        Valeur::Liste(l) => {
            let l = l.borrow().clone();
            if l.is_empty() {
                out.push_str("[]");
            } else {
                out.push('[');
                for (i, x) in l.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    nl(out, niv + 1);
                    json_ecrire(x, ind, niv + 1, out)?;
                }
                nl(out, niv);
                out.push(']');
            }
        }
        Valeur::Struct(d) => {
            let d = d.borrow().clone();
            let ks = d.cles_ordonnees();
            if ks.is_empty() {
                out.push_str("{}");
            } else {
                out.push('{');
                for (i, k) in ks.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    nl(out, niv + 1);
                    out.push_str(&json_guillemets(k));
                    out.push(':');
                    if !ind.is_empty() {
                        out.push(' ');
                    }
                    json_ecrire(d.get(k).unwrap(), ind, niv + 1, out)?;
                }
                nl(out, niv);
                out.push('}');
            }
        }
    }
    Ok(())
}

pub fn json_lire(s: &str) -> Result<Valeur, String> {
    let c: Vec<char> = s.chars().collect();
    let mut p = 0usize;
    let v = jv(&c, &mut p)?;
    jws(&c, &mut p);
    if p < c.len() {
        return Err(format!("Unexpected non-whitespace character after JSON at position {}", p));
    }
    Ok(v)
}

fn jws(c: &[char], p: &mut usize) {
    while *p < c.len() && matches!(c[*p], ' ' | '\t' | '\n' | '\r') {
        *p += 1;
    }
}

fn jv(c: &[char], p: &mut usize) -> Result<Valeur, String> {
    jws(c, p);
    if *p >= c.len() {
        return Err("Unexpected end of JSON input".into());
    }
    match c[*p] {
        '{' => {
            *p += 1;
            let mut d = Dict::nouveau();
            jws(c, p);
            if *p < c.len() && c[*p] == '}' {
                *p += 1;
                return Ok(Valeur::structure(d));
            }
            loop {
                jws(c, p);
                if *p >= c.len() || c[*p] != '"' {
                    return Err(format!("Expected double-quoted property name in JSON at position {}", p));
                }
                let k = jstr(c, p)?;
                jws(c, p);
                if *p >= c.len() || c[*p] != ':' {
                    return Err(format!("Expected ':' after property name in JSON at position {}", p));
                }
                *p += 1;
                let v = jv(c, p)?;
                d.set(&k, v);
                jws(c, p);
                match c.get(*p) {
                    Some(',') => *p += 1,
                    Some('}') => {
                        *p += 1;
                        return Ok(Valeur::structure(d));
                    }
                    _ => return Err(format!("Expected ',' or '}}' after property value in JSON at position {}", p)),
                }
            }
        }
        '[' => {
            *p += 1;
            let mut l = Vec::new();
            jws(c, p);
            if *p < c.len() && c[*p] == ']' {
                *p += 1;
                return Ok(Valeur::liste(l));
            }
            loop {
                l.push(jv(c, p)?);
                jws(c, p);
                match c.get(*p) {
                    Some(',') => *p += 1,
                    Some(']') => {
                        *p += 1;
                        return Ok(Valeur::liste(l));
                    }
                    _ => return Err(format!("Expected ',' or ']' after array element in JSON at position {}", p)),
                }
            }
        }
        '"' => Ok(Valeur::texte(jstr(c, p)?)),
        't' if c[*p..].starts_with(&['t', 'r', 'u', 'e']) => {
            *p += 4;
            Ok(Valeur::Bool(true))
        }
        'f' if c[*p..].starts_with(&['f', 'a', 'l', 's', 'e']) => {
            *p += 5;
            Ok(Valeur::Bool(false))
        }
        'n' if c[*p..].starts_with(&['n', 'u', 'l', 'l']) => {
            *p += 4;
            Ok(Valeur::Rien)
        }
        ch if ch == '-' || ch.is_ascii_digit() => {
            let d = *p;
            if c[*p] == '-' {
                *p += 1;
            }
            let ds = *p;
            while *p < c.len() && c[*p].is_ascii_digit() {
                *p += 1;
            }
            if *p == ds || (c[ds] == '0' && *p - ds > 1) {
                return Err(format!("Unexpected number in JSON at position {}", d));
            }
            if *p < c.len() && c[*p] == '.' {
                *p += 1;
                let fs = *p;
                while *p < c.len() && c[*p].is_ascii_digit() {
                    *p += 1;
                }
                if *p == fs {
                    return Err(format!("Unterminated fractional number in JSON at position {}", p));
                }
            }
            if *p < c.len() && (c[*p] == 'e' || c[*p] == 'E') {
                *p += 1;
                if *p < c.len() && (c[*p] == '+' || c[*p] == '-') {
                    *p += 1;
                }
                let es = *p;
                while *p < c.len() && c[*p].is_ascii_digit() {
                    *p += 1;
                }
                if *p == es {
                    return Err(format!("Exponent part is missing a number in JSON at position {}", p));
                }
            }
            let t: String = c[d..*p].iter().collect();
            Ok(Valeur::Nombre(t.parse().map_err(|_| "nombre invalide".to_string())?))
        }
        ch => Err(format!("Unexpected token '{}' in JSON at position {}", ch, p)),
    }
}

fn jstr(c: &[char], p: &mut usize) -> Result<String, String> {
    *p += 1;
    let mut s = String::new();
    loop {
        if *p >= c.len() {
            return Err(format!("Unterminated string in JSON at position {}", p));
        }
        let ch = c[*p];
        *p += 1;
        match ch {
            '"' => return Ok(s),
            '\\' => {
                if *p >= c.len() {
                    return Err(format!("Bad escaped character in JSON at position {}", p));
                }
                let e = c[*p];
                *p += 1;
                match e {
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    '/' => s.push('/'),
                    'b' => s.push('\u{8}'),
                    'f' => s.push('\u{c}'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    'u' => {
                        let lire4 = |p: &mut usize| -> Result<u32, String> {
                            if *p + 4 > c.len() {
                                return Err(format!("Bad Unicode escape in JSON at position {}", p));
                            }
                            let h: String = c[*p..*p + 4].iter().collect();
                            *p += 4;
                            u32::from_str_radix(&h, 16).map_err(|_| format!("Bad Unicode escape in JSON at position {}", p))
                        };
                        let a = lire4(p)?;
                        if (0xD800..0xDC00).contains(&a) && c.get(*p) == Some(&'\\') && c.get(*p + 1) == Some(&'u') {
                            *p += 2;
                            let b = lire4(p)?;
                            let cp = 0x10000 + ((a - 0xD800) << 10) + (b.wrapping_sub(0xDC00) & 0x3FF);
                            s.push(char::from_u32(cp).unwrap_or('\u{fffd}'));
                        } else {
                            s.push(char::from_u32(a).unwrap_or('\u{fffd}'));
                        }
                    }
                    _ => return Err(format!("Bad escaped character in JSON at position {}", *p - 1)),
                }
            }
            c0 if (c0 as u32) < 0x20 => return Err(format!("Bad control character in string literal in JSON at position {}", *p - 1)),
            c0 => s.push(c0),
        }
    }
}

// ---------------------------------------------------------------- Dates (UTC)
pub fn civil_depuis_jours(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub struct DateCiv {
    pub an: i64,
    pub mois: u32,
    pub jour: u32,
    pub h: u32,
    pub mi: u32,
    pub s: u32,
    pub ms: u32,
}

pub fn date_depuis_ms(ts: f64) -> Option<DateCiv> {
    if !ts.is_finite() || ts.abs() > 8.64e15 {
        return None;
    }
    let ts = ts.trunc() as i64;
    let jours = ts.div_euclid(86_400_000);
    let reste = ts.rem_euclid(86_400_000);
    let (an, mois, jour) = civil_depuis_jours(jours);
    Some(DateCiv {
        an,
        mois,
        jour,
        h: (reste / 3_600_000) as u32,
        mi: ((reste / 60_000) % 60) as u32,
        s: ((reste / 1000) % 60) as u32,
        ms: (reste % 1000) as u32,
    })
}

pub fn iso(d: &DateCiv) -> String {
    let an = if (0..=9999).contains(&d.an) {
        format!("{:04}", d.an)
    } else if d.an < 0 {
        format!("-{:06}", -d.an)
    } else {
        format!("+{:06}", d.an)
    };
    format!("{}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z", an, d.mois, d.jour, d.h, d.mi, d.s, d.ms)
}
