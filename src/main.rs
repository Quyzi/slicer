use std::fs::File;
use decorum::*;

pub mod geometry;

mod stl;
use geometry::mesh::Mesh as Mesh;
use geometry::slice::Slice as Slice;
use stl::STLFile as STLFile;

fn main() {
    let mut file = match File::open("fixtures/3DBenchy_bin.stl") {
        Ok(f) => f,
        Err(e) => panic!("Couldn't read input file. {}", e),
    };
    let stl: STLFile = STLFile::new("fixtures/3DBenchy_bin.stl".to_string());
    let mut mesh: Mesh = stl.mesh;

    mesh.normalize(1.0.into(), 1.0.into(), 0.0.into());
    
    let mut height: R32 = 0.2.into();
    let mut slices: Vec<Slice> = Vec::new();

    for _ in 0..238 {
        height = height.mul_add(1.0.into(), 0.2.into());
        slices.push(mesh.slice_at( height ));
    }
    

    println!("x, y, z");
    for slice in slices {
        for line in slice.lines {
            println!("{}, {}, {}\n{}, {}, {}\n", line.v1.x, line.v1.y, line.v1.z, line.v2.x, line.v2.y, line.v2.z);
        }
    }
}