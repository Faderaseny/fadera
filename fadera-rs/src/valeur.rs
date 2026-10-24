//! Valeurs Fadéra et utilitaires reproduisant la sémantique JavaScript du moteur de référence
//! (formatage des nombres, conversions, ordre des clés, égalité, etc.).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


pub type Scope = Rc<ScopeData>;

/// Variables d'une portée, indexées par identifiant de nom interné (aucun hachage, aucune allocation de texte).
/// Portée de fonction : petit tableau parcouru linéairement. Portée globale : tableau dense indexé par l'identifiant.
pub enum Vars {
    Petit(Vec<(u32, Valeur)>),
    Dense(Vec<Option<Valeur>>),
}

impl Vars {
    #[inline]
    pub fn get(&self, id: u32) -> Option<&Valeur> {
        match self {
            Vars::Petit(v) => v.iter().find(|(k, _)| *k == id).map(|(_, x)| x),
            Vars::Dense(v) => v.get(id as usize).and_then(|x| x.as_ref()),
        }
    }
    #[inline]
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Valeur> {
        match self {
            Vars::Petit(v) => v.iter_mut().find(|(k, _)| *k == id).map(|(_, x)| x),
            Vars::Dense(v) => v.get_mut(id as usize).and_then(|x| x.as_mut()),
        }
    }
    #[inline]
    pub fn definir(&mut self, id: u32, val: Valeur) {
        match self {
            Vars::Petit(v) => {
                if let Some(e) = v.iter_mut().find(|(k, _)| *k == id) {
                    e.1 = val;
                } else {
                    v.push((id, val));
                }
            }
            Vars::Dense(v) => {
                let i = id as usize;
                if i >= v.len() {
                    v.resize(i + 1, None);
                }
                v[i] = Some(val);
            }
        }
    }
}

pub struct ScopeData {
    pub vars: RefCell<Vars>,
    pub parent: Option<Scope>,
}

impl ScopeData {
    pub fn nouveau(parent: Option<Scope>) -> Scope {
        Rc::new(ScopeData { vars: RefCell::new(Vars::Petit(Vec::new())), parent })
    }
    pub fn avec_capacite(parent: Option<Scope>, n: usize) -> Scope {
        Rc::new(ScopeData { vars: RefCell::new(Vars::Petit(Vec::with_capacity(n.max(4)))), parent })
    }
    pub fn globale() -> Scope {
        Rc::new(ScopeData { vars: RefCell::new(Vars::Dense(Vec::new())), parent: None })
    }
}

