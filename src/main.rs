use std::{
    env,
    process::{self, Command, Stdio},
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    stderr
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .expect("Unable to set stderr color!");

    if args.is_empty() {
        eprintln!("No arguments provided!");
        process::exit(1);
    }

    if !args[0].starts_with('t') {
        eprintln!("Argument does not start with a `t`!");
        process::exit(1);
    }

    args[0].remove(0);

    if args[0].is_empty() {
        args.remove(0);
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .expect("Unable to set stdout color!");

    println!(
        "Running `git{}{}`...",
        if !args.is_empty() { " " } else { "" },
        args.join(" ")
    );

    stdout.reset().expect("Unable to reset stderr color!");

    println!("");

    Command::new("git")
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to run git!");
}
