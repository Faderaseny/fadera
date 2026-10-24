//! Lit des lignes « fonction bits_x [bits_y] » (hexadécimal) sur l'entrée standard, écrit les bits du résultat.
//! Sert à comparer bit à bit le moteur Rust et le portage JavaScript (tests/verifier_maths.js).
use fadera::mathfn as m;
use std::io::{BufRead, Write};

fn b(h: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(h, 16).unwrap())
}
fn main() {
    let stdin = std::io::stdin();
    let mut out = std::io::BufWriter::new(std::io::stdout());
    for l in stdin.lock().lines() {
        let l = l.unwrap();
        let p: Vec<&str> = l.split(' ').collect();
        let x = b(p[1]);
        let r = match p[0] {
            "sin" => m::sin(x),
            "cos" => m::cos(x),
            "tan" => m::tan(x),
            "atan" => m::atan(x),
            "asin" => m::asin(x),
            "acos" => m::acos(x),
            "exp" => m::exp(x),
            "log" => m::log(x),
            "log2" => m::log2(x),
            "log10" => m::log10(x),
            "cbrt" => m::cbrt(x),
            "atan2" => m::atan2(x, b(p[2])),
            "pow" => m::pow(x, b(p[2])),
            f => panic!("fonction inconnue {}", f),
        };
        writeln!(out, "{:016x}", r.to_bits()).unwrap();
    }
}
