use std::path::PathBuf;

use schemas::atlas::texture_atlas::{Frames, SubTexture, TextureAtlas};

use super::{Converter, get_file, write_file_xml};

pub struct PackerJsonToXML {
    input_path: PathBuf,
    output_path: PathBuf,
    excess_length: Option<usize>
}

impl PackerJsonToXML {
    pub const fn new(
        input_path: PathBuf, 
        output_path: PathBuf, 
        excess_length: Option<usize>
    ) -> Self {
        PackerJsonToXML { input_path, output_path, excess_length }
    }
}

impl Converter for PackerJsonToXML {
    fn start(&self) -> anyhow::Result<&str> {
        let content = get_file(&self.input_path)?;
        let json = serde_json::from_str::<Frames>(&content)?;
        let sub_textures = json.frames.iter()
            .map(|st| {
                let filename = 
                    if let Some(len) = self.excess_length {
                         st.filename
                            .chars()
                            .skip(len + 1)
                            .collect()
                    } else {
                        st.filename.clone()
                    };
                SubTexture {
                    name: filename,
                    x: st.frame.x,
                    y: st.frame.y,
                    width: st.frame.w,
                    height: st.frame.h,
                }
            }).collect::<Vec<SubTexture>>();
        let mut new_input_path = self.input_path.to_owned();
        new_input_path.set_extension("png");
        let image_path = new_input_path.to_str()
            .unwrap_or("")
            .to_string();
        let texture_atlas = TextureAtlas {
            image_path,
            sub_textures
        };
        write_file_xml(&self.output_path, texture_atlas)?;
        Ok("Successful conversion from .json to .xml")
    }
}
