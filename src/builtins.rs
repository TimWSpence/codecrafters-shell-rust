use anyhow::*;

pub fn handle(cmd: &str, args: Vec<&str>) -> Option<Result<()>> {
    match cmd {
        "exit" => {
            let code = args.first().unwrap_or(&"1");
            let code = code.parse().unwrap();
            std::process::exit(code);
        }
        "echo" => {
            println!("{}", args.join(" "));
            Some(Ok(()))
        }
        _ => None,
    }
}
