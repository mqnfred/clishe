#[macro_export]
macro_rules! hybrids {
    (
        $(
            $name:ident(
                &$self:ident,
                $state:ident: &mut $state_ty:ty
            ) -> Result<$ret_ty:ty> $body:block dispatches [
                $(#[$meta:meta] $sub_name:ident: $sub:ty,)*
            ]
        )*
    ) => {
        $(
            ::paste::item! {
                #[derive(Clap)]
                pub struct $name {
                    #[clap(subcommand)]
                    subs: [< $name C o m m a n d s >],
                }
            }

            ::paste::item! {
                #[derive(Clap)]
                enum [< $name C o m m a n d s >] {
                    $(
                        #[$meta]
                        $sub_name($sub),
                    )*
                }
            }

            ::paste::item! {
                impl $name {
                    pub fn run(&self, state: &mut $state_ty) -> Result<()> {
                        match &self.subs {
                            $([< $name C o m m a n d s >]::$sub_name(sub) => sub.run(state),)*
                        }
                    }
                }
            }
        )*
    }
}
