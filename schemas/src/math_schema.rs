#[derive(Serialize, Deserialize, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    pub w: i32,
    pub h: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}
