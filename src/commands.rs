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
