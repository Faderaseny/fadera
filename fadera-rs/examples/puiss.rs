fn main() { for x in 2..=60u32 { for y in 2..=120u32 { let r = fadera::mathfn::pow(x as f64, y as f64); if r.is_finite() { println!("{} {} {:x}", x, y, r.to_bits()); } } } }
