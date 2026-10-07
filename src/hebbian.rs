use crate::input::{InnerMind, MIN_COOCCURRENCE, N};
pub fn learn(mind: &mut InnerMind, pre: &[f32], post: &[f32], eta: f32) {
    for i in 0..N {
        if pre[i].abs() < 1e-6 {
            continue;
        }
        for j in 0..N {
            if post[j].abs() < 1e-6 {
                continue;
            }
            let c = mind.w_count[i].entry(j).or_insert(0);
            *c += 1;
            if *c >= MIN_COOCCURRENCE {
                let e = mind.w[i].entry(j).or_insert(0.0);
                *e += eta * pre[i] * post[j];
            }
        }
    }
}
