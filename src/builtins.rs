use core::result::Result::Ok;
use std::{env, fs};

use anyhow::*;

use crate::path;

pub fn dispatch(cmd: &str) -> Option<Box<dyn Handler>> {
    match cmd {
        "exit" => Some(Box::new(ExitHandler {})),
        "echo" => Some(Box::new(EchoHandler {})),
        "type" => Some(Box::new(TypeHandler {})),
        "pwd" => Some(Box::new(PwdHandler {})),
        "cd" => Some(Box::new(CdHandler {})),
        _ => None,
    }
}

pub trait Handler {
    fn handle(&self, args: Vec<&str>) -> Result<()>;
}

struct ExitHandler {}

impl Handler for ExitHandler {
    fn handle(&self, args: Vec<&str>) -> Result<()> {
        let code = args.first().unwrap_or(&"1");
        let code = code.parse().unwrap();
        std::process::exit(code);
    }
}

struct EchoHandler {}

impl Handler for EchoHandler {
    fn handle(&self, args: Vec<&str>) -> Result<()> {
        println!("{}", args.join(" "));
        Ok(())
    }
}

struct TypeHandler {}

impl Handler for TypeHandler {
    fn handle(&self, args: Vec<&str>) -> Result<()> {
        if args.len() == 1 {
            let cmd = args.first().unwrap();
            let tpe = match dispatch(cmd) {
                Some(_) => Some("is a shell builtin".to_owned()),
                None => {
                    path::search(cmd)?.map(|p| format!("is {}", p.to_str().unwrap().to_owned()))
                }
            };

            if let Some(tpe) = tpe {
                println!("{} {}", cmd, tpe)
            } else {
                println!("{}: not found", cmd)
            };

            Ok(())
        } else {
            eprintln!("No argument provided for `type`");
            Ok(())
        }
    }
}
struct PwdHandler {}

impl Handler for PwdHandler {
    fn handle(&self, _args: Vec<&str>) -> Result<()> {
        let cwd = env::current_dir()?;
        println!("{}", cwd.to_str().unwrap());
        Ok(())
    }
}

struct CdHandler {}

impl Handler for CdHandler {
    fn handle(&self, args: Vec<&str>) -> Result<()> {
        //TODO: cd to ~ if no argument supplied
        let path = args.first().unwrap();
        let meta = fs::metadata(path);
        match meta {
            Ok(_) => env::set_current_dir(path),
            Err(_) => {
                eprintln!("cd: {}: No ush file or directory", path);
                Ok(())
            }
        }?;
        Ok(())
    }
}
