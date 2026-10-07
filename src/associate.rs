use crate::input::{InnerMind, N};
pub fn normalize(v: &mut [f32]) {
    let n: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if n > 1e-6 {
        for x in v.iter_mut() {
            *x /= n;
        }
    }
}
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na > 1e-6 && nb > 1e-6 {
        dot / (na * nb)
    } else {
        0.0
    }
}
pub fn associate(mind: &InnerMind, cue: &[f32]) -> Vec<f32> {
    let mut y = vec![0.0; N];
    for i in 0..N {
        if cue[i].abs() < 1e-6 {
            continue;
        }
        for (&j, &w) in &mind.w[i] {
            y[j] += w * cue[i];
        }
    }
    normalize(&mut y);
    y
}
