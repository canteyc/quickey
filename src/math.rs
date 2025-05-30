use std::cmp::Ordering;

pub fn frac(n: u32, d: u32) -> f64 {
    n as f64 / d as f64
}

pub fn cmp_f32(a: f32, b: f32) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Less)
}
