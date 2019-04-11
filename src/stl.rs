use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use byteorder::{ReadBytesExt, LittleEndian};
use decorum::*;

use crate::geometry as geometry;
use crate::geometry::Mesh as Mesh;

pub struct STLFile {
    pub file: String,
    pub mesh: geometry::Mesh,
    pub triangle_count: u32,
    pub triangles: Vec<geometry::Triangle>,
}

impl Default for STLFile {
    fn default() -> STLFile {
        STLFile {
            file: String::new(),
            mesh: geometry::Mesh::default(),
            triangle_count: 0u32,
            triangles: Vec::new(),
        }
    }
}

impl STLFile {

    pub fn new(filename: String) -> Self {
        let mut me = STLFile::default();

        let mut file = match File::open(&filename) {
            Ok(f) => f,
            Err(e) => return STLFile::default(),
        };

        me.file = filename;
        me.mesh = match me.read_stl(&mut file) {
            Ok(mesh) => mesh,
            Err(e) => return STLFile::default(),
        };

        return me;
    }

    pub(crate) fn read_stl<T: ReadBytesExt>(&mut self, input: &mut T) -> Result<geometry::Mesh> {
        let header = match STLFile::read_header(input) {
            Ok(h) => h,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error: Couldn't read STL Header: {}", e))),
        };

        // Check if STL Header starts with "solid" indicating an ascii stl file. Otherwise, read as binary
        match &header[0..5] {
            b"solid" => return Err(Error::new(ErrorKind::Other, "ASCII STL not yet implemented.")),
            _ => {
                match STLFile::read_binary_stl(self, input) {
                    Ok(_) => (),
                    Err(e) => return Err(Error::new(ErrorKind::Other, "Couldn't read Binary STL File.")),
                }
                self.mesh = Mesh::from(self.triangles.clone());
                Ok(self.mesh.clone())
            },
        }

    }

    pub(crate) fn read_binary_stl<T: ReadBytesExt>(&mut self, input: &mut T) -> Result<()> {
        self.triangle_count = match input.read_u32::<LittleEndian>() {
            Ok(tc) => tc,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read Triangle count: {}", e))),
        };

        // Loop over file reading individual triangles. 
        for tn in 0..self.triangle_count {
            match STLFile::read_triangle(input) {
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

    pub(crate) fn read_triangle<T: ReadBytesExt>(input: &mut T) -> Result<geometry::Triangle> {
        let mut t: geometry::Triangle = geometry::Triangle::default();

        let mut normal: [f32; 3] = [0f32; 3];
        match input.read_f32_into::<LittleEndian>(&mut normal) {
            Ok(_) => t.normal = geometry::Vertex::from(normal),
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read normal: {}", e))),
        }

        for v in 0..3 {
            let mut tri: [f32; 3] = [0f32; 3];
            match input.read_f32_into::<LittleEndian>(&mut tri) {
                Ok(_) => {
                    t.vertices[v] = geometry::Vertex::from(tri);
                },
                Err(e) => return Err(Error::new(ErrorKind::Other, "Couldn't read triangle.")),
            }
        }

        match input.read_u16::<LittleEndian>() {
            Ok(abc) => t.attr_byte_count = abc,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read attribute byte count for triangle: {}", e))),
        }

        Ok(t)
    }

}