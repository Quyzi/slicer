use std::io::{Error, ErrorKind, Result};
use byteorder::{ReadBytesExt, LittleEndian};
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

#[derive(Debug, Clone)]
pub struct Slice {
    pub lines: Vec<Line>,
    pub height: R32,
}
impl Default for Slice {
    fn default() -> Slice {
        Slice {
            lines: Vec::new(),
            height: 0.0.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub normal: Vertex,
    pub vertices: [Vertex; 3],
    pub attr_byte_count: u16,
}

impl Default for Triangle {
    fn default() -> Triangle {
        Triangle {
            normal: Vertex::default(),
            vertices: [Vertex::default(); 3],
            // lines: [Line::default(); 3],
            attr_byte_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub triangle_count: u32,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            triangles: Vec::new(),
            triangle_count: 0u32,
        }
    }

    pub fn read_stl<T: ReadBytesExt>(&mut self, input: &mut T) -> Result<()> {
        let header = match Mesh::read_header(input) {
            Ok(h) => h,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error: Couldn't read STL Header: {}", e))),
        };

        // Check if STL Header starts with "solid" indicating an ascii stl file. Otherwise, read as binary
        match &header[0..5] {
            b"solid" => return Err(Error::new(ErrorKind::Other, "ASCII STL not yet implemented.")),
            _ => self.read_binary_stl(input),
        }

    }

    pub fn slice(&mut self, point: R32) -> Slice {
        let mut slice: Slice = Slice {
            lines: Vec::new(),
            height: point,
        };

        // Find relevant triangles
        let mut triangles: Vec<Triangle> = Vec::new();
        for triangle in &self.triangles {
            
        }
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
        
        println!("Mins: {}, {}, {}", minx, miny, minz);

        if minx < 0.0|| miny < 0.0 || minz < 0.0 {
            let x_offset: R32 = minx.abs() + x;
            let y_offset: R32 = miny.abs() + y;
            let z_offset: R32 = minz.abs() + z;
            println!("Offset: {}, {}, {}", x_offset, y_offset, z_offset);
            for t in &mut self.triangles {
                for v in 0..3 {
                    t.vertices[v].x = t.vertices[v].x.mul_add(1.0.into(), x_offset);
                    t.vertices[v].y = t.vertices[v].y.mul_add(1.0.into(), y_offset);
                    t.vertices[v].z = t.vertices[v].z.mul_add(1.0.into(), z_offset);
                }
            }
        }
    }

    pub(crate) fn sort_by_z(&mut self) {
        for triangle in &mut self.triangles {
            triangle.vertices.sort_by(|a, b| b.z.cmp(&a.z));
        }
    }

    pub(crate) fn read_binary_stl<T: ReadBytesExt>(&mut self, input: &mut T) -> Result<()> {
        self.triangle_count = match input.read_u32::<LittleEndian>() {
            Ok(tc) => tc,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read Triangle count: {}", e))),
        };

        // Loop over file reading individual triangles. 
        for tn in 0..self.triangle_count {
            match Mesh::read_triangle(input) {
                Ok(t) => self.triangles.push(t),
                Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read Triangle {}/{}: {}", tn, self.triangle_count, e))),
            }
        }

        // Check if triangles read from file match file triangle count.
        match self.triangle_count == self.triangles.len() as u32 {
            true => Ok(()),
            false => Err(Error::new(ErrorKind::Other, "Triangle count doesn't match.")),
        }
    }

    pub(crate) fn read_header<T: ReadBytesExt>(input: &mut T) -> Result<[u8; 80]> {
        let mut header = [0u8; 80];

        match input.read(&mut header) {
            Ok(l) if l == 80 => Ok(header),
            Ok(l) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read 80 byte header. Read {} bytes.", l))),
            Err(e) => return Err(e),
        }
    }

    pub(crate) fn read_triangle<T: ReadBytesExt>(input: &mut T) -> Result<Triangle> {
        let mut t: Triangle = Triangle::default();

        let mut normal: [f32; 3] = [0f32; 3];
        match input.read_f32_into::<LittleEndian>(&mut normal) {
            Ok(_) => t.normal = Vertex::from(normal),
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read normal: {}", e))),
        }

        for v in 0..3 {
            let mut tri: [f32; 3] = [0f32; 3];
            match input.read_f32_into::<LittleEndian>(&mut tri) {
                Ok(_) => {
                    t.vertices[v] = Vertex::from(tri);
                },
                Err(e) => return Err(Error::new(ErrorKind::Other, "Couldn't read triangle.")),
            }
        }

        match input.read_u16::<LittleEndian>() {
            Ok(abc) => t.attr_byte_count = abc,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read attribute byte count for triangle: {}", e))),
        }

        // t.lines = [ Line::from((t.vertices[0], t.vertices[1])),
        //     Line::from((t.vertices[1], t.vertices[2])),
        //     Line::from((t.vertices[2], t.vertices[0]))];

        Ok(t)
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
                    z: z,
                };
                let v2: Vertex = Vertex {
                    x:  vertices[0].x + t2 * (vertices[1].x - vertices[0].x),
                    y: vertices[0].y + t1 * (vertices[1].y - vertices[0].y),
                    z: z,
                };
                return Some(Line {
                    v1: v1,
                    v2: v2,
                });
            } else if vertices[1].z <= z {
                let t1 = (z - vertices[2].z) / (vertices[1].z - vertices[2].z);
                let t2 = (z - vertices[2].z) / (vertices[0].z - vertices[2].z);

                let v1: Vertex = Vertex {
                    x: vertices[2].x + t1 * (vertices[1].x - vertices[2].x),
                    y: vertices[2].y + t1 * (vertices[1].y - vertices[2].y),
                    z: z,
                };
                let v2: Vertex = Vertex {
                    x: vertices[2].x + t2 * (vertices[0].x - vertices[2].x),
                    y: vertices[2].y + t2 * (vertices[0].y - vertices[2].y),
                    z: z,
                };
                return Some(Line {
                    v1: v1,
                    v2: v2,
                });
            }
        }
        return None;
    }
}