/// Hachage rapide (FNV-1a) pour l'index des grandes structures (le hachage par défaut est lent, surtout en WebAssembly).
#[derive(Clone, Copy)]
pub struct Fnv(u64);
impl Default for Fnv {
    fn default() -> Self {
        Fnv(0xcbf29ce484222325)
    }
}
impl std::hash::Hasher for Fnv {
    fn write(&mut self, octets: &[u8]) {
        for b in octets {
            self.0 ^= *b as u64;
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
    }
    fn finish(&self) -> u64 {
        self.0
    }
}
type Index = HashMap<String, usize, std::hash::BuildHasherDefault<Fnv>>;

/// Au-delà de ce nombre de champs, on construit un index de hachage (en dessous, la recherche linéaire est plus rapide).
const SEUIL_INDEX: usize = 8;

/// Structure Fadéra : table ordonnée (ordre des clés identique à JavaScript).
/// Une seule liste (clé, valeur) dans l'ordre d'insertion ; petit : recherche linéaire, grand : index.
#[derive(Default, Clone)]
pub struct Dict {
    entrees: Vec<(String, Valeur)>,
    index: Option<Index>,
    /// Classe de l'instance (étiquette cachée : invisible pour les clés, la taille, le JSON…). None = structure ordinaire.
    pub classe: Option<Rc<str>>,
}

impl Dict {
    pub fn nouveau() -> Dict {
        Dict::default()
    }
    fn position(&self, k: &str) -> Option<usize> {
        match &self.index {
            Some(ix) => ix.get(k).copied(),
            None => self.entrees.iter().position(|(c, _)| c == k),
        }
    }
    fn reconstruire_index(&mut self) {
        if self.entrees.len() > SEUIL_INDEX {
            let mut ix = Index::default();
            for (i, (k, _)) in self.entrees.iter().enumerate() {
                ix.insert(k.clone(), i);
            }
            self.index = Some(ix);
        } else {
            self.index = None;
        }
    }
    pub fn get(&self, k: &str) -> Option<&Valeur> {
        self.position(k).map(|i| &self.entrees[i].1)
    }
    pub fn contient(&self, k: &str) -> bool {
        self.position(k).is_some()
    }
    pub fn set(&mut self, k: &str, v: Valeur) {
        if let Some(i) = self.position(k) {
            self.entrees[i].1 = v;
            return;
        }
        self.entrees.push((k.to_string(), v));
        let n = self.entrees.len();
        match &mut self.index {
            Some(ix) => {
                ix.insert(k.to_string(), n - 1);
            }
            None => {
                if n > SEUIL_INDEX {
                    self.reconstruire_index();
                }
            }
        }
    }
    pub fn supprimer(&mut self, k: &str) -> Option<Valeur> {
        let i = self.position(k)?;
        let (_, v) = self.entrees.remove(i);
        self.reconstruire_index();
        Some(v)
    }
    pub fn len(&self) -> usize {
        self.entrees.len()
    }
    /// Clés dans l'ordre d'insertion.
    pub fn cles(&self) -> impl Iterator<Item = &String> {
        self.entrees.iter().map(|(k, _)| k)
    }
    /// Clés dans l'ordre JavaScript : indices entiers croissants d'abord, puis ordre d'insertion.
    pub fn cles_ordonnees(&self) -> Vec<String> {
        let cles: Vec<String> = self.cles().cloned().collect();
        ordonner_cles(&cles)
    }
}

pub fn est_indice(k: &str) -> Option<u64> {
    if k.is_empty() || k.len() > 10 {
        return None;
    }
    if k != "0" && k.starts_with('0') {
        return None;
    }
    if !k.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: u64 = k.parse().ok()?;
    if n < 4294967295 {
        Some(n)
    } else {
        None
    }
}

pub fn ordonner_cles(cles: &[String]) -> Vec<String> {
    let mut idx: Vec<(u64, &String)> = Vec::new();
    let mut autres: Vec<&String> = Vec::new();
    for k in cles {
        match est_indice(k) {
            Some(n) => idx.push((n, k)),
            None => autres.push(k),
        }
    }
    idx.sort_by_key(|x| x.0);
    idx.into_iter().map(|x| x.1.clone()).chain(autres.into_iter().cloned()).collect()
}

pub struct FonctionFadera {
    pub parametres: Rc<Vec<crate::ast::Nom>>,
    pub corps: Rc<crate::ast::Corps>,
    pub fermeture: Option<Scope>,
    pub nom: Option<String>,
    /// Nom de la fonction native si c'est une fonction native passée comme valeur.
    pub natif: Option<String>,
}

pub struct Reseau {
    pub n_entrees: usize,
    pub n_cache: usize,
    pub n_sorties: usize,
    pub w1: Vec<Vec<f64>>,
    pub b1: Vec<f64>,
    pub w2: Vec<Vec<f64>>,
    pub b2: Vec<f64>,
}

#[derive(Clone)]
pub enum Valeur {
    Rien,
    Nombre(f64),
    /// Texte : `Rc<String>` (et non `Rc<str>`) pour pouvoir ajouter à la fin en place (`s = s + …` en O(1) amorti).
    Texte(Rc<String>),
    Bool(bool),
    Liste(Rc<RefCell<Vec<Valeur>>>),
    Struct(Rc<RefCell<Dict>>),
    Fonction(Rc<FonctionFadera>),
    Reseau(Rc<RefCell<Reseau>>),
}

impl Valeur {
    pub fn texte(s: impl AsRef<str>) -> Valeur {
        Valeur::Texte(Rc::new(s.as_ref().to_string()))
    }
    /// Texte à partir d'un `String` déjà construit (sans le recopier).
    pub fn texte_string(s: String) -> Valeur {
        Valeur::Texte(Rc::new(s))
    }
    pub fn liste(v: Vec<Valeur>) -> Valeur {
        Valeur::Liste(Rc::new(RefCell::new(v)))
    }
    pub fn structure(d: Dict) -> Valeur {
        Valeur::Struct(Rc::new(RefCell::new(d)))
    }
    pub fn struct_de(paires: Vec<(&str, Valeur)>) -> Valeur {
        let mut d = Dict::nouveau();
        for (k, v) in paires {
            d.set(k, v);
        }
        Valeur::structure(d)
    }
    pub fn nombres(v: &[f64]) -> Valeur {
        Valeur::liste(v.iter().map(|x| Valeur::Nombre(*x)).collect())
    }
    pub fn est_structure(&self) -> bool {
        matches!(self, Valeur::Struct(_))
    }
    pub fn est_liste(&self) -> bool {
        matches!(self, Valeur::Liste(_))
    }
    /// Vérité à la JavaScript.
    pub fn vrai(&self) -> bool {
        match self {
            Valeur::Rien => false,
            Valeur::Nombre(n) => *n != 0.0 && !n.is_nan(),
            Valeur::Texte(t) => !t.is_empty(),
            Valeur::Bool(b) => *b,
            _ => true,
        }
    }
}

pub fn type_de(v: &Valeur) -> &'static str {
    match v {
        Valeur::Rien => "rien",
        Valeur::Fonction(_) => "fonction",
        Valeur::Liste(_) => "liste",
        Valeur::Nombre(_) => "nombre",
        Valeur::Texte(_) => "texte",
        Valeur::Bool(_) => "booléen",
        Valeur::Reseau(_) => "réseau",
        Valeur::Struct(_) => "structure",
    }
}

