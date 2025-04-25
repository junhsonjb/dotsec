use anyhow::Result;
use clap::{Args, CommandFactory, Parser, Subcommand};
use sled::Db;

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
    key: String,
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
    force: bool,
}

impl Command {
    fn run(&self) -> Result<()> {
        match self {
            Command::Put(args) => put(args),
            Command::Get(args) => get(args),
            Command::List => list(),
            Command::Delete(args) => delete(args),
        }
    }
}

fn put(args: &PutArgs) -> Result<()> {
    let storage = SledStorage::new()?;
    storage.put(&args.key, &args.value)
}

fn get(args: &GetArgs) -> Result<()> {
    let storage = SledStorage::new()?;
    let result = storage.get(&args.key)?;
    let output = match result {
        Some(value) => format!("{value}"),
        None => String::from("No value found"), // TODO: shouldn't this go to stderr somehow??
    };

    println!("{output}");
    Ok(())
}

fn list() -> Result<()> {
    let storage = SledStorage::new()?;
    for key in storage.list()? {
        println!("{key}");
    }
    Ok(())
}

fn delete(args: &DeleteArgs) -> Result<()> {
    if args.delete_flags.force {
        let storage = SledStorage::new()?;
        storage.delete(&args.key)?;
    } else {
        println!("would delete secret with name {}", args.key);
    }
    Ok(())
}

pub trait Storage {
    fn put(&self, key: &str, value: &str) -> Result<()>;
    fn get(&self, key: &str) -> Result<Option<String>>;
    fn list(&self) -> Result<Vec<String>>;
    fn delete(&self, key: &str) -> Result<()>;
}

pub struct SledStorage {
    db: Db,
}

impl SledStorage {
    fn new() -> Result<SledStorage> {
        let db = sled::open("/tmp/sled-dev")?;
        Ok(SledStorage { db })
    }
}

impl Storage for SledStorage {
    fn put(&self, key: &str, value: &str) -> Result<()> {
        self.db.insert(key.as_bytes(), value.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<String>> {
        match self.db.get(key.as_bytes())? {
            Some(value) => {
                let value_str =
                    String::from_utf8(value.to_vec()).expect("Non-UTF8 value found in database");
                Ok(Some(value_str))
            }
            None => Ok(None),
        }
    }

    fn list(&self) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        for result in self.db.iter() {
            let (key, _) = result?;
            let key_str = String::from_utf8(key.to_vec()).expect("Non-UTF8 key found in database");
            keys.push(key_str);
        }
        Ok(keys)
    }

    fn delete(&self, key: &str) -> Result<()> {
        self.db.remove(key.as_bytes())?;
        Ok(())
    }
}

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
