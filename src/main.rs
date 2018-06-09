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

impl ToString for Term {
    fn to_string(&self) -> String {
        use Term::*;
        match *self {
            Var(ref s) => s.to_string(),
            Func(ref s, ref v) => {
                if v.len() == 0 {
                    s.to_string()
                } else {
                    let term_strs: Vec<String> = v.iter().map(|t| t.to_string()).collect();
                    format!("{}({})", s, term_strs.join(","))
                }
            }
        }
    }
}

impl Term {
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
                Func(s.to_string(), unified_term)
            }
        }
    }
    fn appear(&self, x: &String) -> bool {
        use Term::*;
        match *self {
            Var(ref s) => s == x,
            Func(_, ref v) => {
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
            (Var(x), t) | (t, Var(x)) => {
                if t.appear(&x) {
                    return None;
                } else {
                    let u = (x, t);
                    constraints = unify(&mut constraints, &u);
                    unifier.push(u);
                }
            }
            (Func(f1, v1), Func(f2, v2)) => {
                if f1 == f2 && v1.len() == v2.len() {
                    for e in v1.iter().map(|t| t.clone()).zip(v2) {
                        constraints.push_back(e);
                    }
                } else {
                    return None;
                }
            }
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
