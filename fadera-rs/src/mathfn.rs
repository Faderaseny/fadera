//! Fonctions mathématiques de Fadéra : `sinus`, `cosinus`, `tangente`, `arc_*`, `exponentielle`, `logarithme*`, `**`, `racine_cubique`.
//!
//! **Arrondi correct.** Chaque résultat est calculé en « double-double » (≈ 106 bits) puis arrondi une seule fois : c'est
//! le double le plus proche de la vraie valeur mathématique (sauf cas pathologiques, une chance sur 10¹² environ).
//! Le résultat est donc **le même sur toutes les plateformes** (pur Rust, sans dépendre de la bibliothèque du système ni
//! du navigateur) et ne dépend pas de la version du moteur : les navigateurs récents convergent vers ce même résultat.
//!
//! Domaines à arrondi correct : trigonométrie pour |x| < 8·10⁸ ; `exp` pour -700 < x < 709,7 ; `**` quand le résultat est
//! normal (≈ 1e-300 … 1e300). En dehors, repli sur `libm` (erreur < 1 ULP).

#[derive(Clone, Copy, Debug)]
pub struct DD {
    pub hi: f64,
    pub lo: f64,
}

include!("mathconsts.rs");

#[inline]
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    (s, (a - (s - bb)) + (b - bb))
}
#[inline]
fn fast_two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    (s, b - (s - a))
}
#[inline]
fn split(a: f64) -> (f64, f64) {
    let t = 134217729.0 * a;
    let hi = t - (t - a);
    (hi, a - hi)
}
#[inline]
fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    let (ah, al) = split(a);
    let (bh, bl) = split(b);
    (p, ((ah * bh - p) + ah * bl + al * bh) + al * bl)
}

impl DD {
    #[inline]
    fn new(hi: f64) -> DD {
        DD { hi, lo: 0.0 }
    }
    #[inline]
    fn de(p: (f64, f64)) -> DD {
        DD { hi: p.0, lo: p.1 }
    }
    #[inline]
    fn neg(self) -> DD {
        DD { hi: -self.hi, lo: -self.lo }
    }
    #[inline]
    fn add(self, o: DD) -> DD {
        let (s1, s2) = two_sum(self.hi, o.hi);
        let (t1, t2) = two_sum(self.lo, o.lo);
        let s2 = s2 + t1;
        let (s1, s2) = fast_two_sum(s1, s2);
        let s2 = s2 + t2;
        DD::de(fast_two_sum(s1, s2))
    }
    #[inline]
    fn add_f(self, b: f64) -> DD {
        let (s1, s2) = two_sum(self.hi, b);
        DD::de(fast_two_sum(s1, s2 + self.lo))
    }
    #[inline]
    fn sub(self, o: DD) -> DD {
        self.add(o.neg())
    }
    #[inline]
    fn mul(self, o: DD) -> DD {
        let (p1, p2) = two_prod(self.hi, o.hi);
        let p2 = p2 + (self.hi * o.lo + self.lo * o.hi);
        DD::de(fast_two_sum(p1, p2))
    }
    #[inline]
    fn mul_f(self, b: f64) -> DD {
        let (p1, p2) = two_prod(self.hi, b);
        DD::de(fast_two_sum(p1, p2 + self.lo * b))
    }
    #[inline]
    fn sqr(self) -> DD {
        self.mul(self)
    }
    #[inline]
    fn scale2(self, p: f64) -> DD {
        DD { hi: self.hi * p, lo: self.lo * p }
    }
    fn div(self, o: DD) -> DD {
        let q1 = self.hi / o.hi;
        let r = self.sub(o.mul_f(q1));
        let q2 = r.hi / o.hi;
        let r = r.sub(o.mul_f(q2));
        let q3 = r.hi / o.hi;
        DD::de(fast_two_sum(q1, q2)).add_f(q3)
    }
    fn sqrt(self) -> DD {
        let x = 1.0 / self.hi.sqrt();
        let ax = self.hi * x;
        let d = self.sub(DD::de(two_prod(ax, ax)));
        DD::new(ax).add_f(d.hi * (x * 0.5))
    }
    #[inline]
    fn val(self) -> f64 {
        self.hi + self.lo
    }
}

