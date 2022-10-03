use std::path::PathBuf;

use super::{Converter, get_file, write_file_json};
use schemas::math_schema::{Area, Vector2, Rectangle};
use schemas::atlas::texture_atlas::{Frames, TextureFrame, TextureAtlas};

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
    fn start(&self) -> anyhow::Result<&str> {
        let content = get_file(&self.input_path)?;
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
        write_file_json(&self.output_path, texture_frame)?;
        Ok("Successful conversion from .xml to .json")
    }
}
