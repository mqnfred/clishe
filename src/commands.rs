/// Defines a DSL for implementing a leaf in our command hierarchy.
///
/// Those leaf commands implement concrete logic which can change the state of the context, return
/// a desired output, or trigger any kind of side-effect whatsoever. They contain the meat of our
/// cli. They can be attached to dispatchers or parsed directly, as any `::clap::App` can.
///
/// The DSL is as follows:
///
/// ```ignore
/// use ::clishe::prelude::*;
/// pub struct Context(u64);
/// commands! {
/// //    vvvv any clap attribute available on clap commands are available here
///     #[clap(author = "Louis Feuvrier <mqnfred@gmail.com>")]
/// //  vvvvv name of the command in the code
///     Store(self,
/// //        ^^^^ fixed, points to the clap structure containing the arguments
///
/// //              vvv can be any identifier, but not _ itself, if unused put _ctx
///     Store(self, ctx: &mut
/// //                   ^^^^ mutability has been decided already, this is mandatory
///
/// //                        vvvvvvvvvvvvvv any path to a structure of any kind if valid
///     Store(self, ctx: &mut crate::Context) -> Result
/// //                                           ^^^^^^ ::anyhow::Result, from prelude
///
/// //             return type of the command hierarchy vv  v curly brackets
///     Store(self, ctx: &mut crate::Context) -> Result<()> {
///         Ok(ctx.0 = self.amount) // do anything to context
/// //      ^^^  return whatever  ^
///     } struct { // mandatory
/// //        vvvv any clap attribute available on clap command fields are available here
///         #[clap(about = "The number to store in our context")]
///         number: String,
/// //      ^^^^^^^^^^^^^^ see clap derive library for more context on how types are interpreted
///     },
///     ^^ curly brackets, with a mandatory comma
/// }
/// ```
#[macro_export]
macro_rules! commands {
    (
        $(
            $(#[$meta:meta])?
            $name:ident(
                $self:ident,
                $context:ident: &mut $context_ty:ty
            ) -> Result<$ret_ty:ty> $body:block struct {
                $($(#[$sub_meta:meta])? $field_name:ident: $field_ty:ty,)*
            },
        ) *
    ) => {
        $(
            #[derive(Clap)]
            $(#[$meta])*
            pub struct $name {
                $($(#[$sub_meta])* $field_name: $field_ty,)*
            }

            impl $name {
                pub fn run($self, $context: &mut $context_ty) -> Result<$ret_ty> $body
            }
        ) *
    }
}
