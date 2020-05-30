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
            ::modularcli::paste::item! {
                #[derive(Clap)]
                $(#[$meta])*
                pub struct $name {
                    #[clap(subcommand)]
                    __subs: [< $name C o m m a n d s >],
                }
            }

            ::modularcli::paste::item! {
                #[derive(Clap)]
                enum [< $name C o m m a n d s >] {
                    $(
                        $(#[$sub_meta])* $sub_name($sub_ty),
                    )*
                }
            }

            ::modularcli::paste::item! {
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
