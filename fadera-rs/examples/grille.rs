// Imprime les bits des résultats libm sur une grille fixe (comparés à Math.* par scratchpad/grille.js)
fn main() {
    let mut xs: Vec<f64> = Vec::new();
    let mut s = 0x9E3779B97F4A7C15u64;
    let mut r = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; (s >> 11) as f64 / (1u64 << 53) as f64 };
    for i in -2000..=2000 { xs.push(i as f64 * 0.01); }            // -20 .. 20 pas 0.01
    for _ in 0..4000 { xs.push((r() - 0.5) * 200.0); }              // -100 .. 100
    for _ in 0..2000 { xs.push((r() - 0.5) * 2.0); }                // -1 .. 1
    for _ in 0..1000 { xs.push(r() * 1e6); }                        // grands arguments
    for _ in 0..1000 { xs.push(r() * 1e-3); }                       // petits
    let f = |n: &str, g: &dyn Fn(f64) -> f64| { for x in &xs { println!("{} {:x} {:x}", n, x.to_bits(), g(*x).to_bits()); } };
    f("sin", &|x| libm::sin(x)); f("cos", &|x| libm::cos(x)); f("tan", &|x| libm::tan(x));
    f("asin", &|x| libm::asin(x)); f("acos", &|x| libm::acos(x)); f("atan", &|x| libm::atan(x));
    f("exp", &|x| libm::exp(x)); f("log", &|x| libm::log(x)); f("log2", &|x| libm::log2(x)); f("log10", &|x| libm::log10(x));
    f("pow3", &|x| libm::pow(x, 3.0)); f("pow05", &|x| libm::pow(x.abs(), 0.5)); f("pow1_7", &|x| libm::pow(x.abs(), 1.7)); f("pow_x", &|x| libm::pow(2.0, x)); f("pow10", &|x| libm::pow(10.0, x));
    f("sqrt", &|x| x.abs().sqrt()); f("cbrt", &|x| libm::cbrt(x));
    for i in 0..xs.len() { let (y, x) = (xs[i], xs[(i * 7 + 3) % xs.len()]); println!("atan2 {:x}_{:x} {:x}", y.to_bits(), x.to_bits(), libm::atan2(y, x).to_bits()); }
}
