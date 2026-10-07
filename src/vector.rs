use crate::input::{InnerMind, K, N};
pub fn atom(_mind: &InnerMind, s: &str) -> Vec<f32> {
    let mut v = vec![0.0; N];
    let mut h: u64 = 1469598103934665603;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    for _ in 0..K {
        h = h
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        v[(h as usize) % N] = 1.0;
    }
    crate::associate::normalize(&mut v);
    v
}
pub fn encode(mind: &InnerMind, tokens: &[String]) -> Vec<f32> {
    let mut v = vec![0.0; N];
    for t in tokens {
        let a = atom(mind, t);
        for i in 0..N {
            v[i] += a[i];
        }
    }
    crate::associate::normalize(&mut v);
    v
}
