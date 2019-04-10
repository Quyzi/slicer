use std::fs::File;

use decorum::*;

mod stl;
use stl::*;

fn main() {
    let mut file = match File::open("fixtures/3DBenchy_bin.stl") {
        Ok(f) => f,
        Err(e) => panic!("Couldn't read input file. {}", e),
    };

    let mut mesh: Mesh = Mesh::new();
    match mesh.read_stl(&mut file) {
        Ok(_) => (),
        Err(e) => println!("{}", e), 
    }

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