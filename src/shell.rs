use ::rustyline::config::Configurer;

impl<C, R, A> ::clap::Parser for crate::Shell<C, R, A>
    where A: ::clap::Parser + crate::Command<C, R>
{}

impl<C, R, A> ::clap::Args for crate::Shell<C, R, A>
    where A: ::clap::Parser + crate::Command<C, R>
{
    fn augment_args<'b>(cmd: ::clap::Command<'b>) -> ::clap::Command<'b> {
        cmd
    }

    fn augment_args_for_update<'b>(cmd: ::clap::Command<'b>) -> ::clap::Command<'b> {
        cmd
    }
}

impl<C, R, A> ::clap::IntoApp for crate::Shell<C, R, A>
    where A: ::clap::Parser + crate::Command<C, R>,
{
    fn into_app<'b>() -> ::clap::App<'b> {
        // TODO: the app should feature a clap `about`, but it does not look
        // like this one is being forwarded/considered by the parent
        ::clap::App::new("shell").about("Try out this CLI in a shell!")
    }

    fn into_app_for_update<'b>() -> ::clap::App<'b> {
        Self::into_app()
    }
}

impl<C, R, A> ::clap::FromArgMatches for crate::Shell<C, R, A>
    where A: ::clap::Parser + crate::Command<C, R>,
{
    fn from_arg_matches(_matches: &::clap::ArgMatches) -> Result<Self, ::clap::Error> {
        Ok(Self{
            _phda: ::std::marker::PhantomData::<A>,
            _phdc: ::std::marker::PhantomData::<C>,
            _phdr: ::std::marker::PhantomData::<R>,
        })
    }

    fn update_from_arg_matches(
        &mut self,
        _matches: &::clap::ArgMatches,
    ) -> Result<(), ::clap::Error> {
        Ok(())
    }
}

impl<C, R, A> crate::Command<C, R> for crate::Shell<C, R, A>
    where A: ::clap::Parser + crate::Command<C, R>,
{
    fn run(self, ctx: &mut C) -> ::anyhow::Result<R> {
        let mut rl = ::rustyline::Editor::<()>::new()?;
        rl.set_completion_type(::rustyline::CompletionType::List);
        rl.set_edit_mode(::rustyline::EditMode::Vi);

        let mut last_res = Err(::anyhow::Error::msg("no result available (no command ran)"));
        loop {
            // generate prompt, parse args
            let args = match ::shellwords::split(rl.readline("> ")?.trim_end()) {
                Err(_) => { eprintln!("mismatched quotes"); continue },
                Ok(args) => args,
            };

            // exit or run command
            if is_asking_to_exit(&args) {
                break last_res; // exit
            } else {
                // TODO: have to prepend with "shell" because i do not
                // find the NoBinaryName setting in the new clap v3 beta...
                // the main problem here is that "shell" appears in the help message
                let args = ::std::iter::once("shell".to_owned()).chain(args.into_iter());
                match A::try_parse_from(args) {
                    Err(err) => eprintln!("{}", err), // TODO: better error printing
                    Ok(app) => {
                        last_res = match app.run(ctx) {
                            Err(err) => { eprintln!("{}", err); Err(err) }
                            Ok(res) => Ok(res),
                        }
                    }
                }
            }
        }
    }
}

fn is_asking_to_exit(args: &Vec<String>) -> bool {
    args.len() == 1 && (args[0] == "exit" || args[0] == "quit" || args[0] == "q")
}
