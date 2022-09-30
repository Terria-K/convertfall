use std::path::PathBuf;

use serde::Serialize;

pub mod towerconverter;
pub mod xmltopackerjson;

pub trait Converter {
    fn start(&self) -> anyhow::Result<()>;
}

pub fn get_file(path: &PathBuf) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn write_file<S: Serialize>(path: &PathBuf, s: S) -> anyhow::Result<()> {
    let content = serde_json::to_string_pretty::<S>(&s)?;
    std::fs::write(path, content)?;
    Ok(())
}
