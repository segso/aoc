use std::path::PathBuf;

use crate::{day::Day, environment::Environment};

/// A representation of an advent of code challenge.
pub trait Challenge {
    /// Should provide the [`Day`] of the challenge.
    fn day(&self) -> Day;

    /// Runs the challenge.
    ///
    /// It returns the result of the challenge as a `Vec` of `String`s.
    ///
    /// The input is commonly getted from a plain-text file in the `input` directory.
    /// See [path](#method.path) for more information.
    fn run(&self, input: String) -> Vec<String>;

    /// Returns the [`PathBuf`] of the input file.
    ///
    /// Takes an [`Environment`] to be able to handle multiple input files.
    ///
    /// By default, it returns the path of a plain-text file with the name
    /// `{environment}_{day}.txt` inside the `input` directory.
    fn path(&self, environment: Environment) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        path.push("input");
        path.push(format!("{}_{}.txt", environment.to_string(), *self.day()));

        path
    }
}
