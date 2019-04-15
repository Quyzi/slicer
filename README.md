# Slicer

[![Build Status](https://travis-ci.com/Quyzi/slicer.svg?branch=master)](https://travis-ci.com/Quyzi/slicer)
![minimum rust 1.33](https://img.shields.io/badge/rust-1.33%2B-orange.svg)

## Goal
To make a slicer to generate gcode for 3d printers. 

## TODO
- [ ] Optimizations. Code is criminally inefficient in many places. 
- [ ] Determine interior areas of slices for infill
- [ ] Mesh Transformations (rotate, align plane on Z axis)
- [ ] Mesh Analysis 
- - [ ] Solid surfaces
- - [ ] Thin walls
- - [ ] Overhangs
- [ ] Actual gcode generation
- - [ ] Exterior paths
- - [ ] Infill
- - [ ] Support materials
- - [ ] Customizable start and end gcode
- - [ ] Customizable per-layer gcode
- [ ] Support for more file formats
- - [X] Binary STL
- - [ ] ASCII STL
- - [ ] Obj
- - [ ] STEP
- - [ ] 3mf