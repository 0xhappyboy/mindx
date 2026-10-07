use crate::input::{DECAY, InnerMind};
pub fn decay_all(mind: &mut InnerMind) {
    for row in mind.w.iter_mut() {
        row.retain(|_, w| {
            *w *= DECAY;
            w.abs() > 1e-5
        });
    }
    for row in mind.w_count.iter_mut() {
        row.retain(|_, c| {
            if *c > 0 {
                *c -= 1;
            }
            *c > 0
        });
    }
}
