use crate::input::InnerMind;
#[derive(Clone)]
pub struct Rule {
    pub name: String,
    pub arity: usize,
    pub program: Program,
}
#[derive(Clone)]
pub enum Program {
    Var(usize),
    Const(f64),
    Add(Box<Program>, Box<Program>),
    Sub(Box<Program>, Box<Program>),
    Mul(Box<Program>, Box<Program>),
    Div(Box<Program>, Box<Program>),
}
// Rule definition. No natural language is hard-coded.
pub fn try_parse(mind: &mut InnerMind, text: &str) -> Option<String> {
    let tokens = crate::tokenize::tokenize(mind, text);
    if tokens.len() < 4 {
        return None;
    }
    if let (Some(rk), Some(mk)) = (mind.rule_kw.clone(), mind.means_kw.clone()) {
        if let (Some(rp), Some(mp)) = (
            tokens.iter().position(|t| *t == rk),
            tokens.iter().position(|t| *t == mk),
        ) {
            if rp < mp && mp + 1 < tokens.len() {
                let name = tokens[rp + 1..mp].join(" ");
                if name.is_empty() {
                    return None;
                }
                return finish(mind, &name, &tokens[mp + 1..]);
            }
        }
        return None;
    }
    let rk = tokens[0].clone();
    let name = tokens[1].clone();
    let mk = tokens[2].clone();
    mind.rule_kw = Some(rk);
    mind.means_kw = Some(mk);
    finish(mind, &name, &tokens[3..])
}
fn finish(mind: &mut InnerMind, name: &str, symbols: &[String]) -> Option<String> {
    if name.is_empty() {
        return None;
    }
    if mind.rules.iter().any(|r| r.name == name) {
        return Some(name.to_string());
    }
    for s in symbols {
        mind.equiv.insert(s.clone(), name.to_string());
        crate::query::vocab_add(mind, s);
    }
    crate::query::vocab_add(mind, name);
    mind.pending_op = Some(name.to_string());
    mind.pending_examples.clear();
    Some(name.to_string())
}
