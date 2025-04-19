use clap::{Args, CommandFactory, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Store a new secret
    Put(PutArgs),

    /// Retrieve a secret
    Get(GetArgs),

    /// List all secrets
    List,

    /// Delete a secret
    Delete(DeleteArgs),
}

#[derive(Debug, Args)]
struct PutArgs {
    /// Name of the secret to store
    key: String,

    /// Value of the secret to store
    value: String, // TODO: should this be a string?
}

#[derive(Debug, Args)]
struct GetArgs {
    /// Name of the secret to get
    key: String
}

#[derive(Debug, Args)]
struct DeleteArgs {
    /// Name of the secret to delete
    key: String,

    /// Delete must be run with either --dry-run or --force
    #[command(flatten)]
    delete_flags: DeleteFlags,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct DeleteFlags {
    /// Display the secret that would be deleted, without removing
    #[arg(long, short('n'), default_value_t = true)]
    dry_run: bool,

    /// Force actual deletion (required in order to delete)
    #[arg(long, short('f'))]
    force:bool,
}

impl Command {
    fn run(&self) {
        match self {
            Command::Put(args) => put(args),
            Command::Get(args) => get(args),
            Command::List => list(),
            Command::Delete(args) => delete(args),
        }
    }
}

fn put(_args: &PutArgs) {
    println!("put: do that thang!");
}

fn get(_args: &GetArgs) {
    println!("get: don't it make you wanna shout!");
}

fn list() {
    println!("the LAWD has been good to you!");
}

fn delete(_args: &DeleteArgs) {
    println!("praise him!");
}

fn main() {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        command.run();
    } else {
        Cli::command().print_help().unwrap();
    }
}
