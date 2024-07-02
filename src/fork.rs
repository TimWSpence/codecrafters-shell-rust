use anyhow::*;
use std::io::{self, Write};
use std::process::Command;

pub fn fork(cmd: &str, args: Vec<&str>) -> Result<()> {
    let output = Command::new(cmd).args(args).output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stdout().flush()?;
    Ok(())
}
