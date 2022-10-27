use assert_cmd::Command;

const BIN: &str = "atlas";

#[test]
fn test_atlas_empty_args() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.assert().failure();
}

#[test]
fn test_atlas_help() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("-h").assert().success();
}

#[test]
fn test_atlas_version() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();

    cmd.arg("-V").assert().success();
}