// ---------------------------------------------------------------------------
// Nombres à la JavaScript
// ---------------------------------------------------------------------------

/// Équivalent de `String(nombre)` en JavaScript.
pub fn js_num(v: f64) -> String {
    if v.is_nan() {
        return "NaN".into();
    }
    if v.is_infinite() {
        return if v > 0.0 { "Infinity".into() } else { "-Infinity".into() };
    }
    if v == 0.0 {
        return "0".into();
    }
    let neg = v < 0.0;
    let a = v.abs();
    // Chiffres significatifs les plus courts qui redonnent a (format {:e} de Rust).
    let s = format!("{:e}", a);
    let (mant, exp) = s.split_once('e').unwrap();
    let e: i32 = exp.parse().unwrap();
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let k = digits.len() as i32;
    let n = e + 1; // position de la virgule
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    if k <= n && n <= 21 {
        out.push_str(&digits);
        for _ in 0..(n - k) {
            out.push('0');
        }
    } else if 0 < n && n <= 21 {
        out.push_str(&digits[..n as usize]);
        out.push('.');
        out.push_str(&digits[n as usize..]);
    } else if -6 < n && n <= 0 {
        out.push_str("0.");
        for _ in 0..(-n) {
            out.push('0');
        }
        out.push_str(&digits);
    } else {
        let ex = n - 1;
        out.push_str(&digits[..1]);
        if k > 1 {
            out.push('.');
            out.push_str(&digits[1..]);
        }
        out.push('e');
        out.push(if ex >= 0 { '+' } else { '-' });
        out.push_str(&ex.abs().to_string());
    }
    out
}

pub fn est_entier(v: f64) -> bool {
    v.is_finite() && v.fract() == 0.0
}

/// `Math.round` de JavaScript (arrondi vers +∞ pour les moitiés).
pub fn js_round(x: f64) -> f64 {
    if !x.is_finite() {
        return x;
    }
    let f = x.floor();
    if x - f >= 0.5 {
        f + 1.0
    } else {
        f
    }
}

pub fn js_trunc(x: f64) -> f64 {
    x.trunc()
}

fn est_ws_js(c: char) -> bool {
    c.is_whitespace() || c == '\u{feff}'
}

pub fn js_trim(s: &str) -> &str {
    s.trim_matches(est_ws_js)
}

