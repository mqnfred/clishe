# Clishé

Clishé is a cli mini-framework in rust. Write your CLI using an idiom axed
around the following values:

 1. Reduction of boilerplate/plumbing for repetitive command declaration
 2. Skeleton around to command args, mutable state (context), return type
 3. Command implementation and definition live side-to-side
 4. We should be able to generate shells on-demand based on a cli
 5. Auto-complete is a first-class citizen, in and out of the inner shell [wip]

When creating an ecosystem, clis are a great gateway into parts of the
ecosystem's deep libraries (encoding, server clients, ...) Lowering the barrier
to creating a sane cli is the objective of this library.

Clishé is a surprisingly thin wrapper around the following technologies:

 - `anyhow` for error handling
 - `clap` for cli building
 - `rustyline`/`shellwords` for shell generation

Most of the features powering this framework are owed to the authors of these
great crates. This crate is more of a DSL and library linking together
different technologies to solve a set of problems.

## Example

This is a simple app with dummy commands to display the spirit of clishe.

```rust
#[macro_use]
extern crate clap;
#[macro_use]
extern crate clishe;
use ::clishe::prelude::*;

fn main() {
    let mut ctx = Context("".to_owned());
    // The same static methods available to the ::clap::Clap trait are available here. If you have
    // a vector of arguments, just use `parse_from()`. If you want to capture the parsing errors
    // instead of letting clap print them and exit them, you should use `try_parse()`
    //                      vvvvv
    if let Err(err) = Food::parse().run(&mut ctx) {
        // ^^^^^^^^ We ignore the Ok(_) scenario here since Returned is a useless unit struct,
        // but this is where we would handle it if the returned value was meaningful.
        eprint!("error: {}", err);
    }
}

// Could also be called Database, State, ... depending on the domain of your CLI. This is the
// single object available in commands aside from arguments/options. The context has two likely
// lifetimes:
//
//  - Created right before, handed to this command and dies with this command.
//  - Created at the beginning of the shell, passed from one command to the next.
pub struct Context(String);

// Could be anything. This turns the cli app into a function(Context, args from user) = Returned.
//
// This type offers us two approaches for our CLI apps: procedural and functional in nature. In the
// first one, one would apply side-effects inside of the application tree. On the other hand, one
// could aggregate functionally all side-effects to the Returned type and execute them at the scope
// of the main.
pub struct Returned;

// Dispatchers are commands which hold sub-commands. The root of a cli-like application is often a
// dispatcher. The application then takes the shape of a tree of dispatcher nodes, with commands!{}
// implementations as leaves.
dispatchers! {
    Food(self, _: &mut Context) -> Result<Returned> [
        Veggies: veggies::Veggies,
        Meat: meat::Meat,
        // The shell command comes with the clishe library. It usually takes a dispatcher and
        // starts a shell using the rustyline library in which all sub-commands of the dispatcher
        // are available as first-level commands. From the point-of-view of the user of the binary,
        // it will look something like this:
        //
        //     $ cargo run --example complete shell
        //     > veggies lettuce friend
        //     Welcome to the table, friend
        //     > 
        #[clap(alias = "sh", about = "The subcommands of this command in a shell")]
        Shell: Shell<Context, Returned, Food>,
    ],
}

mod veggies {
    use ::clishe::prelude::*;

    // All dispatchers are created equal, they could all be used as the root of an app.
    dispatchers! {
        // All clap macro attributes available on top of clap commands can be used here.
        #[clap(about = "Welcome to the Jungle")]
        // The name under which commands are declared inside a dispatcher is the name that is used.
        // vvvv This is just the name of the structure inside of the program. Same for dispatchers.
        Veggies(self, _: &mut crate::Context) -> Result<crate::Returned> [
            Carrots: Carrots,
            Lettuce: Lettuce,
        ],
    }

    // These are clap commands, they contain concrete command implementations. They are used as
    // leaves under the dispatchers. They could also be used as the root of the application, making
    // most of the point of the framework moot!
    commands! {
        Carrots(self, _ctx: &mut crate::Context) -> Result<crate::Returned> {
            Ok(crate::Returned)
        } struct {
            // All clap macro attributes available on top of clap command fields can be used here.
            #[clap(short, long)]
            name: Option<String>,
        },

        // The return type must be the same for every command and dispatcher in the command
        // hierarchy.                                      vvvvvvvvvvvvvvv
        Lettuce(self, _ctx: &mut crate::Context) -> Result<crate::Returned> {
            println!("Welcome to the table, {}", self.name.as_ref().map(|s| {
                s.as_ref()
            }).unwrap_or("unknown"));
            Ok(crate::Returned)
        } struct {
            name: Option<String>,
        },
    }
}

mod meat {
    use ::clishe::prelude::*;

    dispatchers! {
        // Overriding the command name at this level is not going to work.
        #[clap(name = "carne", about = "Aimez la viande, mangez-en mieux")]
        Meat(self, _: &mut crate::Context) -> Result<crate::Returned> [
            // All sub-commands' clap attributes are shadowed by
            // attributes applied at higher levels in the command hierarchy.
            #[clap(about = "Le boeuf. C'est ça qu'on mange au dîner")]
            // If you want to override the name of a command, do it here.
            Boeuf: Beef,
            // Or you could also use the clap macro attribute. With different abouts.
            #[clap(name = "vaca", about = "Vaca. Lo que vamos a comer en la cena")]
            Beef: Beef,
        ],
    }

    commands! {
        // The "about" override here and the "name" override in the Meat dispatcher will combine.
        #[clap(about = "Beef. It's What for Dinner")]
        Beef(self, ctx: &mut crate::Context) -> Result<crate::Returned> {
            // All fields are available as owned in here vvvvvvvvv
            ctx.0 = format!("Welcome to the table, {}!", self.name);
            Ok(crate::Returned)
        } struct {
            name: String,
        },
    }
}
```

This code will provide you with the following program:

```
$ cargo run --example complete
clishe

USAGE:
    complete <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    meat       Aimez la viande, mangez-en mieux
    shell      The subcommands of this command in a shell
    veggies    Welcome to the Jungle
```

You can also invoke the shell and "enter" the cli:

```
$ cargo run --example complete shell
> veggies lettuce friend
Welcome to the table, friend
> 
```
