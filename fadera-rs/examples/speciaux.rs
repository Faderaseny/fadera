use fadera::mathfn as m;
fn main() {
    let v = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -0.0, 1.0, -1.0, 0.5, -0.5, 2.0, 5e-324, -5e-324, f64::MAX, f64::MIN_POSITIVE, 1e-200, 1e300, 710.0, -746.0, 1e9, 8e8];
    let same = |a: f64, b: f64| a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan());
    let mut bad = 0;
    macro_rules! u { ($n:expr, $f:expr, $g:expr) => { for &x in &v { let (a, b) = ($f(x), $g(x)); if !same(a, b) { let rel = ((a - b) / b).abs(); if !(rel < 1e-15) { bad += 1; println!("{} {:e}: mathfn={:e} libm={:e}", $n, x, a, b); } } } } }
    u!("sin", m::sin, libm::sin); u!("cos", m::cos, libm::cos); u!("tan", m::tan, libm::tan); u!("atan", m::atan, libm::atan);
    u!("asin", m::asin, libm::asin); u!("acos", m::acos, libm::acos); u!("exp", m::exp, libm::exp); u!("log", m::log, libm::log);
    u!("log2", m::log2, libm::log2); u!("log10", m::log10, libm::log10); u!("cbrt", m::cbrt, libm::cbrt);
    for &x in &v { for &y in &v { let (a, b) = (m::pow(x, y), libm::pow(x, y)); if !same(a, b) && !((a - b) / b).abs().lt(&1e-15) { bad += 1; println!("pow({:e},{:e}) mathfn={:e} libm={:e}", x, y, a, b); }
        let (a, b) = (m::atan2(x, y), libm::atan2(x, y)); if !same(a, b) && !((a - b) / b).abs().lt(&1e-15) { bad += 1; println!("atan2({:e},{:e}) mathfn={:e} libm={:e}", x, y, a, b); } } }
    println!("écarts significatifs : {}", bad);
}
