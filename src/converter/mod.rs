use std::path::PathBuf;

use quick_xml::{writer::Writer, se::Serializer};
use serde::Serialize;

pub mod towerconverter;
pub mod packerjsontoxml;
pub mod xmltopackerjson;

pub trait Converter {
    fn start(&self) -> anyhow::Result<()>;
}

pub fn get_file(path: &PathBuf) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn write_file_json<S: Serialize>(path: &PathBuf, s: S) -> anyhow::Result<()> {
    let content = serde_json::to_string_pretty::<S>(&s)?;
    std::fs::write(path, content)?;
    Ok(())
}

pub fn write_file_xml<S: Serialize>(path: &PathBuf, s: S) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    let writer = Writer::new_with_indent(&mut buffer, b' ', 2);
    let mut ser = Serializer::with_root(writer, None);
    s.serialize(&mut ser)?;
    std::fs::write(path, buffer)?;
    Ok(())
}
