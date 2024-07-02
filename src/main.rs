use anyhow::*;
#[allow(unused_imports)]
use std::io::{self, Write};

mod builtins;

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
            let args: Vec<&str> = iter.collect();

            if let Some(_res_) = builtins::handle(cmd, args) {
            } else {
                println!("{cmd}: command not found")
            }
        }
    }
}
