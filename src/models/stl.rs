use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use byteorder::{ReadBytesExt, LittleEndian};

use crate::geometry as geometry;

pub enum STLType {
    Binary,
    ASCII,
}

pub struct STLFile {
    pub file: String,
    pub filetype: STLType,
    pub triangle_count: u32,
    pub triangles: Vec<geometry::Triangle>,
}

impl Default for STLFile {
    fn default() -> Self {
        STLFile {
            file: String::new(),
            triangle_count: 0u32,
            filetype: STLType::Binary,
            triangles: Vec::new(),
        }
    }
}

impl STLFile {
    pub fn new(filename: String) -> Result<Self> {
        let mut me = STLFile::default();

        let mut file = match File::open(&filename) {
            Ok(f) => f,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read input file {}: {}", filename, e))),
        };
        me.file = filename;

        match me.read_header(&mut file) {
            Ok(_) => Ok(me),
            Err(e) => Err(e),
        }

    }

    pub(crate) fn read_header<T: ReadBytesExt>(&mut self, input: &mut T) -> Result<()> {
        let mut header = [0u8; 5];
        match input.read(&mut header) {
            Ok(_) => (),
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read STL File header: {}", e))),
        }
        match &header {
            b"solid" => {
                self.filetype = STLType::ASCII;
                match self.read_ascii_stl(input) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            },
            _ => {
                self.filetype = STLType::Binary;
                match self.read_binary_stl(input) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }
    }

    pub(crate) fn read_binary_stl<T: ReadBytesExt>(&mut self, input: &mut T) -> Result<()> {
        let mut header = [0u8; 75];

        // Read remaining 75 u8 of header.
        match input.read(&mut header) {
            Ok(_) => (),
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read STL Header: {}", e))),
        }

        // Read number of triangles
        self.triangle_count = match input.read_u32::<LittleEndian>() {
            Ok(tc) => tc,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read Triangle count: {}", e))),
        };

        for tn in 0..self.triangle_count {
            match STLFile::read_triangle(input) {
                Ok(t) => self.triangles.push(t),
                Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read triangle {} of {}: {}", tn, self.triangle_count, e))),
            }
        }

        if self.triangle_count == self.triangles.len() as u32 {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Triangle count does not match STL."))
        }
    }

    pub(crate) fn read_triangle<T: ReadBytesExt>(input: &mut T) -> Result<geometry::Triangle> {
        let mut t: geometry::Triangle = geometry::Triangle::default();

        let mut normal: [f32; 3] = [0f32; 3];
        match input.read_f32_into::<LittleEndian>(&mut normal) {
            Ok(_) => t.normal = geometry::Vertex::from(normal),
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read normal: {}", e))),
        }

        for _ in 0..3 {
            let mut tri: [f32; 3] = [0f32; 3];
            match input.read_f32_into::<LittleEndian>(&mut tri) {
                Ok(_) => {
                    t.vertices.push(geometry::Vertex::from(tri));
                },
                Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read triangle: {}", e))),
            }
        }

        match input.read_u16::<LittleEndian>() {
            Ok(abc) => t.attr_byte_count = abc,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("Couldn't read attribute byte count for triangle: {}", e))),
        }

        Ok(t)
    }

    pub(crate) fn read_ascii_stl<T: ReadBytesExt>(&mut self, _input: &mut T) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "ASCII STL Files not yet implemented."))
    }
}