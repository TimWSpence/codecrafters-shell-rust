use anyhow::*;
use core::result::Result::Ok;
use std::{
    env,
    fs::{self},
    path::PathBuf,
};

fn path() -> Result<Vec<String>> {
    let path = env::var("PATH")?;
    Ok(path.split(':').map(|s| s.to_owned()).collect())
}

pub fn search(cmd: &str) -> Result<Option<PathBuf>> {
    let path = path()?;

    let mut res: Option<PathBuf> = None;
    for p in path {
        let fs = fs::read_dir(p);
        if let Ok(fs) = fs {
            for f in fs.flatten() {
                if let Ok(t) = f.file_type() {
                    if t.is_file() && f.file_name() == cmd {
                        res = Some(f.path());
                    }
                }
            }
        }
    }

    Ok(res)
}
