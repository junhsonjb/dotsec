use crate::storage::{SledStorage, Storage};
use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
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
pub struct PutArgs {
    /// Name of the secret to store
    key: String,

    /// Value of the secret to store
    value: String, // TODO: should this be a string?
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// Name of the secret to get
    key: String,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
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
    force: bool,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        match self {
            Command::Put(args) => put(args),
            Command::Get(args) => get(args),
            Command::List => list(),
            Command::Delete(args) => delete(args),
        }
    }
}

const DEV_PATH: &str = "/tmp/sled-dev"; // TODO: replace this with a real path

fn put(args: &PutArgs) -> Result<()> {
    let storage = SledStorage::new(DEV_PATH)?;
    storage.put(&args.key, &args.value)
}

fn get(args: &GetArgs) -> Result<()> {
    let storage = SledStorage::new(DEV_PATH)?;
    let result = storage.get(&args.key)?;
    let output = match result {
        Some(value) => format!("{value}"),
        None => String::from("No value found"), // TODO: shouldn't this go to stderr somehow??
    };

    println!("{output}");
    Ok(())
}

fn list() -> Result<()> {
    let storage = SledStorage::new(DEV_PATH)?;
    for key in storage.list()? {
        println!("{key}");
    }
    Ok(())
}

fn delete(args: &DeleteArgs) -> Result<()> {
    if args.delete_flags.force {
        let storage = SledStorage::new(DEV_PATH)?;
        storage.delete(&args.key)?;
    } else {
        println!("would delete secret with name {}", args.key);
    }
    Ok(())
}
