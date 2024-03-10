pub trait Guard<Ctx> {
    fn new(ctx: &mut Ctx, new_ctx: Ctx) -> Self;
    fn restore(&self, ctx: &mut Ctx);
}