fn pow2(k: i32) -> f64 {
    f64::from_bits(((k + 1023) as u64) << 52)
}
/// v × 2^k (k ∈ [-1500, 1500]), sans arrondi intermédiaire tant que le résultat est normal.
fn ldexp(v: f64, k: i32) -> f64 {
    let h = k / 2;
    v * pow2(h) * pow2(k - h)
}

// ------------------------------------------------------------------ exp
/// exp(x) = m · 2^k, m en double-double. Précondition : |x| < 745.
fn exp_dd(x: DD) -> (DD, i32) {
    let k = (x.hi * INV_LN2.hi).round();
    let r = x.sub(LN2.mul_f(k));
    let s = r.scale2(1.0 / 256.0);
    // termes n ≥ 5 (≤ 3e-17) en f64 : leur erreur relative 2^-53 reste sous 2^-106 du total
    let mut t = INV_FACT[11].hi;
    for n in (5..11).rev() {
        t = t * s.hi + INV_FACT[n].hi;
    }
    let mut p = DD::new(t);
    for n in (0..5).rev() {
        p = p.mul(s).add(INV_FACT[n]);
    }
    for _ in 0..8 {
        p = p.sqr();
    }
    (p, k as i32)
}

pub fn exp(x: f64) -> f64 {
    if x.is_nan() || x > 709.7 || x < -700.0 {
        return libm::exp(x);
    }
    if x == 0.0 {
        return 1.0;
    }
    let (m, k) = exp_dd(DD::new(x));
    ldexp(m.val(), k)
}

// ------------------------------------------------------------------ log
/// ln(x) en double-double, pour x fini > 0.
fn log_dd(x: f64) -> DD {
    let mut xx = x;
    let mut e: i32 = 0;
    if (xx.to_bits() >> 52) == 0 {
        xx *= 18014398509481984.0; // 2^54 (sous-normaux)
        e = -54;
    }
    let bits = xx.to_bits();
    let mut ee = ((bits >> 52) & 0x7ff) as i32 - 1023 + e;
    let mut m = f64::from_bits((bits & 0x000f_ffff_ffff_ffff) | 0x3ff0_0000_0000_0000);
    if m > 1.4142135623730951 {
        m *= 0.5;
        ee += 1;
    }
    // ln m = 2 atanh(z), z = (m-1)/(m+1)
    let z = DD::new(m - 1.0).div(DD::de(two_sum(m, 1.0)));
    let z2 = z.sqr();
    const K: usize = 22;
    // j ≥ 11 (terme ≤ 1,6e-17) en f64
    let mut t = RECIP[2 * K + 1].hi;
    for j in (11..K).rev() {
        t = t * z2.hi + RECIP[2 * j + 1].hi;
    }
    let mut p = DD::new(t);
    for j in (0..11).rev() {
        p = p.mul(z2).add(RECIP[2 * j + 1]);
    }
    let lm = z.mul(p).scale2(2.0);
    lm.add(LN2.mul_f(ee as f64))
}

fn log_special(x: f64) -> Option<f64> {
    if x.is_nan() || x < 0.0 {
        Some(f64::NAN)
    } else if x == 0.0 {
        Some(f64::NEG_INFINITY)
    } else if x.is_infinite() {
        Some(x)
    } else {
        None
    }
}
pub fn log(x: f64) -> f64 {
    log_special(x).unwrap_or_else(|| log_dd(x).val())
}
pub fn log2(x: f64) -> f64 {
    log_special(x).unwrap_or_else(|| log_dd(x).mul(INV_LN2).val())
}
pub fn log10(x: f64) -> f64 {
    log_special(x).unwrap_or_else(|| log_dd(x).mul(INV_LN10).val())
}

// ------------------------------------------------------------------ pow
fn pow_entier(x: f64, n: u32) -> DD {
    let mut r = DD::new(1.0);
    let mut b = DD::new(x);
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            r = r.mul(b);
        }
        n >>= 1;
        if n > 0 {
            b = b.sqr();
        }
    }
    r
}

