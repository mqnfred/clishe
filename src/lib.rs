//! Clishé is a cli mini-framework in rust.
//!
//! Write your CLI using an idiom focused on the following values:
//!
//!  1. Reduction of boilerplate for repetitive command declaration
//!  2. Command implementation and definition live side-to-side
//!  3. Control-flow skeleton around args, mutable state (context), return type
//!  4. We should be able to generate shells on-demand based on a cli
//!  5. Auto-complete is a first-class citizen, in and out of the inner shell [wip]
//!
//! When I work on my rust ecosystem, I sometimes find it necessary to "try out a
//! piece" by calling some specific endpoint or library API. It should be trivial
//! to create small, re-usable clis for these purposes. This is what clishé aims to
//! achieve by being opinionated and limiting scaffolding.
//!
//! Clishé is a surprisingly thin wrapper around the following technologies:
//!
//!  - `anyhow` for error handling
//!  - `clap` for cli building
//!  - `rustyline`/`shellwords` for shell generation
//!
//! Those libraries contribute most of its power to this framework, which is not
//! much more than a collection of technologies, a thin
//! [DSL](https://en.wikipedia.org/wiki/Domain-specific_language) and an opinion.
//!
//! ## Example
//!
//! This is a simple app with dummy commands to display the spirit of clishe.
//! how to use this framework.
//!
//! ```
//! #[macro_use]
//! extern crate clap;
//! #[macro_use]
//! extern crate clishe;
//! use ::clishe::prelude::*;
//!
//! fn main() {
//!     let mut ctx = Context("".to_owned());
//!     // The same static methods available to the ::clap::Clap trait are
//!     // available here. If you have a vector of arguments, just use
//!     // `parse_from()`. If you want to capture the parsing errors instead of
//!     // letting clap print them and exit them, you should use `try_parse()`
//!     //                      vvvvv
//!     if let Err(err) = Food::parse().run(&mut ctx) {
//!         // ^^^^^^^^ We ignore the Ok(_) scenario here since Returned is a
//!         // useless unit struct, but this is where we would handle it if the
//!         // returned value was meaningful.
//!         eprint!("error: {}", err);
//!     }
//! }
//!
//! // Could also be called Database, State, ... depending on the domain of your
//! // CLI. This is the single object available in commands aside from
//! // arguments/options. The context has two likely lifetimes:
//! //
//! //  - Created right before, handed to this command and dies with this command
//! //  - Created at the beginning of the shell, passed from one command to another
//! pub struct Context(String);
//!
//! // Could be anything. This turns the cli app into a function(Context, args from
//! // user) = Returned.
//! //
//! // This type offers us two approaches for our CLI apps: procedural and
//! // functional in nature. In the first one, one would apply side-effects inside
//! // of the application tree. On the other hand, one could aggregate functionally
//! // all side-effects to the Returned type and execute them at the scope of the
//! // main.
//! pub struct Returned;
//!
//! // Dispatchers are commands which hold sub-commands. The root of a cli-like
//! // application is often a dispatcher. The application then takes the shape of a
//! // tree of dispatcher nodes, with commands!{} implementations as leaves.
//! dispatchers! {
//!     // Any of the clap attributes you would use on top of the main clap app,
//!     // we will use here on the Food dispatcher, as we have chosen it to be the
//!     // root of our cli application.
//!     #[clap(
//!         name = "clishe",
//!         version = "0.2.0",
//!         about = "Food market",
//!         before_help = "In case you're hungry, here is a",
//!         after_help = "For you",
//!     )]
//!     Food(self, _: &mut Context) -> Result<Returned> [
//!         Veggies: veggies::Veggies,
//!         Meat: meat::Meat,
//!         // The shell command comes with the clishe library. It usually takes a
//!         // dispatcher and starts a shell using the rustyline library in which
//!         // all sub-commands of the dispatcher are available as first-level
//!         // commands. From the point-of-view of the user of the binary, it will
//!         // look something like this:
//!         //
//!         //     $ cargo run --example complete shell
//!         //     > veggies lettuce friend
//!         //     Welcome to the table, friend
//!         //     >
//!         #[clap(alias = "sh", about = "Subcommands of this in a shell")]
//!         Shell: Shell<Context, Returned, Food>,
//!     ],
//! }
//!
//! mod veggies {
//!     use ::clishe::prelude::*;
//!
//!     // All dispatchers are created equal, they
//!     // could all be used as the root of an app.
//!     dispatchers! {
//!         // All clap macro attributes available on
//!         // top of clap commands can be used here.
//!         #[clap(about = "Welcome to the Jungle")]
//!         // The name under which commands are declared inside a dispatcher is
//!         // the name that is used. This is just the name of the structure
//!         // vvvv inside of the program. Same for dispatchers.
//!         Veggies(self, _: &mut crate::Context) -> Result<crate::Returned> [
//!             Carrots: Carrots,
//!             Lettuce: Lettuce,
//!         ],
//!     }
//!
//!     // These are clap commands, they contain concrete command implementations.
//!     // They are used as leaves under the dispatchers. They could also be used
//!     // as the root of the application, making most of the point of the
//!     // framework moot!
//!     commands! {
//!         Carrots(self, _ctx: &mut crate::Context) -> Result<crate::Returned> {
//!             Ok(crate::Returned)
//!         } struct {
//!             // All clap macro attributes available on top
//!             // of clap command fields can be used here.
//!             #[clap(short, long)]
//!             name: Option<String>,
//!         },
//!
//!         // The return type must be the same for every command and dispatcher
//!         // in the command hierarchy.                       vvvvvvvvvvvvvvv
//!         Lettuce(self, _ctx: &mut crate::Context) -> Result<crate::Returned> {
//!             println!("Welcome to the table, {}", self.name.as_ref().map(|s| {
//!                 s.as_ref()
//!             }).unwrap_or("unknown"));
//!             Ok(crate::Returned)
//!         } struct {
//!             name: Option<String>,
//!         },
//!     }
//! }
//!
//! mod meat {
//!     use ::clishe::prelude::*;
//!
//!     dispatchers! {
//!         // Overriding the command name at this level is not going to work.
//!         #[clap(name = "carne", about = "Aimez la viande, mangez-en mieux")]
//!         Meat(self, _: &mut crate::Context) -> Result<crate::Returned> [
//!             // All sub-commands' clap attributes are shadowed by
//!             // attributes applied at higher levels in the command hierarchy.
//!             #[clap(about = "Le boeuf. C'est ça qu'on mange")]
//!             // If you want to override the name of a command, do it here.
//!             Boeuf: Beef,
//!             // You could use the clap macro attribute. With different abouts.
//!             #[clap(name = "vaca", about = "Vaca. Lo que vamos a comer")]
//!             Beef: Beef,
//!         ],
//!     }
//!
//!     commands! {
//!         // The "about" override here and the "name"
//!         // override in the Meat dispatcher will combine.
//!         #[clap(about = "Beef. It's What for Dinner")]
//!         Beef(self, ctx: &mut crate::Context) -> Result<crate::Returned> {
//!             // All fields are available as owned in here vvvvvvvvv
//!             ctx.0 = format!("Welcome to the table, {}!", self.name);
//!             Ok(crate::Returned)
//!         } struct {
//!             name: String,
//!         },
//!     }
//! }
//! ```