/// `Number(chaîne)`
pub fn js_nombre_texte(s: &str) -> f64 {
    let t = js_trim(s);
    if t.is_empty() {
        return 0.0;
    }
    match t {
        "Infinity" | "+Infinity" => return f64::INFINITY,
        "-Infinity" => return f64::NEG_INFINITY,
        _ => {}
    }
    let b = t.as_bytes();
    if b.len() > 2 && b[0] == b'0' {
        let radix = match b[1] {
            b'x' | b'X' => 16,
            b'o' | b'O' => 8,
            b'b' | b'B' => 2,
            _ => 0,
        };
        if radix != 0 {
            return match u128::from_str_radix(&t[2..], radix) {
                Ok(n) => n as f64,
                Err(_) => f64::NAN,
            };
        }
    }
    if !t.bytes().all(|c| c.is_ascii_digit() || matches!(c, b'+' | b'-' | b'.' | b'e' | b'E')) {
        return f64::NAN;
    }
    t.parse::<f64>().unwrap_or(f64::NAN)
}

/// `Number(valeur)`
pub fn js_nombre(v: &Valeur) -> f64 {
    match v {
        Valeur::Rien => 0.0,
        Valeur::Bool(b) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        Valeur::Nombre(n) => *n,
        Valeur::Texte(t) => js_nombre_texte(t),
        Valeur::Liste(l) => {
            let l = l.borrow();
            match l.len() {
                0 => 0.0,
                1 => js_nombre(&Valeur::texte(js_texte(&l[0]))),
                _ => f64::NAN,
            }
        }
        _ => f64::NAN,
    }
}

/// `parseInt(String(v), 10)`
pub fn js_parse_int(v: &Valeur) -> f64 {
    let s = js_texte(v);
    let t = js_trim(&s);
    let mut chars = t.chars().peekable();
    let mut neg = false;
    if let Some(&c) = chars.peek() {
        if c == '-' || c == '+' {
            neg = c == '-';
            chars.next();
        }
    }
    let rest: String = chars.collect();
    let (radix, digits) = if rest.starts_with("0x") || rest.starts_with("0X") {
        (16, &rest[2..])
    } else {
        (10, &rest[..])
    };
    let d: String = digits.chars().take_while(|c| c.is_digit(radix)).collect();
    if d.is_empty() {
        return f64::NAN;
    }
    let mut n = 0f64;
    for c in d.chars() {
        n = n * radix as f64 + c.to_digit(radix).unwrap() as f64;
    }
    if neg {
        -n
    } else {
        n
    }
}

/// `parseFloat(String(v))`
pub fn js_parse_float(v: &Valeur) -> f64 {
    let s = js_texte(v);
    let t = js_trim(&s);
    let b = t.as_bytes();
    let mut i = 0;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }
    if t[i..].starts_with("Infinity") {
        return if b.first() == Some(&b'-') { f64::NEG_INFINITY } else { f64::INFINITY };
    }
    let ds = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    let mut nd = i - ds;
    if i < b.len() && b[i] == b'.' {
        i += 1;
        let fs = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        nd += i - fs;
    }
    if nd == 0 {
        return f64::NAN;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < b.len() && (b[j] == b'+' || b[j] == b'-') {
            j += 1;
        }
        let es = j;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
        }
        if j > es {
            i = j;
        }
    }
    let mut sl = &t[..i];
    if sl.ends_with('.') {
        sl = &sl[..sl.len() - 1];
    }
    sl.parse::<f64>().unwrap_or(f64::NAN)
}

/// `String(valeur)` de JavaScript.
pub fn js_texte(v: &Valeur) -> String {
    match v {
        Valeur::Rien => "null".into(),
        Valeur::Bool(b) => b.to_string(),
        Valeur::Nombre(n) => js_num(*n),
        Valeur::Texte(t) => t.to_string(),
        Valeur::Liste(l) => l
            .borrow()
            .iter()
            .map(|x| if matches!(x, Valeur::Rien) { String::new() } else { js_texte(x) })
            .collect::<Vec<_>>()
            .join(","),
        Valeur::Struct(_) => "[object Object]".into(),
        Valeur::Fonction(_) => "function".into(),
        Valeur::Reseau(_) => "[object Object]".into(),
    }
}

// ---------------------------------------------------------------------------
// Formatage Fadéra
// ---------------------------------------------------------------------------

