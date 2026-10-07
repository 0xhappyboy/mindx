use crate::html::is_ideographic;
use crate::input::{InnerMind, MERGE_THRESHOLD};
/// Only ideographic pairs are counted.
/// Letter scripts are separated by whitespace already, so pair statistics would only add noise there.
pub fn update_pair_freq(mind: &mut InnerMind, text: &str) {
    let chars: Vec<char> = text.chars().collect();
    for i in 0..chars.len().saturating_sub(1) {
        let a = chars[i];
        let b = chars[i + 1];
        if is_ideographic(a) && is_ideographic(b) {
            let key = (a.to_string(), b.to_string());
            *mind.pair_freq.entry(key).or_insert(0.0) += 1.0;
        }
    }
}
pub fn discover_words(mind: &mut InnerMind) -> usize {
    let before = mind.merged_words.len();
    let freq = mind.pair_freq.clone();
    for ((a, b), f) in &freq {
        if *f >= MERGE_THRESHOLD {
            let w = format!("{}{}", a, b);
            if !mind.merged_words.contains(&w) {
                mind.merged_words.push(w);
            }
        }
    }
    mind.merged_words.len() - before
}
