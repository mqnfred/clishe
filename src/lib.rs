pub extern crate paste;

#[macro_use]
mod commands;
#[macro_use]
mod dispatchers;

pub mod prelude {
    pub use ::anyhow::{Error,Result};
    pub use ::clap::Clap as _;
    pub use crate::Command;
    pub use crate::Shell;
}

pub trait Command<C, R> {
    fn run(self, ctx: &mut C) -> ::anyhow::Result<R>;
}

pub struct Shell<C, R, A: ::clap::Clap + Command<C, R>> {
    _phda: ::std::marker::PhantomData<A>,
    _phdc: ::std::marker::PhantomData<C>,
    _phdr: ::std::marker::PhantomData<R>,
}
mod shell;
