use crate::expr::Expr;

pub struct Evaluator<'a> {
    exprs: &'a Vec<Expr>,
}

impl<'a> Evaluator<'a> {
    pub fn new(exprs: &'a Vec<Expr>) -> Self {
        Self { exprs }
    }

    pub fn evaluate(&self) {
        for expr in self.exprs {
            println!("{}", self.token_to_string(expr));
        }
    }

    fn token_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::String(s) => s.trim_matches('"').to_string(),
            Expr::Number(n) => {
                if n.contains('.') {
                    let trimmed = n.trim_end_matches('0');
                    if trimmed.ends_with('.') {
                        trimmed.trim_end_matches('.').to_string()
                    } else {
                        trimmed.to_string()
                    }
                } else {
                    n.to_string()
                }
            }
            Expr::Group(g) => self.token_to_string(g),
            _ => expr.to_string(),
        }
    }
}
