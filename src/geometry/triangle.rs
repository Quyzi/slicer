use decorum::*;

use super::vertex::*;
use super::line::*;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub normal: Vertex,
    pub vertices: Vec<Vertex>,
    pub attr_byte_count: u16,
}

impl Default for Triangle {
    fn default() -> Triangle {
        Triangle {
            normal: Vertex::default(),
            vertices: Vec::new(), 
            attr_byte_count: 0,
        }
    }
}



impl Triangle {
    pub fn intersects_z(&self, z: R32) -> Option<Line> {
        // Logic from https://kandepet.com/3d-printing-slicing-3d-objects/
        // sort vertices by Z height
        let mut vertices: Vec<Vertex> = vec!(self.vertices[0], self.vertices[1], self.vertices[2]);
        vertices.sort_by(|a, b| b.z.cmp(&a.z));
        vertices.reverse();

        if vertices[0].z > z && vertices[1].z > z && vertices[2].z > z {
            return None;
        } else if vertices[0].z <= vertices[1].z && vertices[1].z <= vertices[2].z {
            if vertices[1].z > z {
                let t1 = (z - vertices[0].z) / (vertices[2].z - vertices[0].z);
                let t2 = (z - vertices[0].z) / (vertices[1].z - vertices[0].z);
                let v1: Vertex = Vertex {
                    x: vertices[0].x + t1 * (vertices[2].x - vertices[0].x),
                    y: vertices[0].y + t1 * (vertices[2].y - vertices[0].y),
                    z,
                };
                let v2: Vertex = Vertex {
                    x: vertices[0].x + t2 * (vertices[1].x - vertices[0].x),
                    y: vertices[0].y + t2 * (vertices[1].y - vertices[0].y),
                    z,
                };
                return Some(Line {
                    v1,
                    v2,
                });
            } else if vertices[1].z <= z {
                let t1 = (z - vertices[2].z) / (vertices[1].z - vertices[2].z);
                let t2 = (z - vertices[2].z) / (vertices[0].z - vertices[2].z);
                let v1: Vertex = Vertex {
                    x: vertices[2].x + t1 * (vertices[1].x - vertices[2].x),
                    y: vertices[2].y + t1 * (vertices[1].y - vertices[2].y),
                    z,
                };
                let v2: Vertex = Vertex {
                    x: vertices[2].x + t2 * (vertices[0].x - vertices[2].x),
                    y: vertices[2].y + t2 * (vertices[0].y - vertices[2].y),
                    z,
                };
                return Some(Line {
                    v1,
                    v2,
                });
            }
        }
        None
    }
}