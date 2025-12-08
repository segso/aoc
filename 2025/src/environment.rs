use std::fmt::Display;

/// Enum useful for executing different code based on if its running in a
/// production or a local environment.
#[derive(Clone, Copy)]
pub enum Environment {
    Production,
    Local,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Production => "prod",
                Self::Local => "local",
            }
        )
    }
}
