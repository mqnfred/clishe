#[macro_use]
extern crate modularcli;
use ::modularcli::prelude::*;

struct Context(String);

commands! {
    Remove(ctx: &mut Context, _matches: &ArgMatches) -> Result<()> {
        ctx.0 = "remove called".to_owned();
        Ok(())
    } using App::new("remove"),
}

dispatchers! {
    Entry(_: &mut Context, _: &ArgMatches) -> Result<()> [
        Remove,
    ] using App::new("entry"),
}

fn exec(ctx: &mut Context, args: Vec<&'static str>) -> Result<()> {
    execute_command(
        &mut Entry::default(),
        ctx,
        &args.into_iter().map(|s| s.to_owned()).collect(),
    )
}

#[test]
fn call_remove() {
    let mut ctx = Context("".to_owned());
    assert!(exec(&mut ctx, vec!["remove"]).is_ok());
    assert_eq!(ctx.0, "remove called");
}

/*
hybrids! {
    Nope(ctx: &mut Context, matches: &ArgMatches) -> Result<()> {
        Ok(())
    } dispatches [
        Hello,
    ] using App::new("hello"),
}
*/
