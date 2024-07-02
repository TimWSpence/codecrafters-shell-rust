use anyhow::*;

pub fn dispatch(cmd: &str) -> Option<Box<dyn Handler>> {
    match cmd {
        "exit" => Some(Box::new(ExitHandler {})),
        "echo" => Some(Box::new(EchoHandler {})),
        "type" => Some(Box::new(TypeHandler {})),
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
                Some(_) => Some("is a shell builtin"),
                None => None,
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
