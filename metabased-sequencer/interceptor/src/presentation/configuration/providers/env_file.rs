use figment::util::nest;
use figment::value::{Dict, Map};
use figment::{Error, Metadata, Provider};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct EnvFile<'a> {
    path: Option<PathBuf>,
    prefix: &'a str,
    profile: figment::Profile,
}

impl<'a> Display for EnvFile<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ENV file {:?} in CWD or parent", self.path())
    }
}

impl<'a> EnvFile<'a> {
    pub fn new() -> Self {
        Self {
            path: None,
            profile: figment::Profile::Default,
            prefix: "",
        }
    }

    pub fn from_filename(filename: &str) -> Self {
        Self {
            path: Some(PathBuf::from(filename)),
            profile: figment::Profile::Default,
            prefix: "",
        }
    }

    pub fn with_prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = prefix;
        self
    }

    fn path(&self) -> PathBuf {
        self.path.clone().unwrap_or(PathBuf::from("localnet.env"))
    }

    fn key(&self, k: String) -> String {
        k.trim_start_matches(self.prefix).to_lowercase()
    }
}

impl<'a> Provider for EnvFile<'a> {
    fn metadata(&self) -> Metadata {
        Metadata::from(
            format!("`{}` environment variable(s)", self.prefix),
            self.path().display().to_string(),
        )
    }

    fn data(&self) -> Result<Map<figment::Profile, Dict>, Error> {
        let attributes_from_file =
            dotenv::from_filename_iter(self.path()).map_err(|v| Error::from(v.to_string()))?;

        let mut dict = Dict::new();

        for result in attributes_from_file {
            let (k, v) = result.unwrap();

            let nested_dict = nest(self.key(k).as_str(), v.parse().expect("infallible"))
                .into_dict()
                .expect("key is non-empty: must have dict");

            for (k, v) in nested_dict {
                dict.insert(k, v);
            }
        }

        Ok(self.profile.collect(dict))
    }
}
