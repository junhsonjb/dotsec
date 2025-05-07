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
    fn new(path: &str) -> Result<SledStorage> {
        let db = sled::open(path)?;
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
        self.db.flush()?;
        Ok(())
    }
}

// TODO: move these tests to their own test file when we move Storage and SledStorage to their own files
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_stores_value() -> Result<()> {
        let sled = SledStorage::new("/tmp/sled-test")?;
        let result = sled.put("key", "val");
        assert!(result.is_ok(), "put did not succeed, returned: {:?}", result);
        Ok(())
    }

    #[test]
    fn test_get_returns_correct_value() -> Result<()> {
        // TODO: use tempfiles so we don't need to append "-2" to the dev directory
        let sled = SledStorage::new("/tmp/sled-test-2")?;
        sled.put("key", "val")?;

        let result = sled.get("key");
        assert!(result.is_ok(), "get did not succeed, returned: {:?}", result);

        let value = result?;
        assert_eq!(value, Some("val".to_string()));

        Ok(())
    }

    #[test]
    fn test_update_overwrites_value() -> Result<()> {
        Ok(())
    }

    #[test]
    fn test_remove_deletes_key() -> Result<()> {
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