/// Required for commands-generation.
pub extern crate paste;

#[macro_use]
mod commands;
#[macro_use]
mod dispatchers;

/// Import this to use the `commands!` and `dispatchers!` macro rules.
///
/// This prelude contains all the generic types required by the aforementioned macros. For the sake
/// of brevity in the macro rules DSL, we choose to bring those objects into scope in the modules
/// declaring and defining commands.
pub mod prelude {
    pub use crate::commands;
    pub use crate::dispatchers;
    pub use crate::Command;
    #[cfg(feature = "shell")]
    pub use crate::Shell;
    pub use anyhow::{Error, Result};
    pub use clap::Clap;
}

/// The trait implemented by all command types in the hierarchy.
///
/// Both dispatcher, custom commands and shell commands implement this trait, this is what allows
/// them to be bound together. You can implement these commands manually if you need to for some
/// reason, but the main point of the library lies with the `commands!` and `dispatchers!` macro
/// rules, which spawn objects implementing this `Command` trait.
pub trait Command<C, R> {
    fn run(self, ctx: &mut C) -> ::anyhow::Result<R>;
}

/// A command that spawns a shell of the provided dispatcher type.
///
/// The shell command spawns a shell using rustyline and shellwords libraries. The type arguments
/// it takes are straightforward:
///
///  - Context type (state)
///  - Return type (same as command hierarchy)
///  - Dispatcher to create a shell for (can be self-referential)
///
/// # Example
///
/// ```ignore
/// # #[macro_rules] extern crate clap;
/// # #[macro_rules] extern crate clishe;
/// # use clishe::prelude::*;
/// dispatchers! {
///     Food(self, _: &mut u64) -> Result<()> [
///         Shell: clishe::Shell<u64, (), Food>,
///     ],
/// }
/// ```
#[cfg(feature = "shell")]
pub struct Shell<C, R, A: ::clap::Clap + Command<C, R>> {
    _phda: ::std::marker::PhantomData<A>,
    _phdc: ::std::marker::PhantomData<C>,
    _phdr: ::std::marker::PhantomData<R>,
}
#[cfg(feature = "shell")]
mod shell;
