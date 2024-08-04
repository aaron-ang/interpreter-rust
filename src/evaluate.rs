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
            println!("{}", expr);
        }
    }
}
