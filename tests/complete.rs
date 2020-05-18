#[macro_use]
extern crate clap;
#[macro_use]
extern crate modularcli;
use ::modularcli::prelude::*;

pub struct Context(String);

dispatchers! {
    Food(&self, _: &mut Context) -> Result<()> [
        Meat: meat::Meat,
        Veggies: veggies::Veggies,
        Shell: Shell<Context, (), Food>,
    ],
}

mod meat {
    use ::modularcli::prelude::*;

    dispatchers! {
        Meat(&self, _: &mut crate::Context) -> Result<()> [
            Beef: Beef,
        ],
    }

    commands! {
        Beef(&self, ctx: &mut crate::Context) -> Result<()> {
            ctx.0 = format!("Hello, {}!", self.name);
            Ok(())
        } struct {
            name: String,
        }
    }
}

mod veggies {
    use ::modularcli::prelude::*;

    dispatchers! {
        Veggies(&self, _: &mut crate::Context) -> Result<()> [
            Carrots: Carrots,
            Lettuce: Lettuce,
        ],
    }

    commands! {
        Carrots(&self, _ctx: &mut crate::Context) -> Result<()> {
            Ok(())
        } struct {
            name: String,
        }

        Lettuce(&self, ctx: &mut crate::Context) -> Result<()> {
            ctx.0 = if let Some(name) = self.name.as_ref() {
                format!("Hello, {}!", name)
            } else {
                "Hello, world!".to_owned()
            };

            Ok(())
        } struct {
            name: Option<String>,
        }
    }
}

#[test]
fn call_beef() {
    assert!(Food::try_parse_from(vec!["food", "meat", "beef"]).is_err());

    let mut ctx = Context("".to_owned());
    let app = Food::try_parse_from(vec!["food", "meat", "beef", "freddie"]).unwrap();
    assert!(app.run(&mut ctx).is_ok());
    assert_eq!(ctx.0, "Hello, freddie!");
}

#[test]
fn call_lettuce() {
    let mut ctx = Context("".to_owned());
    let app = Food::try_parse_from(vec!["food", "veggies", "lettuce", "friend"]).unwrap();
    assert!(app.run(&mut ctx).is_ok());
    assert_eq!(ctx.0, "Hello, friend!");

    let mut ctx = Context("".to_owned());
    let app = Food::try_parse_from(vec!["food", "veggies", "lettuce"]).unwrap();
    assert!(app.run(&mut ctx).is_ok());
    assert_eq!(ctx.0, "Hello, world!");
}
