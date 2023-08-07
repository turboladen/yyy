use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{path::Path, process::Command}; // Run programs

#[test]
fn test_db_create() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("yyy")?;

    cmd.arg("db-create");

    cmd.assert().success();

    if !Path::new("yyy.test.db").try_exists().unwrap() {
        panic!("database directory doesn't exist");
    }

    Ok(())
}

#[test]
fn test_import() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("yyy")?;

    cmd.arg("import");

    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided:",
    ));

    // cmd.arg("brands").arg("seeds/brands.yml");

    // cmd.assert().success();

    Ok(())
}
