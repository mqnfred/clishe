#[macro_export]
macro_rules! hybrids {
    (
        $(
            $name:ident(
                $state:ident: &mut $state_type:ty,
                $matches:ident: &ArgMatches
            ) -> Result<()> $body:block dispatches [
                $($subcmd:ident,) *
            ] using $app:expr,
        ) *
    ) => {
        $(
            pub struct $name {
                subs: ::std::collections::BTreeMap<String, Box<dyn ::modularcli::Command<$state_type>>>,
            }

            impl Default for $name {
                fn default() -> Self {
                    type Command = Box<dyn ::modularcli::Command<$state_type>>;
                    let mut subs = ::std::collections::BTreeMap::<String, Command>::default();
                    $({
                        let sub = $subcmd::default();
                        subs.insert(sub.app().get_name().to_owned(), Box::new(sub));
                    })*
                    Self{subs}
                }
            }

            impl ::modularcli::Command<$state_type> for $name {
                fn execute(
                    &mut self,
                    $state: &mut $state_type,
                    $matches: &ArgMatches,
                ) -> Result<()> {
                    if $matches.subcommand_name().is_some() {
                        let app = self.app();
                        ::modularcli::dispatch(&mut self.subs, $state, $matches, app)
                    } else $body
                }

                fn app<'a>(&self) -> App<'a> {
                    let mut app = $app;
                    for (_, subcmd) in &self.subs {
                        app = app.subcommand(subcmd.app());
                    }
                    app
                }
            }
        )*
    }
}
