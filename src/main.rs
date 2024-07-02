use anyhow::*;
#[allow(unused_imports)]
use std::io::{self, Write};

mod builtins;
mod path;

fn main() -> Result<()> {
    // Wait for user input
    let stdin = io::stdin();
    loop {
        let mut input = String::new();

        print!("$ ");
        io::stdout().flush()?;

        stdin.read_line(&mut input)?;

        let mut iter = input.split_whitespace();
        if let Some(cmd) = iter.next() {
            if let Some(handler) = builtins::dispatch(cmd) {
                let args: Vec<&str> = iter.collect();
                handler.handle(args)?;
            } else {
                println!("{cmd}: command not found")
            }
        }
    }
}
