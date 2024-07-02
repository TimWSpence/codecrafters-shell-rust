use anyhow::*;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> Result<()> {
    // Wait for user input
    let stdin = io::stdin();
    loop {
        let mut input = String::new();

        print!("$ ");
        io::stdout().flush()?;

        stdin.read_line(&mut input)?;

        if let Some(cmd) = input.split_whitespace().next() {
            println!("{cmd}: command not found")
        }
    }
}
