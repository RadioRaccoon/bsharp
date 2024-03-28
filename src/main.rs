use clap::Parser;
use std::{io, env, fs, process::ExitCode};
use anyhow::{Context, Result};

#[derive(Debug, Parser)]
struct Cli {
    mode: String,
    path: std::path::PathBuf,
}

// Credit to Alefith for the bishop design
const WELCOME_MSG: &'static str = 
"
                                      _O
                                     / //\\
  _         _                       {     }
 | |       | |                       \\___/
 | |__  ___| |__   __ _ _ __ _ __    (___)
 | '_ \\/ __| '_ \\ / _` | '__| '_ \\    |_|
 | |_) \\__ \\ | | | (_| | |  | |_) |  /   \\
 |_.__/|___/_| |_|\\__,_|_|  | .__/  (_____)
                            | |    (_______)
                            |_|    /_______\\
";

fn main() -> Result<()> {
    // Handle user input
    let _args = Cli::try_parse()?;

    main_menu(_args);


    Ok(())
}

fn main_menu(_input: Cli) {
    println!("{}", WELCOME_MSG);
    println!(
        "
Welcome to bsharp! Select an option below:
[1] Load Game
[2] Sandbox
[3] Exit");
}
