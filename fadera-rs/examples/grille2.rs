// Imprime « fonction bits_arg(s) bits_mathfn bits_libm » sur une grille (vérifié contre mpmath par scratchpad/oracle.py)
use fadera::mathfn as m;
fn main() {
    let mut s = 0x9E3779B97F4A7C15u64;
    let mut r = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; (s >> 11) as f64 / (1u64 << 53) as f64 };
    let mut xs: Vec<f64> = Vec::new();
    for i in -1000..=1000 { xs.push(i as f64 * 0.02); }
    for _ in 0..3000 { xs.push((r() - 0.5) * 200.0); }
    for _ in 0..1500 { xs.push((r() - 0.5) * 2.0); }
    for _ in 0..1500 { xs.push((r() - 0.5) * 2e6); }
    for _ in 0..800 { xs.push(r() * 1e8); }
    for _ in 0..800 { xs.push(r() * 1e-3); }
    for k in 1..400 { let c = k as f64 * std::f64::consts::FRAC_PI_2; xs.push(c); xs.push(c + 1e-9); xs.push(f64::from_bits(c.to_bits() + 1)); xs.push(f64::from_bits(c.to_bits() - 1)); }
    for k in [1.0f64, 2.0, 10.0, 1e5, 1e10, 3.0, 0.5] { for d in 0..60 { xs.push(f64::from_bits(k.to_bits() + d)); xs.push(f64::from_bits(k.to_bits() - d)); } }
    let pos = |x: f64| x.abs() + 1e-300;
    let un = |x: f64| (x * 0.999999999).max(-1.0).min(1.0);
    let mut out = Vec::new();
    macro_rules! f { ($n:expr, $arg:expr, $m:expr, $l:expr) => { for &x in &xs { let a: f64 = $arg(x); if a.is_nan() { continue; } out.push(format!("{} {:x} {:x} {:x}", $n, a.to_bits(), ($m)(a).to_bits(), ($l)(a).to_bits())); } } }
    f!("sin", |x| x, m::sin, libm::sin); f!("cos", |x| x, m::cos, libm::cos); f!("tan", |x| x, m::tan, libm::tan);
    f!("atan", |x| x, m::atan, libm::atan); f!("asin", un, m::asin, libm::asin); f!("acos", un, m::acos, libm::acos);
    f!("exp", |x: f64| x.max(-700.0).min(709.0), m::exp, libm::exp);
    f!("log", pos, m::log, libm::log); f!("log2", pos, m::log2, libm::log2); f!("log10", pos, m::log10, libm::log10);
    f!("cbrt", |x| x, m::cbrt, libm::cbrt);
    f!("pow_3_7", |x: f64| x.abs(), |a| m::pow(a, 3.7), |a| libm::pow(a, 3.7));
    f!("pow_x_1_5", |x: f64| x.abs(), |a| m::pow(a, 1.5), |a| libm::pow(a, 1.5));
    f!("pow_2_x", |x: f64| x.max(-1000.0).min(1000.0), |a| m::pow(2.0, a), |a| libm::pow(2.0, a));
    f!("pow_10_x", |x: f64| x.max(-300.0).min(300.0), |a| m::pow(10.0, a), |a| libm::pow(10.0, a));
    f!("pow_1_1_x", |x: f64| x.max(-900.0).min(900.0), |a| m::pow(1.1, a), |a| libm::pow(1.1, a));
    f!("pow_x_3", |x: f64| x, |a| m::pow(a, 3.0), |a| libm::pow(a, 3.0));
    f!("pow_x_m5", |x: f64| x, |a| m::pow(a, -5.0), |a| libm::pow(a, -5.0));
    for i in 0..xs.len() { let (y, x) = (xs[i], xs[(i * 7 + 3) % xs.len()]); if x == 0.0 { continue; } out.push(format!("atan2 {:x}_{:x} {:x} {:x}", y.to_bits(), x.to_bits(), m::atan2(y, x).to_bits(), libm::atan2(y, x).to_bits())); }
    println!("{}", out.join("\n"));
}
