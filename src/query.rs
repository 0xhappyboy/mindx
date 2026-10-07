use crate::associate;
use crate::html::{is_ideographic, is_letter};
use crate::input::InnerMind;
use crate::tokenize::tokenize;
use crate::vector;
pub fn vocab_add(mind: &mut InnerMind, s: &str) {
    if !mind.vocab.iter().any(|v| v == s) {
        mind.vocab.push(s.to_string());
    }
}
// Coarse script tag. 0 = unknown.
pub fn lang_tag(s: &str) -> u8 {
    for c in s.chars() {
        let u = c as u32;
        if is_ideographic(c) {
            return 1;
        }
        if (0x3040..=0x30FF).contains(&u) {
            return 7;
        }
        if (0xAC00..=0xD7AF).contains(&u) {
            return 6;
        }
        if (0x0600..=0x06FF).contains(&u) {
            return 5;
        }
        if (0x0370..=0x03FF).contains(&u) {
            return 4;
        }
        if (0x0400..=0x04FF).contains(&u) {
            return 3;
        }
        if is_letter(c) {
            return 2;
        }
    }
    0
}
pub fn associate_query(mind: &InnerMind, text: &str) -> String {
    let tokens = tokenize(mind, text);
    if tokens.is_empty() {
        return String::new();
    }
    let cue = vector::encode(mind, &tokens);
    let state = associate::associate(mind, &cue);
    // Language tag of the query: first non-zero tag found.
    let q_lang = tokens
        .iter()
        .map(|t| lang_tag(t))
        .find(|&l| l != 0)
        .unwrap_or(0);
    let mut scores: Vec<(String, f32)> = mind
        .vocab
        .iter()
        .filter(|w| q_lang == 0 || lang_tag(w) == q_lang)
        .map(|w| (w.clone(), associate::cosine(&state, &vector::atom(mind, w))))
        .collect();
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let top: Vec<String> = scores
        .into_iter()
        .take(10)
        .filter(|(_, s)| *s > 0.05)
        .map(|(w, _)| w)
        .collect();
    top.join(" ")
}
