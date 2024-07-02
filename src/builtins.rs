use anyhow::*;

pub fn handle(cmd: &str, args: Vec<&str>) -> Result<Option<()>> {
    match cmd {
        "exit" => {
            let code = args.first().unwrap_or(&"1");
            let code = code.parse()?;
            std::process::exit(code);
        }
        _ => Ok(None),
    }
}
