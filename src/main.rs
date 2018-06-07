use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
enum Term {
    Var(String),
    Func(String, Vec<Term>),
}

type Equality = (Term, Term);
type Constraints = VecDeque<Equality>;

type VarTerm = (String, Term);
type Unifier = Vec<VarTerm>;

impl Term {
    fn to_string(&self) -> String {
        use Term::*;
        match *self {
            Var(ref s) => s.to_string(),
            Func(ref s, ref v) => {
                let term_strs: Vec<String> = v.iter().map(|t| t.to_string()).collect();
                format!("{}({})", s, term_strs.join(","))
            }
        }
    }
    fn unify(&self, u: &VarTerm) -> Self {
        use Term::*;
        match *self {
            Var(ref s) => if *s == u.0 {
                u.1.clone()
            } else {
                self.clone()
            },
            Func(ref s, ref v) => {
                let unified_term: Vec<Term> = v.iter().map(|t| t.unify(u)).collect();
                Func(s.clone(), unified_term)
            }
        }
    }
    fn appear(&self, x: &String) -> bool {
        use Term::*;
        match *self {
            Var(ref s) => s == x,
            Func(ref s, ref v) => {
                for t in v {
                    if t.appear(x) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

fn unify(constraints: &mut Constraints, u: &VarTerm) -> Constraints {
    let mut unified_constraints = Constraints::new();
    while !constraints.is_empty() {
        let constraint = constraints.pop_front().unwrap();
        unified_constraints.push_back((constraint.0.unify(&u), constraint.1.unify(&u)));
    }
    return unified_constraints;
}

fn calculate_mgu(constraints: &Constraints) -> Option<Unifier> {
    use Term::*;
    let mut constraints = constraints.clone();
    let mut unifier = Unifier::new();
    while !constraints.is_empty() {
        let constraint = constraints.pop_front().unwrap();
        match constraint {
            (Var(ref x), ref t) | (ref t, Var(ref x)) => {
                if t.appear(x) {
                    return None;
                } else {
                    let u = (x.clone(), t.clone());
                    constraints = unify(&mut constraints, &u);
                    unifier.push(u);
                }
            }
            (Func(ref f1, ref v1), Func(ref f2, ref v2)) => {
                if f1 == f2 && v1.len() == v2.len() {
                    for e in v1.iter().zip(v2) {
                        constraints.push_back((e.0.clone(), e.1.clone()));
                    }
                } else {
                    return None;
                }
            }
            _ => {}
        }
    }
    Some(unifier)
}

fn main() {
    use Term::*;
    let t1 = Func(
        "f".to_string(),
        vec![
            Func("g".to_string(), vec![Var("y".to_string())]),
            Var("z".to_string()),
        ],
    );
    let t2 = Func(
        "f".to_string(),
        vec![
            Var("x".to_string()),
            Func("g".to_string(), vec![Var("y".to_string())]),
        ],
    );
    println!("{{ {} = {} }}", t1.to_string(), t2.to_string());
    let mut constraints: Constraints = Constraints::new();
    constraints.push_back((t1, t2));
    match calculate_mgu(&constraints) {
        Some(unifier) => {
            for (x, t) in unifier {
                print!("[{}/{}]", t.to_string(), x);
            }
            println!("");
        }
        None => {
            println!("fail");
        }
    }
}
