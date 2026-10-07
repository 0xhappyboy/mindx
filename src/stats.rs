use crate::input::InnerMind;
pub fn stats(mind: &InnerMind) -> String {
    let edges: usize = mind.w.iter().map(|r| r.len()).sum();
    format!("{} {} {}", mind.vocab.len(), edges, mind.merged_words.len())
}
