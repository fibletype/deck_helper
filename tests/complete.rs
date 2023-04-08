use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

const BIN_PATH: &str = env!("CARGO_BIN_EXE_deck_helper");

fn run<I, S>(args: I) -> Result<std::process::Output, std::io::Error>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
{
    let bin = PathBuf::from(BIN_PATH);
    let out = Command::new(bin.as_os_str()).args(args).output()?;
    assert!(out.status.success());

    // let stdout = std::str::from_utf8(&out.stdout).unwrap();
    // println!("out:\n{}", stdout);

    Ok(out)
}

#[test]
fn run_help() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = run(["-h"]).map(|out| out.stdout)?;
    assert!(std::str::from_utf8(&stdout)?.contains("Usage:"));

    Ok(())
}
