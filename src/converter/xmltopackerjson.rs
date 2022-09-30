use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use super::{Converter, get_file, write_file};

pub struct XmlToPackerJson {
    input_path: PathBuf,
    output_path: PathBuf
}

impl XmlToPackerJson {
    pub const fn new(input_path: PathBuf, output_path: PathBuf) -> Self {
        XmlToPackerJson { input_path, output_path }
    }
}

impl Converter for XmlToPackerJson {
    fn start(&self) -> anyhow::Result<()> {
        let content = get_file(&self.input_path)?;
        // println!("{content}");
        let xml = quick_xml::de::from_str::<TextureAtlas>(&content)?;
        let frames = xml.sub_textures.iter()
            .map(|texture| {
                TextureFrame {
                    filename: (*texture.name).to_string(),
                    frame: Rectangle { 
                        x: texture.x, 
                        y: texture.y, 
                        w: texture.width, 
                        h: texture.height 
                    },
                    rotated: false,
                    trimmed: false,
                    sprite_source_size: Rectangle { 
                        x: 0, 
                        y: 0, 
                        w: texture.width, 
                        h: texture.height 
                    },
                    source_size: Area { w: texture.width, h: texture.height },
                    pivot: Vector2 { x: 0.5, y: 0.5 }
                }
            })
        .collect::<Vec<TextureFrame>>();
        let texture_frame = Frames { frames };
        write_file(&self.output_path, texture_frame)?;
        Ok(())
    }
}

// Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct TextureAtlas {
    #[serde(rename = "imagePath")]
    image_path: String,
    #[serde(rename = "$value")]
    sub_textures: Vec<SubTexture>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubTexture {
    name: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

// Generated
#[derive(Serialize, Deserialize, Debug)]
pub struct Frames {
    frames: Vec<TextureFrame>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextureFrame {
    filename: String,
    frame: Rectangle,
    rotated: bool,
    trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    sprite_source_size: Rectangle,
    #[serde(rename = "sourceSize")]
    source_size: Area,
    pivot: Vector2
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector2 {
    x: f32,
    y: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    w: i32,
    h: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}



