use decorum::*;

use super::vertex::*;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub v1: Vertex,
    pub v2: Vertex,
}

impl Default for Line {
    fn default() -> Line {
        Line {
            v1: Vertex::default(),
            v2: Vertex::default(),
        }
    }
}
impl From<(Vertex, Vertex)> for Line {
    fn from(input: (Vertex, Vertex)) -> Self {
        Line {
            v1: input.0,
            v2: input.1,
        }
    }
}