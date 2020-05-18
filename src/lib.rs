#[macro_use]
mod commands;
#[macro_use]
mod dispatchers;

pub mod prelude {
    pub use ::anyhow::{Error,Result};
    pub use ::clap::{App,Arg,ArgMatches};
    pub use ::clap::Clap as _;
    pub use crate::Command;
}

pub trait Command<S, R> {
    fn run(&self, state: &mut S) -> ::anyhow::Result<R>;
}
