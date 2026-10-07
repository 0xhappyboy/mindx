use crate::equivalence;
use crate::example;
use crate::hebbian;
use crate::number;
use crate::query;
use crate::rule_def;
use crate::tokenize;
use crate::vector;
use crate::word_discovery;
use std::collections::HashMap;
pub const N: usize = 16384;
pub const K: usize = 16;
pub const ETA: f32 = 0.2;
pub const DECAY: f32 = 0.999;
pub const MERGE_THRESHOLD: f32 = 2.0;
pub const MIN_COOCCURRENCE: u32 = 2;
pub struct InnerMind {
    pub vocab: Vec<String>,
    pub w: Vec<HashMap<usize, f32>>,
    pub w_count: Vec<HashMap<usize, u32>>,
    pub equiv: HashMap<String, String>,
    pub rules: Vec<rule_def::Rule>,
    pub pending_examples: Vec<(Vec<f64>, f64)>,
    pub pending_op: Option<String>,
    pub pair_freq: HashMap<(String, String), f32>,
    pub merged_words: Vec<String>,
    pub rule_kw: Option<String>,
    pub means_kw: Option<String>,
    pub same_kw: Option<String>,
}
impl InnerMind {
    pub fn new() -> Self {
        InnerMind {
            vocab: Vec::new(),
            w: (0..N).map(|_| HashMap::new()).collect(),
            w_count: (0..N).map(|_| HashMap::new()).collect(),
            equiv: HashMap::new(),
            rules: Vec::new(),
            pending_examples: Vec::new(),
            pending_op: None,
            pair_freq: HashMap::new(),
            merged_words: Vec::new(),
            rule_kw: None,
            means_kw: None,
            same_kw: None,
        }
    }
    // Learn only. No reply.
    pub fn perceive(&mut self, text: &str) -> String {
        let text = text.trim();
        if text.is_empty() {
            return String::new();
        }
        word_discovery::update_pair_freq(self, text);
        if let Some(r) = rule_def::try_parse(self, text) {
            return r;
        }
        if let Some(r) = example::try_parse(self, text) {
            return r;
        }
        if let Some(r) = number::try_compute(self, text) {
            return r;
        }
        if let Some(r) = equivalence::try_parse(self, text) {
            return r;
        }
        let tokens = tokenize::tokenize(self, text);
        if tokens.is_empty() {
            return String::new();
        }
        for t in &tokens {
            query::vocab_add(self, t);
        }
        let vecs: Vec<Vec<f32>> = tokens.iter().map(|t| vector::atom(self, t)).collect();
        for i in 0..vecs.len() {
            if i + 1 < vecs.len() {
                hebbian::learn(self, &vecs[i], &vecs[i + 1], ETA);
            }
            if i + 2 < vecs.len() {
                hebbian::learn(self, &vecs[i], &vecs[i + 2], ETA * 0.5);
            }
        }
        String::new()
    }
    // Learn and reply.
    pub fn process(&mut self, text: &str) -> String {
        let text = text.trim();
        if text.is_empty() {
            return String::new();
        }
        word_discovery::update_pair_freq(self, text);
        if let Some(r) = rule_def::try_parse(self, text) {
            return r;
        }
        if let Some(r) = example::try_parse(self, text) {
            return r;
        }
        if let Some(r) = number::try_compute(self, text) {
            return r;
        }
        if let Some(r) = equivalence::try_parse(self, text) {
            return r;
        }
        let tokens = tokenize::tokenize(self, text);
        if tokens.is_empty() {
            return String::new();
        }
        for t in &tokens {
            query::vocab_add(self, t);
        }
        let vecs: Vec<Vec<f32>> = tokens.iter().map(|t| vector::atom(self, t)).collect();
        for i in 0..vecs.len() {
            if i + 1 < vecs.len() {
                hebbian::learn(self, &vecs[i], &vecs[i + 1], ETA);
            }
            if i + 2 < vecs.len() {
                hebbian::learn(self, &vecs[i], &vecs[i + 2], ETA * 0.5);
            }
        }
        query::associate_query(self, text)
    }
}
