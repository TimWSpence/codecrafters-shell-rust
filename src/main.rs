use anyhow::*;
#[allow(unused_imports)]
use std::io::{self, Write};

mod builtins;
mod fork;
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
            } else if let Some(cmd) = path::search(cmd)? {
                let args: Vec<&str> = iter.collect();
                fork::fork(cmd.to_str().unwrap(), args)?;
            } else {
                println!("{cmd}: command not found")
            }
        }
    }
}
