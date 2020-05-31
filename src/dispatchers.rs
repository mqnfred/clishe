/// Defines a DSL to introduce subcommand dispatcher commands.
///
/// All commands which have children in the command hierarchy are dispatchers. This means they do
/// not implement any custom logic, they just contain 1 or more subcommands (they can contain 0 but
/// that would be pointless, as place-holders maybe.)
///
/// The DSL is as follows:
///
/// ```ignore
/// use ::clishe::prelude::*;
/// pub struct Context(u64);
/// dispatchers! {
/// //    vvvv any clap attribute available on clap commands are available here
///     #[clap(author = "Louis Feuvrier <mqnfred@gmail.com>")]
/// //  vvvv name of the command in the code
///     CRUD(self
/// //       ^^^^ same as for the commands! macro rule
///
/// //             v fixed, these dispatcher commands cannot modify the context
///     CRUD(self, _: &mut crate::Context
/// //                ^^^^ mutability has been decided already, this is mandatory
///
/// //         ::anyhow::Result, from prelude vvvvvv     v attention, square brackets
///     CRUD(self, _: &mut crate::Context) -> Result<()> [
/// //          return type of the command hierarchy ^^
/// //        vvvv any clap attribute available on clap commands are available here
///         #[clap(about = "storing of integers in contexts")]
/// //        ^^^^ those clap attributes combine. this one gets priority on the other one
/// //      vvvvv default name of the command
///         Store: store::Store,
/// //             ^^^^^^^^^^^^ path to the object implementing ::clishe::Command
///     ],
/// //  ^^ square bracket, with a mandatory comma
/// }
/// ```
#[macro_export]
macro_rules! dispatchers {
    (
        $(
            $(#[$meta:meta])?
            $name:ident(self, _: &mut $context_ty:ty) -> Result<$ret_ty:ty> [
                $($(#[$sub_meta:meta])? $sub_name:ident: $sub_ty:ty,)*
            ],
        )*
    ) => {
        $(
            ::clishe::paste::item! {
                #[derive(Clap)]
                $(#[$meta])*
                pub struct $name {
                    #[clap(subcommand)]
                    __subs: [< $name C o m m a n d s >],
                }
            }

            ::clishe::paste::item! {
                #[derive(Clap)]
                enum [< $name C o m m a n d s >] {
                    $(
                        $(#[$sub_meta])* $sub_name($sub_ty),
                    )*
                }
            }

            ::clishe::paste::item! {
                impl Command<$context_ty, $ret_ty> for $name {
                    fn run(self, ctx: &mut $context_ty) -> Result<$ret_ty> {
                        match self.__subs {
                            $([< $name C o m m a n d s >]::$sub_name(sub) => sub.run(ctx),)*
                        }
                    }
                }
            }
        )*
    }
}
