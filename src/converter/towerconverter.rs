#![allow(dead_code)]
#![allow(unused_imports)]
use std::path::PathBuf;

use schemas::workshop::tower::{Tower, BitString, Level};
use serde::{Deserialize, Serialize};

use crate::converter::write_file_xml;

use super::{Converter, get_file, write_file_json};

pub struct TowerConverter {
    input_path: PathBuf,
    output_path: PathBuf,
    output_type: String
}

impl TowerConverter {
    pub const fn new(input_path: PathBuf, output_path: PathBuf, output_type: String) 
        -> Self {
        TowerConverter { input_path, output_path, output_type }
    }


}

impl Converter for TowerConverter {
    fn start(&self) -> anyhow::Result<&str> {
        let content = get_file(&self.input_path)?;
        let tower = quick_xml::de::from_str::<Tower>(&content)?;
        let output_path = self.output_path.clone();
        if !output_path.exists() {
            std::fs::create_dir(&output_path)?;
        }
        oel_conversion(tower, output_path)?;
        Ok("Successful Conversion from .tower to .oel")
    }
}

fn bitstring_to_vec(bits: Option<BitString>) -> Vec<String> {
    if let Some(bits) = bits {
        bits.value.map(|x| {
            x.chars().map(|x| {
               x.to_string()
            }).filter(|x| x != "\n")
            .collect::<Vec<String>>()
        }).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn bitstring_to_int_vec(bits: Option<BitString>) -> Vec<i32> {
    if let Some(bits) = bits {
        bits.value.map(|x| {
            x.split(',')
                .map(|x| x.parse::<i32>().unwrap_or(-1))
                .collect::<Vec<i32>>()
        }).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn oel_conversion(tower: Tower, output_path: PathBuf) -> anyhow::Result<()> { 
    for (i, level) in tower.levels.level.iter().enumerate() {
        let output_path = output_path.join(format!("0{}.oel", i));
        let bg_tiles = bitstring_to_int_vec(level.bg_tiles.to_owned());
        // let mut row = 0;
        // TODO Make serializing BG Tiles correctly aligned
        let bg_tiles_str = bg_tiles.iter()
            .enumerate()
            .map(|(_i, x)| {
                // if row * 31 + i >= 30  {
                //     row += 1;
                //     x.to_string() + "\n"
                // } else {
                x.to_string() + ","
                // }
            })
            .collect::<String>();
        let bg_tiles_str = BitString { 
            tileset: Some("Tileset".into()), export_mode: "TrimmedCSV".into(), 
            value: Some(bg_tiles_str) 
        };
        let solid_tiles_str = BitString {
            tileset: Some("Tileset".into()), export_mode: "TrimmedCSV".into(), 
            value: level.solid_tiles.to_owned().unwrap().value
        };
        let level = Level {
            width: Some(320),
            height: Some(240),
            ffa: None,
            teams: None,
            load_seed: 0,
            bg_tiles: Some(bg_tiles_str),
            solid_tiles: Some(solid_tiles_str),
            ..level.to_owned()
        };
        write_file_xml(&output_path, level)?;
    }
    Ok(())
}

