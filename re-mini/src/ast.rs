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
            Expr::FieldRef(name) => ctx
                .get(String::from(name))
                .cloned()
                .ok_or_else(|| format!("field {} not found", name)),
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

enum CmpOp {
    Eq,
    Lt,
    Gt,
}

enum Condition {
    Compare { left: Expr, op: CmpOp, right: Expr },
    Or(Box<Condition>, Box<Condition>),
    And(Box<Condition>, Box<Condition>),
}

impl Condition {
    fn evaluate(&self, ctx: &DataContext) -> Result<bool, String> {
        match self {
            Condition::Compare { left, op, right } => {
                let l = left.evaluate(ctx);
                let r = right.evaluate(ctx);
                match op {
                    CmpOp::Eq => Ok(l.eq(&r)),
                    CmpOp::Gt => Ok(l.gt(&r)),
                    CmpOp::Lt => Ok(l.lt(&r)),
                }
            }
            Condition::And(a, b) => Ok(a.evaluate(ctx)? && b.evaluate(ctx)?),
            Condition::Or(a, b) => Ok(a.evaluate(ctx)? || b.evaluate(ctx)?),
        }
    }
}

enum Action {
    Assign { field: String, expr: Expr },
}

impl Action {
    fn execute(&self, ctx: &mut DataContext) -> Result<(), String> {
        match self {
            Action::Assign { field, expr } => {
                let val = expr.evaluate(ctx)?;
                ctx.set(String::from(field), val);

                Ok(())
            }
        }
    }
}

pub struct Rule {
    name: String,
    condition: Condition,
    actions: Vec<Action>,
}

impl Rule {
    fn evaluate(&self, ctx: &DataContext) -> Result<bool, String> {
        self.condition.evaluate(ctx)
    }

    fn execute(&self, ctx: &mut DataContext) -> Result<(), String> {
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

    // Example
    // rule: "when A == 3, set C = A + B"
    #[test]
    fn test_rule_evaluate_and_execute() {
        let mut ctx = DataContext::new();
        ctx.set("A".into(), Value::Int(3));
        ctx.set("B".into(), Value::Int(5));

        let mut actions = Vec::new();
        actions.push(Action::Assign {
            field: "C".into(),
            expr: Expr::BinOp {
                left: Box::new(Expr::FieldRef("A".into())),
                op: Op::Add,
                right: Box::new(Expr::FieldRef("B".into())),
            },
        });

        let rule = Rule {
            name: String::from("add_rule"),
            condition: Condition::Compare {
                left: Expr::FieldRef("A".into()),
                op: CmpOp::Eq,
                right: Expr::Literal(Value::Int(3)),
            },
            actions: actions,
        };

        assert_eq!(rule.evaluate(&ctx).unwrap(), true);
        rule.execute(&mut ctx).unwrap();

        assert_eq!(ctx.get("C".into()), Some(&Value::Int(8)));
    }
}
