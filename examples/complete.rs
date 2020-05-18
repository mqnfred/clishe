#[macro_use]
extern crate clap;
#[macro_use]
extern crate modularcli;
use ::modularcli::prelude::*;

fn main() {
    let mut ctx = Context("".to_owned());
    if let Err(err) = Food::parse().run(&mut ctx) {
        eprintln!("error: {}", err);
    } else {
        println!("success: {}", ctx.0);
    }
}

pub struct Context(String);

dispatchers! {
    Food(&self, _: &mut Context) -> Result<()> [
        Shell: Shell<Context, (), Food>,
        Veggies: veggies::Veggies,
        Meat: meat::Meat,
    ],
}

mod veggies {
    use ::modularcli::prelude::*;

    dispatchers! {
        #[clap(name = "veggies", about = "fresh from the garden")]
        Veggies(&self, _: &mut crate::Context) -> Result<()> [
            Shell: Shell<crate::Context, (), Veggies>,
            Carrots: Carrots,
            Lettuce: Lettuce,
        ],
    }

    commands! {
        #[clap(name = "carottes")]
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

mod meat {
    use ::modularcli::prelude::*;

    dispatchers! {
        #[clap(about = "nothing like a properly cooked meat")]
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
