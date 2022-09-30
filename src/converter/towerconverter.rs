#![allow(dead_code)]
#![allow(unused_imports)]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{Converter, get_file};

pub struct TowerConverter {
    input_path: PathBuf,
    output_path: PathBuf
}

impl TowerConverter {
    pub const fn new(input_path: PathBuf, output_path: PathBuf) -> Self {
        TowerConverter { input_path, output_path }
    }
}

impl Converter for TowerConverter {
    fn start(&self) -> anyhow::Result<()> {
        unimplemented!("It's not yet implemented, further updates will come soon");
        // let content = get_file(&self.input_path)?;
        // // println!("{content}");
        // let tower = quick_xml::de::from_str::<Tower>(&content)?;
        // println!("{:?}", tower);
        // todo!("Add a .tower converter");
        // Ok(())
    }
}

// Structures

#[derive(Deserialize, Serialize, Debug)]
pub struct Tower {
    title: String,
    author: String,
    // icon: Icon,
    map: Vector2,
    // variants: Vec<Variants>,
    #[serde(rename = "darkWorldDLC")]
    // Booleans are needed to be string
    dark_world_dlc: String,
    world: String,
    // treasure is not supported
    tileset: String,
    #[serde(rename = "bgtileset")]
    bg_tileset: String,
    background: String,
    music: String,
    lanterns: String,
    #[serde(rename = "darkOpacity")]
    dark_opacity: f32,
    cold: String, 
    levels: Vec<Level>
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Icon {
    #[serde(rename = "data")]
    Data(String),
    #[serde(rename = "tile")]
    Tile(String)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Vector2 {
    x: f32, y: f32
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Variants {
    // Not supported yet
}

// Because of how .tower structure their data, we need to separate
// the level and their data
// #[derive(Deserialize, Serialize, Debug)]
// pub struct Level {
//     #[serde(rename = "$value")]
//     level: LevelData
// }

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "level")]
pub struct Level {
    ffa: Option<i32>,
    teams: Option<String>,
    level_element: LevelElement
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LevelElement {
    LoadSeed(i32),
    Solids(Option<String>),
    BG(Option<String>),
    Entities(Option<Vec<Entities>>),
    BGTiles(Option<String>)
}

// #[derive(Deserialize, Serialize, Debug)]
// pub struct LevelElement {
//     #[serde(rename = "LoadSeed")]
//     load_seed: Option<i32>,
//     #[serde(rename = "Solids")]
//     solids: Option<String>,
//     #[serde(rename = "BG")]
//     bg: Option<String>,
//     // #[serde(rename = "Entities")]
//     // entities: Option<Vec<Entities>>,
//     #[serde(rename = "BGTiles")]
//     bg_tiles: Option<String>
// }

#[derive(Deserialize, Serialize, Debug)]
pub enum Entities {
    TreasureChest { x: f32, y: f32 },
    Spawner { x: f32, y: f32 },
    PlayerSpawn { x: f32, y: f32 },
    BGLantern { x: f32, y: f32 },
    ExplodingOrb { x: f32, y: f32 },
    OrbEd { x: f32, y: f32 },
    ProximityBlock { x: f32, y: f32 },
    MoonGlassBlock { x: f32, y: f32, width: f32, height: f32 },
    EndlessPortal { x: f32, y: f32 }
}
