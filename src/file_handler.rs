use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;

pub fn write_file(content: String, path: String) -> Result<()> {
    let mut file = File::create(path)?;
    write!(file, "{}", content)?;
    file.flush()?;
    Ok(())
}

pub fn do_mkdir(file_name: &str) -> Result<()> {
    fs::create_dir(file_name)?;
    Ok(())
}
