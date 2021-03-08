#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    process::{Command, Stdio},
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

    transform_args(&mut args)?;

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
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
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
pub fn transform_args(args: &mut Vec<String>) -> GiResult {
    // expect some arguments (not just `gi`)
    if args.is_empty() {
        return Err(GiError::NoArgs);
    }

    // if the first argument doesn't start with a `t`, it's too far away to assume it should be git
    if !args[0].starts_with('t') {
        return Err(GiError::BadPrefix);
    }

    args[0].remove(0);

    if args[0].is_empty() {
        args.remove(0);
    }

    Ok(())
}

type GiResult = Result<(), GiError>;

/// An enum to describe different errors that could occur while executing `gi_t`.
#[derive(Debug, Clone)]
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
