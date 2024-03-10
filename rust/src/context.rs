use crate::guard::Guard;

#[derive(Clone)]
pub struct Context {
    pub indent: usize,
}

impl Context {
    pub fn new(indent: usize) -> Context {
        Context { indent }
    }

    pub fn guard(&mut self, new_ctx: Context) -> ContextGuard {
        ContextGuard::new(self, new_ctx)
    }
}

pub struct ContextGuard {
    ctx: Context,
}

impl Guard<Context> for ContextGuard {
    fn new(ctx: &mut Context, new_ctx: Context) -> ContextGuard {
        let original_ctx = ctx.clone();
        ctx.indent = new_ctx.indent;
        ContextGuard { ctx: original_ctx }
    }

    fn restore(&self, ctx: &mut Context) {
        ctx.indent = self.ctx.indent;
    }
}
