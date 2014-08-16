//   Copyright 2014 Colin Sherratt
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.


extern crate obj = "obj-rs";

use obj::ObjFile;
use std::io::BufReader;

static square: &'static str = "
v 0 1 0
v 0 0 0
v 1 0 0
v 1 1 0
f 1 2 3 4
";

static square_vbo: &'static [[f32, ..3]] = &[
    [0., 1., 0.],
    [0., 0., 0.],
    [1., 0., 0.],
    [1., 1., 0.],
];

#[test]
fn test_load_square() {
    let mut reader = BufReader::new(square.as_bytes());
    let obj = ObjFile::load(&mut reader);

    let (v, _) = obj.vertex_position();

    for (a, b) in v.iter().zip(square_vbo.iter()) {
        assert_eq!(a.as_slice(), b.as_slice());
    }
}

static cube: &'static str = "
v 0 1 1
v 0 0 1
v 1 0 1
v 1 1 1
v 0 1 0
v 0 0 0
v 1 0 0
v 1 1 0
# 8 vertices

o cube
g front cube
f 1 2 3 4
g back cube
f 8 7 6 5
g right cube
f 4 3 7 8
g top cube
f 5 1 4 8
g left cube
f 5 6 2 1
g bottom cube
f 2 6 7 3
# 6 elements
";

static cube_vbo: &'static [[f32, ..3]] = &[
    [0., 1., 1.],
    [0., 0., 1.],
    [1., 0., 1.],
    [1., 1., 1.],
    [1., 1., 0.],
    [1., 0., 0.],
    [0., 0., 0.],
    [0., 1., 0.]
];

static cube_names: &'static [&'static str] = &[
    "front cube",
    "back cube",
    "right cube",
    "top cube",
    "left cube",
    "bottom cube",
];


#[test]
fn test_load_cube() {
    let mut reader = BufReader::new(cube.as_bytes());
    let obj = ObjFile::load(&mut reader);

    let (v, _) = obj.vertex_position();

    for (a, b) in v.iter().zip(cube_vbo.iter()) {
        assert_eq!(a.as_slice(), b.as_slice());
    }

    for obj in obj.object_iter() {
        assert_eq!(obj.name.as_slice(), "cube");
        for (g, &name) in obj.group_iter().zip(cube_names.iter()) {
            assert_eq!(name, g.name.as_slice());
        }
    }
}

static cube_negative: &'static str = "
v 0 1 1
v 0 0 1
v 1 0 1
v 1 1 1
f -4 -3 -2 -1

v 1 1 0
v 1 0 0
v 0 0 0
v 0 1 0
f -4 -3 -2 -1

v 1 1 1
v 1 0 1
v 1 0 0
v 1 1 0
f -4 -3 -2 -1

v 0 1 0
v 0 1 1
v 1 1 1
v 1 1 0
f -4 -3 -2 -1

v 0 1 0
v 0 0 0
v 0 0 1
v 0 1 1
f -4 -3 -2 -1

v 0 0 1
v 0 0 0
v 1 0 0
v 1 0 1
f -4 -3 -2 -1
";

#[test]
fn test_load_cube_negative() {
    let mut reader = BufReader::new(cube_negative.as_bytes());
    let obj = ObjFile::load(&mut reader);

    let (v, _) = obj.vertex_position();

    for (a, b) in v.iter().zip(cube_vbo.iter()) {
        assert_eq!(a.as_slice(), b.as_slice());
    }
}