use std::{
    env,
    process::{self, Command, Stdio},
};

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

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

    println!(
        "Running `git{}{}`...",
        if !args.is_empty() { " " } else { "" },
        args.join(" ")
    );

    Command::new("git")
        .args(&args)
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to run git!");
}
