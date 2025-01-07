use crate::config::root::Args;
use figment::value::{Dict, Map};
use figment::{Error, Metadata, Provider};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CliArgs {
    args: Args,
    profile: figment::Profile,
}

impl Display for CliArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CLI args")
    }
}

impl CliArgs {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            profile: figment::Profile::Default,
        }
    }
}

macro_rules! dict {
    ($args:expr, $($k:tt),* $(,)?) => ({
        let mut dict = Dict::new();

        $(if let Some(v) = $args.$k.as_ref() {
            dict.insert(stringify!($k).to_owned(), v.to_string().parse().unwrap());
        })*

        dict
    });
}

impl Provider for CliArgs {
    fn metadata(&self) -> Metadata {
        Metadata::from("program argument(s)", "command-line")
    }

    fn data(&self) -> Result<Map<figment::Profile, Dict>, Error> {
        let dict = dict!(self.args, port);

        Ok(self.profile.collect(dict))
    }
}
