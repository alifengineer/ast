use crate::context::DataContext;

enum Expr {}

enum Condition {}

impl Condition {
    fn evaluate(&self, ctx: &DataContext) -> Result<bool, ()> {
        // TODO: implement
        Ok(false)
    }
}

enum Action {

}

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