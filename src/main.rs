#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::*;

fn main() -> Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;

    if let Some(cmd) = input.split_whitespace().next() {
        println!("{cmd}: command not found")
    }

    Ok(())
}