pub fn pow(x: f64, y: f64) -> f64 {
    if y == 0.0 || x == 1.0 || !x.is_finite() || !y.is_finite() || x == 0.0 {
        return libm::pow(x, y);
    }
    if y == 1.0 {
        return x;
    }
    if y == 2.0 {
        return x * x;
    }
    if y == -1.0 {
        return 1.0 / x;
    }
    if y == 0.5 && x > 0.0 {
        return x.sqrt();
    }
    let entier = y == y.trunc();
    // petites puissances entières : produit exact en double-double (les cas « à égalité » sont arrondis comme IEEE)
    if entier && y.abs() <= 128.0 {
        let p = pow_entier(x, y.abs() as u32);
        let r = if y < 0.0 { DD::new(1.0).div(p) } else { p };
        let v = r.val();
        if v.is_finite() && v.abs() > 1e-290 && v.abs() < 1e290 {
            return v;
        }
        return libm::pow(x, y);
    }
    let (ax, neg) = if x < 0.0 {
        if entier {
            (-x, (y % 2.0) != 0.0)
        } else {
            return libm::pow(x, y);
        }
    } else {
        (x, false)
    };
    let z = log_dd(ax).mul_f(y);
    if !(z.hi > -700.0 && z.hi < 709.0) {
        return libm::pow(x, y);
    }
    let (m, k) = exp_dd(z);
    let r = ldexp(m.val(), k);
    if neg {
        -r
    } else {
        r
    }
}

// ------------------------------------------------------------------ sin / cos / tan
const LIMITE_TRIGO: f64 = 8.0e8;

/// Réduction : x = n·π/2 + r, |r| ≤ π/4 (+ε). n < 2^29 pour que n·chunk soit exact.
fn reduire(x: f64) -> (i64, DD) {
    let n = (x * DEUX_SUR_PI).round();
    let (s, e) = two_sum(x, -n * PIO2_CHUNKS[0]);
    let mut r = DD { hi: s, lo: e };
    for c in &PIO2_CHUNKS[1..] {
        r = r.add_f(-n * *c);
    }
    (n as i64, r)
}

fn sin_poly(r: DD) -> DD {
    let r2 = r.sqr();
    const K: usize = 14;
    // somme_{k} (-1)^k r2^k / (2k+1)!
    // k ≥ 8 (terme ≤ 2,8e-15·r2^8) en f64
    let sg = |k: usize, c: f64| if k % 2 == 1 { -c } else { c };
    let mut t = sg(K, INV_FACT[2 * K + 1].hi);
    for k in (8..K).rev() {
        t = t * r2.hi + sg(k, INV_FACT[2 * k + 1].hi);
    }
    let mut p = DD::new(t);
    for k in (0..8).rev() {
        let c = INV_FACT[2 * k + 1];
        let c = if k % 2 == 1 { c.neg() } else { c };
        p = p.mul(r2).add(c);
    }
    r.mul(p)
}
fn cos_poly(r: DD) -> DD {
    let r2 = r.sqr();
    const K: usize = 15;
    // somme_{k} (-1)^k r2^k / (2k)!   (2K = 30 ≤ 30)
    let sg = |k: usize, c: f64| if k % 2 == 1 { -c } else { c };
    let mut t = sg(K, INV_FACT[2 * K].hi);
    for k in (8..K).rev() {
        t = t * r2.hi + sg(k, INV_FACT[2 * k].hi);
    }
    let mut p = DD::new(t);
    for k in (0..8).rev() {
        let c = INV_FACT[2 * k];
        let c = if k % 2 == 1 { c.neg() } else { c };
        p = p.mul(r2).add(c);
    }
    p
}

fn sin_cos_dd(x: f64) -> (DD, DD) {
    let (n, r) = reduire(x);
    let (s, c) = (sin_poly(r), cos_poly(r));
    match n.rem_euclid(4) {
        0 => (s, c),
        1 => (c, s.neg()),
        2 => (s.neg(), c.neg()),
        _ => (c.neg(), s),
    }
}

