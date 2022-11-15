use crate::ast::Expr;
use crate::tipo::Tipo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub params: Vec<(String, Tipo)>,
    pub ret: Tipo,
    pub body: Box<Expr>,
}

impl Function {
    pub fn get_tipo(&self) -> Tipo {
        let args = self.params.iter().map(|(_, t)| t).cloned().collect();

        Tipo::new_fn(args, self.ret.clone())
    }
}
