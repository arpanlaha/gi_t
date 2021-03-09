#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    process::Command,
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Corrects arguments passed to `gi_t` into the corresponding `git` command for execution.
///
/// # Arguments
/// * `args` - the arguments to the program in [`Vec`] form.
///
/// # Errors
/// If the function receives malformed arguments or runs into errors setting terminal colors or
/// spawning a child process, the corresponding [`GiError`] will be returned.
pub fn process_args(mut args: Vec<String>) -> Result<(), GiError> {
    // remove program name
    args.remove(0);

    // save original arguments
    let original = args.join(" ");

    let args = transform_args(args)?;

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .map_err(|_| GiError::StdoutSet)?;

    println!(
        "Correcting `gi {}` to `git{}{}`...",
        original,
        if args.is_empty() { "" } else { " " },
        args.join(" ")
    );

    stdout.reset().map_err(|_| GiError::StdoutReset)?;

    println!();

    Command::new("git")
        .args(&args)
        .status()
        .map_err(|_| GiError::GitFail)?;

    Ok(())
}

/// Transformed arguments passed to `gi_t` into the corresponding `git` arguments.
///
/// # Arguments
/// * `args` - the arguments to the program in [`Vec`] form.
///
/// # Errors
/// If the function receives malformed arguments, the corresponding [`GiError`] will be returned.
pub fn transform_args(mut args: Vec<String>) -> GiResult<Vec<String>> {
    // expect some arguments (not just `gi`)
    if args.is_empty() {
        return Err(GiError::NoArgs);
    }

    let first = &mut args[0];

    // if the first argument doesn't start with a `t`, it's too far away to assume it should be git
    if !first.starts_with('t') {
        return Err(GiError::BadPrefix);
    }

    first.remove(0);

    if first.is_empty() {
        args.remove(0);
    }

    Ok(args)
}

type GiResult<T> = Result<T, GiError>;

/// An enum to describe different errors that could occur while executing `gi_t`.
#[derive(Debug, Clone, PartialEq)]
pub enum GiError {
    /// The first argument does not begin with a `t`.
    BadPrefix,

    /// Spawning `git` child process results in a failure.
    GitFail,

    /// No arguments are passed to `gi_t`.
    NoArgs,

    /// A failure is encountered resetting stdout terminal color.
    StdoutReset,

    /// A failure is encountered setting stdout terminal color.
    StdoutSet,
}

impl GiError {
    /// A helper method to print the associated error message for a [`GiError`].
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

#[cfg(test)]
mod tests {
    use crate::{transform_args, GiError};

    #[test]
    fn no_args() {
        assert_eq!(transform_args(vec![]), Err(GiError::NoArgs));
    }

    #[test]
    fn no_t() {
        assert_eq!(transform_args(vec!["foo".into()]), Err(GiError::BadPrefix));
    }

    #[test]
    fn only_t() {
        assert_eq!(transform_args(vec!["t".into()]), Ok(vec![]));
    }

    #[test]
    fn simple_valid() {
        assert_eq!(
            transform_args(vec!["tstatus".into()]),
            Ok(vec!["status".into()])
        );
    }

    #[test]
    fn complex_valid() {
        assert_eq!(
            transform_args(vec![
                "tconfig".into(),
                "--global".into(),
                "name".into(),
                "Firstname Lastname".into()
            ]),
            Ok(vec![
                "config".into(),
                "--global".into(),
                "name".into(),
                "Firstname Lastname".into()
            ])
        );
    }
}
