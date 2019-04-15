#Slicer

[![Build Status](https://travis-ci.org/Quyzi/slicer.svg?branch=master)](https://travis-ci.org/Quyzi/slicer)
![minimum rust 1.33](https://img.shields.io/badge/rust-1.33%2B-orange.svg)

```rust
    let stl: STLFile = match STLFile::new("fixtures/3DBenchy_bin.stl".to_string()) {
        Ok(stl) => stl,
        Err(e) => panic!("{}", e),
    };
    let mut mesh = Mesh::from(stl);
    mesh.normalize(1.0.into(), 1.0.into(), 0.0.into());
    
    let mut height: R32 = 0.2.into();
    let mut slices: Vec<Slice> = Vec::new();

    for _ in 0..238 {
        height = height.mul_add(1.0.into(), 0.2.into());
        slices.push(mesh.slice_at( height ));
    }
```