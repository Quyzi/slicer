use decorum::*;
use rayon::prelude::*;

use super::slice::*;
use super::triangle::*;
use super::line::*;
use crate::geometry;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub triangles: Vec<geometry::Triangle>,
    pub triangle_count: u32,
}

impl From<Vec<geometry::Triangle>> for Mesh {
    fn from(input: Vec<geometry::Triangle>) -> Self {
        Mesh {
            triangles: input.clone(),
            triangle_count: input.len() as u32,
        }
    }
}

impl From<crate::models::STLFile> for Mesh {
    fn from(input: crate::models::STLFile) -> Self {
        Mesh {
            triangles: input.triangles,
            triangle_count: input.triangle_count as u32,
        }
    }
}

impl Default for Mesh {
    fn default() -> Mesh {
        Mesh {
            triangles: Vec::new(),
            triangle_count: 0u32,
        }
    }
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            triangles: Vec::new(),
            triangle_count: 0u32,
        }
    }

    pub fn slice_at(&self, point: R32) -> Slice {
        let mut slice: Slice = Slice::default();

        // Find relevant triangles
        let mut triangles: Vec<Triangle> = Vec::new();
        for triangle in &self.triangles {
            if triangle.vertices[0].z >= point || triangle.vertices[1].z >= point || triangle.vertices[2].z >= point {
                // At least one Z is above slice point. 
                if triangle.vertices[0].z <= point || triangle.vertices[1].z <= point || triangle.vertices[2].z <= point {
                // At least one Z is <= slice point.
                    triangles.push(triangle.clone());
                }
            }
        }

        let lines: Vec<Option<Line>> = triangles.par_iter().map(|triangle| {
            if let Some(line) = triangle.intersects_z(point) {
                Some(line)
            } else {
                None
            }
        }).collect();
        slice.lines = lines.into_iter().flatten().collect();

        slice
    }

    pub(crate) fn normalize(&mut self, x: R32, y: R32, z: R32) {
        let mut minx: R32 = 0.0.into();
        let mut miny: R32 = 0.0.into();
        let mut minz: R32 = 0.0.into();

        for t in &self.triangles {
            for v in 0..3 {
                if t.vertices[v].x < minx {
                    minx = t.vertices[v].x;
                }
                if t.vertices[v].y < miny {
                    miny = t.vertices[v].y;
                }
                if t.vertices[v].z < minz {
                    minz = t.vertices[v].z;
                }
            }
        }

        // println!("Mins: {}, {}, {}", minx, miny, minz);

        if minx < 0.0 || miny < 0.0 || minz < 0.0 {
            let x_offset: R32 = minx.abs() + x;
            let y_offset: R32 = miny.abs() + y;
            let z_offset: R32 = minz.abs() + z;
            // println!("Offset: {}, {}, {}", x_offset, y_offset, z_offset);
            for t in &mut self.triangles {
                for v in 0..3 {
                    t.vertices[v].x = t.vertices[v].x.mul_add(1.0.into(), x_offset);
                    t.vertices[v].y = t.vertices[v].y.mul_add(1.0.into(), y_offset);
                    t.vertices[v].z = t.vertices[v].z.mul_add(1.0.into(), z_offset);
                }
            }
        }
    }
}
