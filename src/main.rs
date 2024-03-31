use clap::Parser;
use anyhow::Result;

pub mod cli;

use crate::cli::*;

fn main() -> Result<()> {
    // Handle user input
    let _args = Cli::try_parse()?;

    main_menu(_args);

    Ok(())
}

