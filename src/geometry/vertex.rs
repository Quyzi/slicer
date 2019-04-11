use decorum::*;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: R32,
    pub y: R32,
    pub z: R32,
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            x: 0.0.into(),
            y: 0.0.into(),
            z: 0.0.into(),
        }
    }
}
impl From<[f32; 3]> for Vertex {
    fn from(input: [f32; 3]) -> Self {
        Vertex {
            x: input[0].into(),
            y: input[1].into(),
            z: input[2].into(),
        }
    }
}