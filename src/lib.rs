#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    process::{Command, Stdio},
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn process_args(mut args: Vec<String>) -> Result<(), GiError> {
    if args.is_empty() {
        return Err(GiError::NoArgs);
    }

    if !args[0].starts_with('t') {
        return Err(GiError::BadPrefix);
    }

    args[0].remove(0);

    if args[0].is_empty() {
        args.remove(0);
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .is_err()
    {
        return Err(GiError::StdoutSet);
    }

    println!(
        "Running `git{}{}`...",
        if args.is_empty() { "" } else { " " },
        args.join(" ")
    );

    if stdout.reset().is_err() {
        return Err(GiError::StdoutReset);
    }

    stdout.reset().expect("Unable to reset stdout color!");

    println!();

    if Command::new("git")
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .is_err()
    {
        return Err(GiError::GitFail);
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub enum GiError {
    BadPrefix,
    GitFail,
    NoArgs,
    StdoutReset,
    StdoutSet,
}

impl GiError {
    pub fn print(self) {
        StandardStream::stderr(ColorChoice::Always)
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .expect("Unable to set stderr color!");

        println!("{}", self)
    }
}

impl Display for GiError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match &self {
                Self::BadPrefix => "Argument does not start with a `t`!",
                Self::GitFail => "Failed to run git!",
                Self::NoArgs => "No arguments provided!",
                Self::StdoutReset => "Unable to reset stdout color!",
                Self::StdoutSet => "Unable to set stdout color!,",
            }
        )
    }
}