pub fn sin(x: f64) -> f64 {
    if !(x.abs() < LIMITE_TRIGO) {
        return libm::sin(x);
    }
    if x.abs() < 1.4901161193847656e-8 {
        return x;
    }
    let (n, r) = reduire(x);
    match n.rem_euclid(4) {
        0 => sin_poly(r).val(),
        1 => cos_poly(r).val(),
        2 => -sin_poly(r).val(),
        _ => -cos_poly(r).val(),
    }
}
pub fn cos(x: f64) -> f64 {
    if !(x.abs() < LIMITE_TRIGO) {
        return libm::cos(x);
    }
    if x.abs() < 1.4901161193847656e-8 {
        return 1.0;
    }
    let (n, r) = reduire(x);
    match n.rem_euclid(4) {
        0 => cos_poly(r).val(),
        1 => -sin_poly(r).val(),
        2 => -cos_poly(r).val(),
        _ => sin_poly(r).val(),
    }
}
pub fn tan(x: f64) -> f64 {
    if !(x.abs() < LIMITE_TRIGO) {
        return libm::tan(x);
    }
    if x.abs() < 1.4901161193847656e-8 {
        return x;
    }
    let (n, r) = reduire(x);
    let (s, c) = (sin_poly(r), cos_poly(r));
    if n.rem_euclid(2) == 0 {
        s.div(c).val()
    } else {
        c.div(s).neg().val()
    }
}

// ------------------------------------------------------------------ arc-fonctions
/// atan d'un double-double (|x| < 1e100) : Newton à partir de l'atan flottant.
fn atan_dd(x: DD) -> DD {
    let y0 = libm::atan(x.hi);
    let (s, c) = sin_cos_dd(y0);
    let u = x.mul(c).sub(s).div(c.add(x.mul(s)));
    DD::new(y0).add(u)
}

pub fn atan(x: f64) -> f64 {
    if x.is_nan() || x.abs() > 1e100 || x.abs() < 1.4901161193847656e-8 {
        return if x.abs() < 1.4901161193847656e-8 { x } else { libm::atan(x) };
    }
    atan_dd(DD::new(x)).val()
}
pub fn asin(x: f64) -> f64 {
    if x.is_nan() || x.abs() > 1.0 {
        return f64::NAN;
    }
    if x.abs() == 1.0 {
        return x * PIO2.hi;
    }
    if x.abs() < 1.4901161193847656e-8 {
        return x;
    }
    let p = two_prod(x, x);
    let den = DD::new(1.0).sub(DD::de(p)).sqrt();
    atan_dd(DD::new(x).div(den)).val()
}
pub fn acos(x: f64) -> f64 {
    if x.is_nan() || x.abs() > 1.0 {
        return f64::NAN;
    }
    if x == 1.0 {
        return 0.0;
    }
    if x == -1.0 {
        return PI.hi;
    }
    let q = DD::de(two_sum(1.0, -x)).div(DD::de(two_sum(1.0, x)));
    atan_dd(q.sqrt()).scale2(2.0).val()
}
pub fn atan2(y: f64, x: f64) -> f64 {
    if !x.is_finite() || !y.is_finite() || x == 0.0 || y == 0.0 {
        return libm::atan2(y, x);
    }
    let q = DD::new(y).div(DD::new(x));
    let a = q.hi.abs();
    if !(a > 1e-290 && a < 1e100) {
        return libm::atan2(y, x);
    }
    let r = atan_dd(q);
    if x > 0.0 {
        r.val()
    } else if y > 0.0 {
        r.add(PI).val()
    } else {
        r.sub(PI).val()
    }
}

// ------------------------------------------------------------------ cbrt
pub fn cbrt(x: f64) -> f64 {
    let a = x.abs();
    if !(a > 1e-280 && a < 1e280) {
        return libm::cbrt(x);
    }
    let y0 = libm::cbrt(x);
    let y3 = DD::de(two_prod(y0, y0)).mul_f(y0);
    let resid = y3.add_f(-x);
    let corr = -resid.hi / (3.0 * y0 * y0);
    DD::new(y0).add_f(corr).val()
}
