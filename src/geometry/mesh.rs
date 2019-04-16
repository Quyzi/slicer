use decorum::*;
use rayon::prelude::*;

use super::slice::*;
use super::triangle::*;
use super::line::*;
use super::vertex::*;
use crate::geometry;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub triangles: Vec<geometry::Triangle>,
    pub triangle_count: u32,
    pub minimum: Vertex,
    pub maximum: Vertex,
}

impl From<Vec<geometry::Triangle>> for Mesh {
    fn from(input: Vec<geometry::Triangle>) -> Self {
        let mut m = Mesh {
            triangles: input.clone(),
            triangle_count: input.len() as u32,
            minimum: Vertex::default(),
            maximum: Vertex::default(),
        };
        m.find_extents();
        m
    }
}

impl From<crate::models::STLFile> for Mesh {
    fn from(input: crate::models::STLFile) -> Self {
        let mut m = Mesh {
            triangles: input.triangles,
            triangle_count: input.triangle_count as u32,
            minimum: Vertex::default(),
            maximum: Vertex::default(),
        };
        m.find_extents();
        m
    }
}

impl Default for Mesh {
    fn default() -> Mesh {
        let mut m = Mesh {
            triangles: Vec::new(),
            triangle_count: 0u32,
            minimum: Vertex::default(),
            maximum: Vertex::default(),
        };
        m.find_extents();
        m
    }
}

impl Mesh {
    pub fn new() -> Mesh {
        let mut m = Mesh {
            triangles: Vec::new(),
            triangle_count: 0u32,
            minimum: Vertex::default(),
            maximum: Vertex::default(),
        };
        m.find_extents();
        m
    }

    pub fn slice(&self, layer_height: R32) -> Vec<Slice> {
        let mut height: R32 = 0.0.into();
        let mut slices: Vec<Slice> = Vec::new();

        while height <= self.maximum.z {
            height = height.mul_add(1.0.into(), layer_height);
            slices.push(self.slice_at( height ));
        }
        slices
    }

    pub(crate) fn slice_at(&self, point: R32) -> Slice {
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
        slice.height = point;

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

    pub fn find_extents(&mut self) {
        let mut xx: Vec<R32> = Vec::new();
        let mut yy: Vec<R32> = Vec::new();
        let mut zz: Vec<R32> = Vec::new();

        for triangle in &self.triangles {
            for v in &triangle.vertices {
                xx.push(v.x);
                yy.push(v.y);
                zz.push(v.z);
            }
        }

        xx.sort_by(|a, b| b.cmp(&a));
        yy.sort_by(|a, b| b.cmp(&a));
        zz.sort_by(|a, b| b.cmp(&a));

        self.minimum = Vertex {
            x: xx.pop().unwrap(),
            y: yy.pop().unwrap(),
            z: zz.pop().unwrap(),
        };
        self.maximum = Vertex {
            x: xx[0],
            y: yy[0],
            z: zz[0],
        };
    }
}
