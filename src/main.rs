use decorum::*;

pub mod geometry;
pub mod models;

use models::stl::*;
use geometry::mesh::Mesh as Mesh;
use geometry::slice::Slice as Slice;

fn main() {

    let stl: STLFile = match STLFile::new("fixtures/3DBenchy_bin.stl".to_string()) {
        Ok(stl) => stl,
        Err(e) => panic!("{}", e),
    };
    let mut mesh = Mesh::from(stl);
    mesh.normalize(1.0.into(), 1.0.into(), 0.0.into());
    mesh.find_extents();
    
    // println!("{:?}\n{:?}", mesh.minimum, mesh.maximum);

    let slices = mesh.slice(0.2.into());

    // println!("x, y, z");
    // for slice in slices {
    //     for line in slice.lines {
    //         println!("{}, {}, {}\n{}, {}, {}\n", line.v1.x, line.v1.y, line.v1.z, line.v2.x, line.v2.y, line.v2.z);
    //     }
    // }
}