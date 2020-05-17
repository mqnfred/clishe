#[macro_export]
macro_rules! commands {
    (
        $(
            $name:ident(
                $state:ident: &mut $state_type:ty,
                $matches:ident: &ArgMatches
            ) -> Result<()> $body:block using $app:expr,
        ) *
    ) => {
        $(
            #[derive(Default)]
            pub struct $name;

            impl ::modularcli::Command<$state_type> for $name {
                fn execute(
                    &mut self,
                    $state: &mut $state_type,
                    $matches: &ArgMatches,
                ) -> Result<()> $body

                fn app<'a>(&self) -> App<'a> {
                    $app
                }
            }
        )*
    }
}
