#[macro_export]
macro_rules! dispatchers {
    (
        $(
            $name:ident(
                _: &mut $state_type:ty,
                _: &ArgMatches
            ) -> Result<()> [
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
                    state: &mut $state_type,
                    matches: &ArgMatches,
                ) -> Result<()> {
                    let app = self.app();
                    ::modularcli::dispatch(&mut self.subs, state, matches, app)
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

pub fn dispatch<S>(
    subs: &mut ::std::collections::BTreeMap<String, Box<dyn crate::Command<S>>>,
    state: &mut S,
    matches: &::clap::ArgMatches,
    mut app: ::clap::App,
) -> ::anyhow::Result<()> {
    match matches.subcommand() {
        (name, Some(matches)) => match subs.get_mut(name) {
            Some(cmd) => cmd.execute(state, matches),
            None => Err(::anyhow::Error::msg(format!(
                "no subcommand {} in {}",
                name,
                stringify!($name),
            ))),
        }
        _ => app.print_help().map_err(|err| { ::anyhow::Error::msg(err.to_string()) }),
    }
}
