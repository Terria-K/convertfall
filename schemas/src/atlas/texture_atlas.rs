use crate::math_schema::{Rectangle, Vector2, Area};

#[derive(Serialize, Deserialize, Debug)]
pub struct TextureAtlas {
    #[serde(rename = "imagePath")]
    pub image_path: String,
    #[serde(rename = "SubTexture")]
    pub sub_textures: Vec<SubTexture>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubTexture {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

// Generated
#[derive(Serialize, Deserialize, Debug)]
pub struct Frames {
    pub frames: Vec<TextureFrame>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextureFrame {
    pub filename: String,
    pub frame: Rectangle,
    pub rotated: bool,
    pub trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    pub sprite_source_size: Rectangle,
    #[serde(rename = "sourceSize")]
    pub source_size: Area,
    pub pivot: Vector2 
}
