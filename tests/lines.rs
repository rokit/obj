//   Copyright 2017 GFX Developers
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

use obj::{Group, Line, LineTuple, ObjData, Object};
use std::io::BufReader;

/// Test that [`std::fmt::Display`] is implemented correctly for
/// [`LineTuple`].
#[test]
fn line_tuple_display() {
    assert_eq!(LineTuple(0, None).to_string(), "1");
    assert_eq!(LineTuple(0, Some(0)).to_string(), "1/1");
}

#[test]
fn test_load_line() {
    let line_square = "
    v 0 0 0
    v 0 1 0
    v 1 1 0
    v 1 0 0
    l 1 2 3 4 1
    ";

    let line_square_vbo = [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]];

    let mut reader = BufReader::new(line_square.as_bytes());
    let obj = ObjData::load_buf(&mut reader).unwrap();

    for (a, b) in obj.position.iter().zip(line_square_vbo.iter()) {
        assert_eq!(a, b);
    }

    let object = obj.objects.first().unwrap();
    let group = object.groups.first().unwrap();

    let expected_lines = vec![Line(vec![
        LineTuple(0, None),
        LineTuple(1, None),
        LineTuple(2, None),
        LineTuple(3, None),
        LineTuple(0, None),
    ])];

    assert_eq!(group.lines, expected_lines)
}

#[test]
#[should_panic(expected = "LineHasNormalIndex { line_number: 5 }")]
fn test_load_line_with_normal() {
    let line_with_normals = "
    v 0 0 0
    v 1 0 0
    vt 0 0
    vn 0 0 0
    l 1/1/1 2/1/1
    ";

    let mut reader = BufReader::new(line_with_normals.as_bytes());
    let _ = ObjData::load_buf(&mut reader).unwrap();
}

#[test]
fn test_export_line() {
    let mut obj_data: ObjData = ObjData::default();
    obj_data.position = vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]];

    let name = "line test".to_string();
    let mut object = Object::new(name.to_owned());
    let mut group = Group::new(name.to_owned());

    group.lines = vec![Line(vec![
        LineTuple(0, None),
        LineTuple(1, None),
        LineTuple(2, None),
        LineTuple(3, None),
        LineTuple(0, None),
    ])];

    object.groups.push(group);
    obj_data.objects.push(object);

    let mut output = Vec::new();
    obj_data.write_to_buf(&mut output).unwrap();
    let output = String::from_utf8(output).unwrap();

    let expected_output = "# Generated by the obj Rust library (https://crates.io/crates/obj).\nv 0 0 0\nv 0 1 0\nv 1 1 0\nv 1 0 0\no line test\ng line test\nl 1 2 3 4 1\n".to_string();
    assert_eq!(output, expected_output);
}