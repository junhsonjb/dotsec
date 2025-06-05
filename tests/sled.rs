use anyhow::Result;
use tempfile::TempDir;
use dotsec::storage::{SledStorage, Storage};
// use assert_cmd::Command;
// use predicates::prelude::*;

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

fn get_test_directory() -> TempDir {
    tempfile::tempdir().expect("failed to create a temporary directory")
}

#[test]
fn test_put_stores_value() -> Result<()> {
    let test_directory = get_test_directory();
    let test_path = test_directory.path().to_str().expect("invalid UTF-8 path");

    let sled = SledStorage::new(test_path)?;
    let result = sled.put("key", "val");
    assert!(
        result.is_ok(),
        "put did not succeed, returned: {:?}",
        result
    );
    Ok(())
}

#[test]
fn test_get_returns_correct_value() -> Result<()> {
    // TODO: use tempfiles so we don't need to append "-2" to the dev directory
    let test_directory = get_test_directory();
    let test_path = test_directory.path().to_str().expect("invalid UTF-8 path");

    let sled = SledStorage::new(test_path)?;
    sled.put("key", "val")?;

    let result = sled.get("key");
    assert!(
        result.is_ok(),
        "get did not succeed, returned: {:?}",
        result
    );

    let value = result?;
    assert_eq!(value, Some("val".to_string()));

    Ok(())
}

#[test]
fn test_update_overwrites_value() -> Result<()> {
    let test_directory = get_test_directory();
    let test_path = test_directory.path().to_str().expect("invalid UTF-8 path");

    let sled = SledStorage::new(test_path)?;
    sled.put("key", "val")?;

    let result = sled.get("key");
    assert!(result.is_ok(), "get did not succeed, returned {:?}", result);

    let value = result?;
    assert_eq!(value, Some("val".to_string()));

    // Overwrite the existing key/value pair and check that the correct value is retrieved
    sled.put("key", "Jalen Brunson is CLUTCH")?;

    let new_result = sled.get("key");
    assert!(
        new_result.is_ok(),
        "get did not succeed, returned {:?}",
        new_result
    );

    let new_value = new_result?;
    assert_eq!(new_value, Some("Jalen Brunson is CLUTCH".to_string()));

    Ok(())
}

#[test]
fn test_remove_deletes_key() -> Result<()> {
    let test_directory = get_test_directory();
    let test_path = test_directory.path().to_str().expect("invalid UTF-8 path");

    let sled = SledStorage::new(test_path)?;
    sled.put("key", "val")?;

    let result = sled.get("key");
    assert!(result.is_ok(), "get did not succeed, returned {:?}", result);

    let value = result?;
    assert_eq!(value, Some("val".to_string()));

    // Delete the key/value pair and check that nothing exists in the DB
    sled.delete("key")?;

    let new_result = sled.get("key");
    assert!(
        matches!(new_result, Ok(None)),
        "expected Ok(None), got {:?}",
        new_result
    );

    Ok(())
}

#[test]
fn test_list() -> Result<()> {
    let test_directory = get_test_directory();
    let test_path = test_directory.path().to_str().expect("invalid UTF-8 path");

    let sled = SledStorage::new(test_path)?;
    sled.put("key", "val")?;
    sled.put("jack", "jill")?;
    sled.put("peanut butter", "jelly")?;
    sled.put("rom", "com")?;

    let mut actual_list = sled.list()?;
    let mut expected_list = vec!["key", "jack", "peanut butter", "rom"];

    actual_list.sort();
    expected_list.sort();

    assert_eq!(actual_list.len(), 4);
    assert_eq!(actual_list, expected_list);

    Ok(())
}
