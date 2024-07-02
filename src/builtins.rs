use anyhow::*;

pub fn dispatch(cmd: &str) -> Option<Box<dyn Handler>> {
    match cmd {
        "exit" => Some(Box::new(ExitHandler {})),
        "echo" => Some(Box::new(EchoHandler {})),
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
