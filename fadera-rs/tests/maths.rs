//! Les fonctions mathématiques doivent donner le double le plus proche de la vraie valeur (arrondi correct).
//! Vecteurs générés avec mpmath (300 bits) : « fonction bits_argument bits_attendu ».
use fadera::mathfn as m;

fn bits(h: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(h, 16).unwrap())
}

#[test]
fn arrondi_correct_sur_vecteurs_mpmath() {
    let src = include_str!("maths_vecteurs.txt");
    let mut n = 0;
    for l in src.lines().filter(|l| !l.is_empty()) {
        let p: Vec<&str> = l.split(' ').collect();
        let attendu = u64::from_str_radix(p[2], 16).unwrap();
        let obtenu = if p[0] == "atan2" {
            let (y, x) = p[1].split_once('_').unwrap();
            m::atan2(bits(y), bits(x))
        } else {
            let a = bits(p[1]);
            match p[0] {
                "sin" => m::sin(a),
                "cos" => m::cos(a),
                "tan" => m::tan(a),
                "atan" => m::atan(a),
                "asin" => m::asin(a),
                "acos" => m::acos(a),
                "exp" => m::exp(a),
                "log" => m::log(a),
                "log2" => m::log2(a),
                "log10" => m::log10(a),
                "cbrt" => m::cbrt(a),
                "pow_3_7" => m::pow(a, 3.7),
                "pow_x_1_5" => m::pow(a, 1.5),
                "pow_2_x" => m::pow(2.0, a),
                "pow_10_x" => m::pow(10.0, a),
                "pow_1_1_x" => m::pow(1.1, a),
                "pow_x_3" => m::pow(a, 3.0),
                "pow_x_m5" => m::pow(a, -5.0),
                f => panic!("fonction inconnue {}", f),
            }
        };
        assert_eq!(obtenu.to_bits(), attendu, "{} : obtenu {:e}, attendu {:e}", l, obtenu, f64::from_bits(attendu));
        n += 1;
    }
    assert!(n > 700, "trop peu de vecteurs : {}", n);
}

#[test]
fn valeurs_remarquables() {
    assert_eq!(m::exp(1.0), std::f64::consts::E);
    assert_eq!(m::exp(0.0), 1.0);
    assert_eq!(m::log(1.0), 0.0);
    assert_eq!(m::log(std::f64::consts::E), 1.0);
    assert_eq!(m::log2(8.0), 3.0);
    assert_eq!(m::log10(1000.0), 3.0);
    assert_eq!(m::sin(0.0).to_bits(), 0.0f64.to_bits());
    assert_eq!(m::sin(-0.0).to_bits(), (-0.0f64).to_bits());
    assert_eq!(m::cos(0.0), 1.0);
    assert_eq!(m::atan(1.0), std::f64::consts::FRAC_PI_4);
    assert_eq!(m::asin(1.0), std::f64::consts::FRAC_PI_2);
    assert_eq!(m::acos(-1.0), std::f64::consts::PI);
    assert_eq!(m::atan2(1.0, 1.0), std::f64::consts::FRAC_PI_4);
    assert_eq!(m::pow(2.0, 10.0), 1024.0);
    assert_eq!(m::pow(10.0, 15.0), 1e15);
    assert_eq!(m::pow(-2.0, 3.0), -8.0);
    assert_eq!(m::pow(3.0, 34.0), 16677181699666568.0); // égalité exacte entre deux doubles : arrondi pair
    assert_eq!(m::cbrt(27.0), 3.0);
    assert!(m::log(-1.0).is_nan() && m::asin(2.0).is_nan() && m::pow(-8.0, 0.5).is_nan());
    assert_eq!(m::log(0.0), f64::NEG_INFINITY);
}
