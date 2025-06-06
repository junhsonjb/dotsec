use clap::{CommandFactory, Parser};
use dotsec::cli::Cli;

fn main() {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        command.run().expect("issue running command");
    } else {
        Cli::command()
            .print_help()
            .expect("issue printing command help");
    }
}
