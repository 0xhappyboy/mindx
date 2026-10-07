use crate::input::InnerMind;
use crate::rule_def::{Program, Rule};
pub fn eval(p: &Program, args: &[f64]) -> Option<f64> {
    match p {
        Program::Var(i) => args.get(*i).copied(),
        Program::Const(c) => Some(*c),
        Program::Add(a, b) => Some(eval(a, args)? + eval(b, args)?),
        Program::Sub(a, b) => Some(eval(a, args)? - eval(b, args)?),
        Program::Mul(a, b) => Some(eval(a, args)? * eval(b, args)?),
        Program::Div(a, b) => {
            let d = eval(b, args)?;
            if d.abs() < 1e-12 {
                None
            } else {
                Some(eval(a, args)? / d)
            }
        }
    }
}
pub fn enumerate(arity: usize, depth: usize) -> Vec<Program> {
    if depth == 0 {
        let mut b = Vec::new();
        for i in 0..arity {
            b.push(Program::Var(i));
        }
        for c in [0.0, 1.0, 2.0] {
            b.push(Program::Const(c));
        }
        return b;
    }
    let sub = enumerate(arity, depth - 1);
    let mut out: Vec<Program> = sub.clone();
    for a in &sub {
        for b in &sub {
            out.push(Program::Add(Box::new(a.clone()), Box::new(b.clone())));
            out.push(Program::Sub(Box::new(a.clone()), Box::new(b.clone())));
            out.push(Program::Mul(Box::new(a.clone()), Box::new(b.clone())));
            out.push(Program::Div(Box::new(a.clone()), Box::new(b.clone())));
        }
    }
    out
}
pub fn induce_rule(mind: &InnerMind, name: &str) -> Option<Rule> {
    let ex = &mind.pending_examples;
    if ex.is_empty() {
        return None;
    }
    let arity = ex[0].0.len();
    for depth in 1..=3 {
        for p in enumerate(arity, depth) {
            let ok = ex
                .iter()
                .all(|(args, exp)| eval(&p, args).map_or(false, |v| (v - exp).abs() < 1e-6));
            if ok {
                return Some(Rule {
                    name: name.to_string(),
                    arity,
                    program: p,
                });
            }
        }
    }
    None
}
