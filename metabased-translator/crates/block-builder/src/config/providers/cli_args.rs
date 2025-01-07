use crate::config::root::Args;
use derive_more::Display;
use figment::value::{Dict, Map};
use figment::{Error, Metadata, Profile, Provider};

#[derive(Debug, Display)]
#[display("CLI args: {:?}", args)]
pub struct CliArgs {
    args: Args,
    profile: Profile,
}

impl CliArgs {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            profile: Profile::Default,
        }
    }
}

/// Creates a [`Dict`] from optional field values in a struct, where:
/// - Only `Some` values are included in the dictionary
/// - Keys are the field names as strings
/// - Values are parsed from the field values' string representation
///
/// Usage: dict!(struct_instance, field1, field2, field3)
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

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        let dict = dict!(self.args, port, chain_id, genesis_timestamp);

        Ok(self.profile.collect(dict))
    }
}
