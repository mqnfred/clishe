#[macro_use]
mod commands;
#[macro_use]
mod dispatchers;
//#[macro_use]
//mod hybrids;

pub mod prelude {
    pub use ::anyhow::{Error,Result};
    pub use ::clap::{App,Arg,ArgMatches};
    pub use ::clap::Clap as _;
    pub use crate::Command as _;
    pub use crate::execute_command;
}

pub fn execute_command<S>(
    entry_point: &mut dyn Command<S>,
    state: &mut S,
    args: &Vec<String>,
) -> ::anyhow::Result<()> {
    let app = entry_point.app().setting(::clap::AppSettings::NoBinaryName);
    entry_point.execute(state, &app.try_get_matches_from(args).map_err(|err| {
        ::anyhow::Error::msg(err.to_string())
    })?)
}

pub trait Command<S> {
    fn execute(&mut self, state: &mut S, matches: &::clap::ArgMatches) -> ::anyhow::Result<()>;
    fn app<'a>(&self) -> ::clap::App<'a>;
}

pub trait Cmd<S, R> {
    fn run(&self, state: &mut S) -> ::anyhow::Result<R>;
}