pub fn json_guillemets(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            '\u{8}' => o.push_str("\\b"),
            '\u{c}' => o.push_str("\\f"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

pub fn formater_nombre(v: f64) -> String {
    if est_entier(v) || !v.is_finite() {
        js_num(v)
    } else {
        // parseFloat(v.toPrecision(12))
        let s = format!("{:.11e}", v);
        let r: f64 = s.parse().unwrap_or(v);
        js_num(r)
    }
}

pub fn formater(v: &Valeur) -> String {
    formater_rec(v, &mut Vec::new())
}

/// `en_cours` : adresses des listes/structures en cours d'affichage ; une structure qui se contient elle-même s'affiche « [...] » / « {...} ».
fn formater_rec(v: &Valeur, en_cours: &mut Vec<usize>) -> String {
    match v {
        Valeur::Rien => "rien".into(),
        Valeur::Bool(b) => if *b { "vrai".into() } else { "faux".into() },
        Valeur::Nombre(n) => formater_nombre(*n),
        Valeur::Texte(t) => t.to_string(),
        Valeur::Fonction(f) => format!("<fonction {}>", f.nom.clone().unwrap_or_else(|| "anonyme".into())),
        Valeur::Reseau(r) => {
            let r = r.borrow();
            format!("[Réseau {}-{}-{}]", r.n_entrees, r.n_cache, r.n_sorties)
        }
        Valeur::Liste(l) => {
            let id = Rc::as_ptr(l) as *const u8 as usize;
            if en_cours.contains(&id) {
                return "[...]".into();
            }
            en_cours.push(id);
            let r = format!("[{}]", l.borrow().iter().map(|x| formater_interne(x, en_cours)).collect::<Vec<_>>().join(", "));
            en_cours.pop();
            r
        }
        Valeur::Struct(d) => {
            let id = Rc::as_ptr(d) as *const u8 as usize;
            if en_cours.contains(&id) {
                return "{...}".into();
            }
            en_cours.push(id);
            let r = {
                let d = d.borrow();
                let parts: Vec<String> = d
                    .cles_ordonnees()
                    .iter()
                    .map(|k| format!("{}: {}", k, formater_interne(d.get(k).unwrap(), en_cours)))
                    .collect();
                match &d.classe {
                    Some(c) => format!("{} {{ {} }}", c, parts.join(", ")),
                    None => format!("{{ {} }}", parts.join(", ")),
                }
            };
            en_cours.pop();
            r
        }
    }
}

fn formater_interne(v: &Valeur, en_cours: &mut Vec<usize>) -> String {
    match v {
        Valeur::Texte(t) => json_guillemets(t),
        _ => formater_rec(v, en_cours),
    }
}

/// Égalité profonde. Deux références identiques sont égales ; une paire déjà en cours de comparaison est supposée égale (structures cycliques).
pub fn egal(a: &Valeur, b: &Valeur) -> bool {
    match (a, b) {
        (Valeur::Rien, Valeur::Rien) => true,
        (Valeur::Nombre(x), Valeur::Nombre(y)) => x == y,
        (Valeur::Texte(x), Valeur::Texte(y)) => x == y,
        (Valeur::Bool(x), Valeur::Bool(y)) => x == y,
        (Valeur::Liste(_), Valeur::Liste(_)) | (Valeur::Struct(_), Valeur::Struct(_)) => egal_rec(a, b, &mut std::collections::HashSet::new()),
        (Valeur::Fonction(x), Valeur::Fonction(y)) => Rc::ptr_eq(x, y),
        (Valeur::Reseau(x), Valeur::Reseau(y)) => Rc::ptr_eq(x, y),
        _ => false,
    }
}

fn egal_rec(a: &Valeur, b: &Valeur, vus: &mut std::collections::HashSet<(usize, usize)>) -> bool {
    match (a, b) {
        (Valeur::Liste(x), Valeur::Liste(y)) => {
            if Rc::ptr_eq(x, y) {
                return true;
            }
            if !vus.insert((Rc::as_ptr(x) as *const u8 as usize, Rc::as_ptr(y) as *const u8 as usize)) {
                return true;
            }
            let (x, y) = (x.borrow(), y.borrow());
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(p, q)| egal_rec(p, q, vus))
        }
        (Valeur::Struct(x), Valeur::Struct(y)) => {
            if Rc::ptr_eq(x, y) {
                return true;
            }
            if !vus.insert((Rc::as_ptr(x) as *const u8 as usize, Rc::as_ptr(y) as *const u8 as usize)) {
                return true;
            }
            let (x, y) = (x.borrow(), y.borrow());
            x.classe == y.classe
                && x.len() == y.len()
                && x.cles().all(|k| match (x.get(k), y.get(k)) {
                    (Some(p), Some(q)) => egal_rec(p, q, vus),
                    _ => false,
                })
        }
        _ => egal(a, b),
    }
}

/// SameValueZero (utilisé par `Set` en JavaScript).
pub fn meme_valeur_zero(a: &Valeur, b: &Valeur) -> bool {
    match (a, b) {
        (Valeur::Nombre(x), Valeur::Nombre(y)) => (x.is_nan() && y.is_nan()) || x == y,
        (Valeur::Liste(x), Valeur::Liste(y)) => Rc::ptr_eq(x, y),
        (Valeur::Struct(x), Valeur::Struct(y)) => Rc::ptr_eq(x, y),
        _ => egal(a, b),
    }
}

/// Comparaison de textes par unités UTF-16 (ordre de `<` en JavaScript).
pub fn cmp_utf16(a: &str, b: &str) -> std::cmp::Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

fn plier(c: char) -> (char, u8) {
    // (lettre de base minuscule, rang d'accent)
    let base = match c {
        'à' | 'â' | 'ä' | 'á' | 'ã' | 'å' => ('a', 1),
        'À' | 'Â' | 'Ä' | 'Á' | 'Ã' | 'Å' => ('a', 1),
        'é' | 'è' | 'ê' | 'ë' => ('e', 1),
        'É' | 'È' | 'Ê' | 'Ë' => ('e', 1),
        'î' | 'ï' | 'í' | 'ì' | 'Î' | 'Ï' | 'Í' | 'Ì' => ('i', 1),
        'ô' | 'ö' | 'ó' | 'ò' | 'õ' | 'Ô' | 'Ö' | 'Ó' | 'Ò' | 'Õ' => ('o', 1),
        'ù' | 'û' | 'ü' | 'ú' | 'Ù' | 'Û' | 'Ü' | 'Ú' => ('u', 1),
        'ç' | 'Ç' => ('c', 1),
        'ÿ' | 'ý' | 'Ý' => ('y', 1),
        'ñ' | 'Ñ' => ('n', 1),
        _ => {
            let l = c.to_lowercase().next().unwrap_or(c);
            (l, 0)
        }
    };
    if base.1 == 1 {
        // l'accent précis (pour départager)
        let acc = match c.to_lowercase().next().unwrap_or(c) {
            'é' | 'á' | 'à' | 'í' | 'ì' | 'ó' | 'ò' | 'ú' | 'ù' | 'ý' => 1,
            'è' | 'â' => 2,
            'ê' | 'ä' | 'î' | 'ô' | 'û' => 3,
            _ => 4,
        };
        (base.0, acc)
    } else {
        (base.0, 0)
    }
}

/// Approximation de `localeCompare(…, "fr")` : insensible à la casse et aux accents d'abord,
/// puis accents, puis minuscule avant majuscule.
pub fn locale_compare(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    let ca: Vec<(char, u8)> = a.chars().map(plier).collect();
    let cb: Vec<(char, u8)> = b.chars().map(plier).collect();
    // Niveau 1 : lettres de base (ponctuation/espaces d'abord, puis chiffres, puis lettres)
    let rang = |c: char| -> (u8, char) {
        if c.is_alphabetic() {
            (2, c)
        } else if c.is_numeric() {
            (1, c)
        } else {
            (0, c)
        }
    };
    for (x, y) in ca.iter().zip(cb.iter()) {
        let o = rang(x.0).cmp(&rang(y.0));
        if o != Equal {
            return o;
        }
    }
    if ca.len() != cb.len() {
        return ca.len().cmp(&cb.len());
    }
    // Niveau 2 : accents
    for (x, y) in ca.iter().zip(cb.iter()) {
        let o = x.1.cmp(&y.1);
        if o != Equal {
            return o;
        }
    }
    // Niveau 3 : casse (minuscule avant majuscule)
    for (x, y) in a.chars().zip(b.chars()) {
        if x != y {
            let (xm, ym) = (x.is_uppercase(), y.is_uppercase());
            if xm != ym {
                return if xm { Greater } else { Less };
            }
        }
    }
    Equal
}
