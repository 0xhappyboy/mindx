use crate::induction::induce_rule;
use crate::input::InnerMind;
use crate::number::{parse_number, split_by_operator};
pub fn try_parse(mind: &mut InnerMind, text: &str) -> Option<String> {
    let (left, right) = text.split_once('=')?;
    let left = left.trim();
    let right = right.trim();
    let result = parse_number(mind, right)?;
    let (op_token, ls, rs) = split_by_operator(mind, left)?;
    let op_name = mind
        .equiv
        .get(&op_token)
        .cloned()
        .unwrap_or(op_token.clone());
    let a = parse_number(mind, &ls)?;
    let b = parse_number(mind, &rs)?;
    mind.pending_examples.push((vec![a, b], result));
    let op = mind.pending_op.clone().unwrap_or(op_name);
    if mind.pending_examples.len() >= 3 {
        if let Some(rule) = induce_rule(mind, &op) {
            let ok = mind.pending_examples.iter().all(|(args, exp)| {
                crate::induction::eval(&rule.program, args)
                    .map_or(false, |v| (v - exp).abs() < 1e-6)
            });
            if ok {
                mind.rules.push(rule);
                mind.pending_examples.clear();
                mind.pending_op = None;
                return Some(op);
            }
        }
    }
    Some(op_token)
}
