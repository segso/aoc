/// Enum useful for executing different code based on if its running in a
/// production or a local environment.
#[derive(Clone, Copy)]
pub enum Environment {
    Production,
    Local,
}

impl ToString for Environment {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Production => "prod",
            Self::Local => "local",
        })
    }
}
