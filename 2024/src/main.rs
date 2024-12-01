use std::env;
use year2024::{run, Day, Environment};

const ENVIRONMENT_VAR: &str = "ENVIRONMENT";
const PRODUCTION_VALUE: &str = "PRODUCTION";
const LOCAL_VALUE: &str = "LOCAL";

fn main() {
    let mut args = env::args();

    // Skip binary path
    args.next();

    let day = match args.next() {
        None => parse_error("no day provided"),
        Some(day) => day.parse::<u8>().unwrap_or_else(|_| {
            parse_error(&format!(
                "day must be between 1 inclusive and 25 inclusive, got '{day}'"
            ))
        }),
    };

    let day = Day::from(day).unwrap_or_else(|err| parse_error(&err.to_string()));

    let environment = match env::var(ENVIRONMENT_VAR)
        .unwrap_or(LOCAL_VALUE.to_string())
        .as_str()
    {
        PRODUCTION_VALUE => Environment::Production,
        LOCAL_VALUE => Environment::Local,
        value => parse_error(&format!(
            "environment {value} is not valid, options are: {PRODUCTION_VALUE}, {LOCAL_VALUE}"
        )),
    };

    run(day, environment);
}

fn parse_error(message: &str) -> ! {
    println!("Error while parsing arguments: {message}");
    std::process::exit(1);
}
