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
    #[serde(rename = "LoadSeed")]
    load_seed: i32,
    #[serde(rename = "Solids")]
    solids: Option<BitString>,
    #[serde(rename = "BG")]
    bg: Option<BitString>,
    #[serde(rename = "SolidTiles", default = "default_tiles")]
    solid_tiles: Option<BitString>,
    #[serde(rename = "Entities")]
    entities: Option<Entities>,
    #[serde(rename = "BGTiles", default = "default_tiles")]
    bg_tiles: Option<BitString>
}

fn default_tiles() -> Option<BitString> {
    Some(BitString {
        export_mode: "TrimmedCSV".into(),
        tileset: None,
        value: None
    })
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BitString {
    tileset: Option<String>,
    #[serde(rename = "exportMode", default = "bits")]
    export_mode: String,
    #[serde(rename = "$value")]
    value: Option<String>
}

fn bits() -> String {
    String::from("Bitstring")
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Entities {
    #[serde(rename = "$value")]
    entities: Vec<EntityData>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[rustfmt::skip]
pub enum EntityData {
    BigTreasureChest { x: f32, y: f32, id: Option<u32> },
    PlayerSpawn { x: f32, y: f32, id: Option<u32>},
    TeamSpawn { x: f32, y: f32, id: Option<u32>},
    ExplodingOrb { x: f32, y: f32, id: Option<u32>},
    OrbEd { x: f32, y: f32, id: Option<u32>},
    CrumbleWall { x: f32, y: f32, id: Option<u32>},
    BGSkeleton { x: f32, y: f32, id: Option<u32>},
    ProximityBlock { x: f32, y: f32, id: Option<u32>},
    Lantern { x: f32, y: f32, id: Option<u32>},
    PirateBanner { x: f32, y: f32, id: Option<u32>},
    WaterDrop { x: f32, y: f32, id: Option<u32>},
    MushroomEd { x: f32, y: f32, id: Option<u32>},
    EndlessPortal { x: f32, y: f32, id: Option<u32>},
    CrackedWall { x: f32, y: f32, id: Option<u32>},
    SpikeBallEd { x: f32, y: f32, height: f32, id: Option<u32>},
    ExplodingSpikeBallEd { x: f32, y: f32, height: f32, id: Option<u32>},
    Chain { x: f32, y: f32, height: f32, id: Option<u32>},
    Ogmo { x: f32, y: f32, id: Option<u32>},
    Icicle { x: f32, y: f32, id: Option<u32>},
    YellowStatue { x: f32, y: f32, id: Option<u32>},
    JumpPad { x: f32, y: f32, width: f32, id: Option<u32> },
    SensorBlock { x: f32, y: f32, width: f32, id: Option<u32> },
    Mud { x: f32, y: f32, width: f32, id: Option<u32> },
    SnowEd { x: f32, y: f32, width: f32, id: Option<u32> },
    HotCoals { x: f32, y: f32, width: f32, id: Option<u32> },
    CrackedPlatform { x: f32, y: f32, width: f32, id: Option<u32> },
    Ice { x: f32, y: f32, width: f32, id: Option<u32> },
    ChalicePad { x: f32, y: f32, width: f32, id: Option<u32> },
    GhostPlatform { x: f32, y: f32, width: f32, id: Option<u32> },
    FakeWall { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    GraniteBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    CrumbleBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    RedSwitchBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    BlueSwitchBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    MoonGlassBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    MovingPlatform { x: f32, y: f32, 
        width: f32, height: f32, id: Option<u32>, 
        #[serde(rename = "$value")] node: Option<Vec<Node>>
    },
    PrismBlock { x: f32, y: f32, 
        width: f32, height: f32, id: Option<u32>, 
        #[serde(rename = "$value")] node: Option<Vec<Node>>
    },
    ShiftBlock { x: f32, y: f32, 
        width: f32, height: f32, id: Option<u32>, 
        #[serde(rename = "$value")] node: Option<Vec<Node>>
    },
    Dummy { x: f32, y: f32, #[serde(rename = "Facing")]facing: String, id: Option<u32>},
    BGLantern { x: f32, y: f32, id: Option<u32>,
        #[serde(rename = "Lit")] lit: Option<String>
    },
    RotatePlatformsCenter { 
        x: f32, y: f32, 
        #[serde(rename = "Amount")] amount: i32,
        #[serde(rename = "Width")] width: i32,
        #[serde(rename = "DegSpeed")] deg_speed: f32
    },
    BossMarker { x: f32, y: f32, id: Option<u32>, 
        #[serde(rename = "$value")] node: Option<Vec<Node>>
    },
    Spawner { x: f32, y: f32 , id: Option<u32>, 
        #[serde(rename = "$value")] node: Option<Vec<Node>>
    },
    BossMarkerB { x: f32, y: f32, id: Option<u32> },
    TreasureChest { x: f32, y: f32, id: Option<u32>,
        #[serde(rename = "Mode")] mode: Option<String>,
        #[serde(rename = "Treasure")] treasure: Option<String>,
        #[serde(rename = "Type")] r_type: Option<String>,
        #[serde(rename = "Timer")] timer: Option<f32>
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node { x: f32, y: f32 }
