use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    mode: String,
    path: std::path::PathBuf,
}

pub enum MenuOptions {
    LoadGame(u8),
    Sandbox(u8),
    Exit(u8),
}

pub trait Display {
    fn execute(&self) -> &str;
    fn undo(&self) -> &str;
}

pub struct UpdateBoard;
impl Display for UpdateBoard {
    fn execute(&self) -> &str {
        "create table"
    }
    fn undo(&self) -> &str {
        "undo table"
    }
}

pub struct CommandLineInterface {
    commands: Vec<Box<dyn Display>>,
}

impl CommandLineInterface {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_display(&mut self, cmd: Box<dyn Display>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }

    fn undo(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| cmd.undo())
            .collect()
    }
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

pub fn main_menu(_input: Cli) {
    println!("{}", WELCOME_MSG);
    println!(
        "
Welcome to bsharp! Select an option below:
[1] Load Game
[2] Sandbox
[3] Exit");
    let mut cli = CommandLineInterface::new();

    let cmd = Box::new(UpdateBoard);
    cli.add_display(cmd);

    assert_eq!(vec!["create table"], cli.execute());
    assert_eq!(vec!["undo table"], cli.undo());
}
