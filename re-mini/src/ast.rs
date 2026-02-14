use crate::{context::DataContext, value::Value};

enum Expr {
    Literal(Value),
    FieldRef(String),
    BinOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
}

enum Op {
    Add,
}

impl Expr {
    fn evaluate(&self, ctx: &DataContext) -> Result<Value, String> {
        match self {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::FieldRef(name) => {
                ctx.get(String::from(name)).cloned().ok_or_else(|| format!("field {} not found", name))
            },
            Expr::BinOp { left, op, right } => {
                let l = left.evaluate(ctx)?;
                let r = right.evaluate(ctx)?;
                match op {
                    Op::Add => l.add(&r),
                }
            }
        }
    }
}

enum Condition {}

impl Condition {
    fn evaluate(&self, ctx: &DataContext) -> Result<bool, ()> {
        // TODO: implement
        Ok(false)
    }
}

enum Action {}

impl Action {
    fn execute(&self, ctx: &mut DataContext) -> Result<(), ()> {
        // TODO: here action on ctx
        Ok(())
    }
}

pub struct Rule {
    name: String,
    condition: Condition,
    actions: Vec<Action>,
}

impl Rule {
    fn evaluate(&self, ctx: &DataContext) -> Result<bool, ()> {
        self.condition.evaluate(ctx)
    }

    fn execute(&self, ctx: &mut DataContext) -> Result<(), ()> {
        for action in &self.actions {
            action.execute(ctx)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // means: import all from parent module (which is `ast`)

    #[test]
    fn test_expr_add() {
        let mut ctx = DataContext::new();
        ctx.set("A".into(), Value::Int(3));
        ctx.set("B".into(), Value::Int(5));

        let expr = Expr::BinOp {
            left: Box::new(Expr::FieldRef("A".into())),
            op: Op::Add,
            right: Box::new(Expr::FieldRef("B".into())),
        };
        let result = expr.evaluate(&ctx).unwrap();
        assert_eq!(result, Value::Int(8));
    }
}
