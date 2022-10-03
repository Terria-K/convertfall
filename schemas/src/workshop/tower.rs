// Structures

#[derive(Deserialize, Serialize, Debug)]
pub struct Tower {
    pub title: String,
    pub author: String,
    // icon: Icon,
    pub map: Vector2,
    // variants: Vec<Variants>,
    #[serde(rename = "darkWorldDLC")]
    // Booleans are needed to be string
    pub dark_world_dlc: String,
    pub world: String,
    // treasure is not supported
    pub tileset: String,
    #[serde(rename = "bgtileset")]
    pub bg_tileset: String,
    pub background: String,
    pub music: String,
    pub lanterns: String,
    #[serde(rename = "darkOpacity")]
    pub dark_opacity: f32,
    pub cold: String, 
    pub levels: Levels
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
    pub x: f32, pub y: f32
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Variants {
    // Not supported yet
}

// Because of how .tower structure their data, we need to separate
// the level and their data
#[derive(Deserialize, Serialize, Debug)]
pub struct Levels {
    pub level: Vec<Level>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename = "level")]
pub struct Level {
    #[serde(default = "width_default")]
    pub width: Option<i32>,
    #[serde(default = "height_default")]
    pub height: Option<i32>,
    pub ffa: Option<i32>,
    pub teams: Option<String>,
    #[serde(rename = "LoadSeed")]
    pub load_seed: i32,
    #[serde(rename = "Solids")]
    pub solids: Option<BitString>,
    #[serde(rename = "BG", default = "bg")]
    pub bg: Option<BitString>,
    #[serde(rename = "SolidTiles", default = "default_tiles")]
    pub solid_tiles: Option<BitString>,
    #[serde(rename = "Entities")]
    pub entities: Option<Entities>,
    #[serde(rename = "BGTiles", default = "default_tiles")]
    pub bg_tiles: Option<BitString>
}

const fn width_default() -> Option<i32> {
    Some(320)
}

const fn height_default() -> Option<i32> {
    Some(240)
}

fn bg() -> Option<BitString> {
    Some(BitString {
        export_mode: "TrimmedCSV".into(),
        tileset: Some("Tileset".into()),
        value: None
    })
}

fn default_tiles() -> Option<BitString> {
    Some(BitString {
        export_mode: "TrimmedCSV".into(),
        tileset: Some("Tileset".into()),
        value: None
    })
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BitString {
    pub tileset: Option<String>,
    #[serde(rename = "exportMode", default = "bits")]
    pub export_mode: String,
    #[serde(rename = "$value")]
    pub value: Option<String>
}

fn bits() -> String {
    String::from("Bitstring")
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Entities {
    #[serde(rename = "$value")]
    pub entities: Vec<EntityData>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[rustfmt::skip]
pub enum EntityData {
    BigTreasureChest { x: f32, y: f32, id: Option<u32>},
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
    JumpPad { x: f32, y: f32, width: f32, id: Option<u32>},
    SensorBlock { x: f32, y: f32, width: f32, id: Option<u32>},
    Mud { x: f32, y: f32, width: f32, id: Option<u32>},
    SnowEd { x: f32, y: f32, width: f32, id: Option<u32>},
    HotCoals { x: f32, y: f32, width: f32, id: Option<u32>},
    CrackedPlatform { x: f32, y: f32, width: f32, id: Option<u32>},
    Ice { x: f32, y: f32, width: f32, id: Option<u32>},
    ChalicePad { x: f32, y: f32, width: f32, id: Option<u32>},
    GhostPlatform { x: f32, y: f32, width: f32, id: Option<u32>},
    FakeWall { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    GraniteBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    CrumbleBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    RedSwitchBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    BlueSwitchBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    MoonGlassBlock { x: f32, y: f32, width: f32, height: f32, id: Option<u32>},
    MovingPlatform { x: f32, y: f32, 
        width: f32, height: f32, id: Option<u32>, 
        node: Option<Vec<Node>>,
    },
    PrismBlock { x: f32, y: f32, 
        width: f32, height: f32, id: Option<u32>, 
        node: Option<Vec<Node>>,
    },
    ShiftBlock { x: f32, y: f32, 
        width: f32, height: f32, id: Option<u32>, 
        node: Option<Vec<Node>>,
    },
    Dummy { x: f32, y: f32, #[serde(rename = "Facing")]facing: String, id: Option<u32>},
    BGLantern { x: f32, y: f32, id: Option<u32>,
        #[serde(rename = "Lit")] lit: Option<String>,
    },
    RotatePlatformsCenter { 
        x: f32, y: f32, 
        #[serde(rename = "Amount")] amount: i32,
        #[serde(rename = "Width")] width: i32,
        #[serde(rename = "DegSpeed")] deg_speed: f32,
    },
    BossMarker { x: f32, y: f32, id: Option<u32>, 
        node: Option<Vec<Node>>,
    },
    Spawner { x: f32, y: f32 , id: Option<u32>, 
        node: Option<Vec<Node>>,
    },
    BossMarkerB { x: f32, y: f32, id: Option<u32>},
    TreasureChest { x: f32, y: f32, id: Option<u32>,
        #[serde(rename = "Mode")] mode: Option<String>,
        #[serde(rename = "Treasure")] treasure: Option<String>,
        #[serde(rename = "Type")] r_type: Option<String>,
        #[serde(rename = "Timer")] timer: Option<f32>,
     },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node { x: f32, y: f32 }
