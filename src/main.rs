use std::fs::File;

mod stl;
use stl::*;

fn main() {
    let mut file = match File::open("fixtures/20mmcube.stl") {
        Ok(f) => f,
        Err(e) => panic!("Couldn't read input file. {}", e),
    };

    let mut mesh: Mesh = Mesh::new();
    match mesh.read_stl(&mut file) {
        Ok(_) => (),
        Err(e) => (), //println!("{}", e),
    }
    // println!("{:?}", mesh);
    // println!("");
    mesh.normalize(1.0.into(), 1.0.into(), 0.0.into());
    // println!("{:?}", mesh);

    let slice: Slice = mesh.slice(1.0.into());
    // println!("{:?}", slice);

}