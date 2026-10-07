use crate::input::InnerMind;
use crate::tokenize::tokenize;
pub fn try_parse(mind: &mut InnerMind, text: &str) -> Option<String> {
    let tokens = tokenize(mind, text);
    if tokens.len() < 3 {
        return None;
    }
    if let Some(sk) = mind.same_kw.clone() {
        if let Some(p) = tokens.iter().position(|t| *t == sk) {
            if p > 0 && p + 1 < tokens.len() {
                let a = tokens[p - 1].clone();
                let b = tokens[p + 1].clone();
                mind.equiv.insert(b.clone(), a.clone());
                crate::query::vocab_add(mind, &b);
                return Some(b);
            }
        }
        return None;
    }
    if tokens.len() == 3 {
        let a = tokens[0].clone();
        let k = tokens[1].clone();
        let b = tokens[2].clone();
        if crate::number::is_operator(mind, &k) {
            return None;
        }
        if mind.rules.iter().any(|r| r.name == k) {
            return None;
        }
        mind.same_kw = Some(k);
        mind.equiv.insert(b.clone(), a.clone());
        crate::query::vocab_add(mind, &b);
        return Some(b);
    }
    None
}
