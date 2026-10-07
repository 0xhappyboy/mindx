use crate::html::{is_ideographic, is_letter};
use crate::input::InnerMind;
pub fn tokenize(mind: &InnerMind, s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if is_letter(c) {
            let mut j = i;
            while j < chars.len() && is_letter(chars[j]) {
                j += 1;
            }
            let run: String = chars[i..j].iter().collect();
            out.push(run);
            i = j;
            continue;
        }
        if c.is_numeric() {
            let mut j = i;
            while j < chars.len() && chars[j].is_numeric() {
                j += 1;
            }
            let run: String = chars[i..j].iter().collect();
            out.push(run);
            i = j;
            continue;
        }
        if let Some((w, len)) = longest_match(mind, &chars[i..]) {
            out.push(w);
            i += len;
            continue;
        }
        if is_ideographic(c) {
            out.push(c.to_string());
            i += 1;
            continue;
        }
        out.push(c.to_string());
        i += 1;
    }
    out
}
fn longest_match(mind: &InnerMind, chars: &[char]) -> Option<(String, usize)> {
    let max_len = chars.len().min(6);
    for len in (1..=max_len).rev() {
        let candidate: String = chars[..len].iter().collect();
        if mind.merged_words.contains(&candidate) || mind.vocab.contains(&candidate) {
            return Some((candidate, len));
        }
    }
    None
}
