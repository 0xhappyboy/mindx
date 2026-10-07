use crate::input::InnerMind;
use crate::tokenize::tokenize;
pub fn parse_number(mind: &InnerMind, s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }
    if let Some(rep) = mind.equiv.get(s) {
        if let Ok(v) = rep.parse::<f64>() {
            return Some(v);
        }
    }
    if let Some(entry) = mind.vocab.iter().find(|v| v.as_str() == s) {
        if let Ok(v) = entry.parse::<f64>() {
            return Some(v);
        }
    }
    None
}
pub fn split_by_operator(mind: &InnerMind, s: &str) -> Option<(String, String, String)> {
    let tokens = tokenize(mind, s);
    let idx = tokens.iter().position(|t| is_operator(mind, t))?;
    if idx == 0 || idx == tokens.len() - 1 {
        return None;
    }
    Some((
        tokens[idx].clone(),
        tokens[..idx].join(" "),
        tokens[idx + 1..].join(" "),
    ))
}
// An operator is anything the system has learned as a rule or equivalence.
pub fn is_operator(mind: &InnerMind, t: &str) -> bool {
    mind.equiv.contains_key(t) || mind.rules.iter().any(|r| r.name == t)
}
pub fn try_compute(mind: &mut InnerMind, text: &str) -> Option<String> {
    let (op, ls, rs) = split_by_operator(mind, text)?;
    let name = mind.equiv.get(&op).cloned().unwrap_or(op.clone());
    let rule = mind.rules.iter().find(|r| r.name == name)?.clone();
    let a = parse_number(mind, &ls)?;
    let b = parse_number(mind, &rs)?;
    let r = crate::induction::eval(&rule.program, &[a, b])?;
    Some(format!("{}", r as i64))
}
