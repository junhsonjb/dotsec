use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

// TODO: put sled tests here:
// just to get the logic done quickly, I don't feel like organizing stuff into a great file
// structure yet. As such, `SledStorage` is still in main.rs. While it's there, the tests will be
// there too (since main.rs is a binary and not a library, making it harder to import/use here).
// So yeah, that's what this file is for.

// TODO: test things like CRUD operation
#[test]
fn this_is_a_test() -> Result<()> {
    Ok(())
}
