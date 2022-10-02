#![allow(dead_code)]
#![allow(unused_imports)]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::converter::write_file_xml;

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
        // unimplemented!("It's not yet implemented, further updates will come soon");
        let content = get_file(&self.input_path)?;
        // println!("{content}");
        let tower = quick_xml::de::from_str::<Tower>(&content)?;
        println!("{:?}", tower);
        let output_path = self.output_path.clone();
        if !output_path.exists() {
            std::fs::create_dir(&output_path)?;
        }
        for (i, level) in tower.levels.level.iter().enumerate() {
            let output_path = output_path.join(format!("{}.oel", i));
            write_file_xml(&output_path, level.clone())?;
        }

        Ok(())
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
    levels: Levels
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
#[derive(Deserialize, Serialize, Debug)]
pub struct Levels {
    level: Vec<Level>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename = "level")]
pub struct Level {
    ffa: Option<i32>,
    teams: Option<String>,
    #[serde(rename = "$unflatten=LoadSeed")]
    load_seed: i32,
    #[serde(rename = "$unflatten=Solids")]
    solids: Option<String>,
    #[serde(rename = "$unflatten=BG")]
    bg: Option<String>,
    #[serde(rename = "Entities")]
    entities: Option<Entities>,
    #[serde(rename = "$unflatten=BGTiles")]
    bg_tiles: Option<String>
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Entities {
    #[serde(rename = "$value")]
    entities: Vec<EntityData>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum EntityData {
    TreasureChest { x: f32, y: f32, id: Option<u32> },
    Spawner { x: f32, y: f32 , id: Option<u32>},
    PlayerSpawn { x: f32, y: f32, id: Option<u32>},
    BGLantern { x: f32, y: f32, id: Option<u32>},
    ExplodingOrb { x: f32, y: f32, id: Option<u32>},
    OrbEd { x: f32, y: f32, id: Option<u32>},
    ProximityBlock { x: f32, y: f32, id: Option<u32>},
    MoonGlassBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    EndlessPortal { x: f32, y: f32, id: Option<u32>},
    SpikeBallEd { x: f32, y: f32, height: f32, id: Option<u32>},
    Chain { x: f32, y: f32, height: f32, id: Option<u32>},
    Ogmo { x: f32, y: f32, id: Option<u32>},
    YellowStatue { x: f32, y: f32, id: Option<u32>},
    FakeWall { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    CrackedWall { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    Dummy { x: f32, y: f32, #[serde(rename = "Facing")]facing: String, id: Option<u32>},
}
