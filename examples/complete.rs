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
    // The return type must be the same    vv  for every command and dispatcher in the tree.
    Food(&self, _: &mut Context) -> Result<()> [
        Veggies: veggies::Veggies,
        Meat: meat::Meat,
    ],
}

mod veggies {
    use ::modularcli::prelude::*;

    dispatchers! {
        #[clap(about = "Fresh from the garden")]
        Veggies(&self, _: &mut crate::Context) -> Result<()> [
            Carrots: Carrots,
            Lettuce: Lettuce,
            #[clap(alias = "sh")]
            Shell: Shell<crate::Context, (), Veggies>,
        ],
    }

    commands! {
        Carrots(&self, _ctx: &mut crate::Context) -> Result<()> {
            Ok(())
        } struct {
            #[clap(short, long)]
            name: Option<String>,
        }

        Lettuce(&self, _ctx: &mut crate::Context) -> Result<()> {
            if let Some(name) = self.name.as_ref() {
                println!("Welcome to the table, {}", self.name.as_ref().map(|s| s.as_ref()).unwrap_or("unknown"));
            }
            Ok(())
        } struct {
            name: Option<String>,
        }
    }
}

mod meat {
    use ::modularcli::prelude::*;

    dispatchers! {
        // Overriding the command name at this level is not going to work
        #[clap(name = "carne", about = "Chillin n Grillin")]
        Meat(&self, _: &mut crate::Context) -> Result<()> [
            // If you want to override the name of a command, do it here.
            #[clap(name = "boeuf")]
            Beef: Beef,
        ],
    }

    commands! {
        // The "about" override here and the "name" override in the Meat dispatcher will combine.
        #[clap(about = "Beef. It's What for Dinner")]
        Beef(&self, ctx: &mut crate::Context) -> Result<()> {
            ctx.0 = format!("Welcome to the table, {}!", self.name);
            Ok(())
        } struct {
            name: String,
        }
    }
}
