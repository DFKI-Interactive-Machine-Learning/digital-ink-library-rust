use std::fs;

use crate::sketch::Sketch;
use crate::stroke::Stroke;

pub fn dump_stroke(stroke: &Stroke, file_path: &String) {
    let json_string = dumps_stroke(stroke);
    fs::write(file_path, json_string).expect("unable to write stroke to file");
}

pub fn dump_sketch(sketch: &Sketch, file_path: &String) {
    let json_string = dumps_sketch(sketch);
    fs::write(file_path, json_string).expect("unable to write sketch to file");
}

pub fn dumps_stroke(stroke: &Stroke) -> String {
    serde_json::to_string_pretty(stroke).unwrap()
}

pub fn dumps_sketch(sketch: &Sketch) -> String {
    serde_json::to_string_pretty(sketch).unwrap()
}

pub fn load_stroke(file_path: &String) -> Result<Stroke, String> {
    let contents = fs::read_to_string(file_path).expect("unable to read stroke from file");
    loads_stroke(contents)
}

pub fn load_sketch(file_path: &String) -> Result<Sketch, String> {
    let contents = fs::read_to_string(file_path).expect("unable to read stroke from file");
    match loads_sketch(contents) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => return Ok(s),
    };
}

pub fn load_sketches(file_path: &String) -> Result<Vec<Sketch>, String> {
    let contents = fs::read_to_string(file_path).expect("unable to read stroke from file");
    match loads_sketches(contents) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => return Ok(s),
    };
}

pub fn loads_stroke(serialized_string: String) -> Result<Stroke, String> {
    match serde_json::from_str(serialized_string.as_str()) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => return Ok(s),
    };
}

pub fn loads_strokes(serialized_string: String) -> Result<Vec<Stroke>, String> {
    match serde_json::from_str(serialized_string.as_str()) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => return Ok(s),
    };
}

pub fn loads_sketch(serialized_string: String) -> Result<Sketch, String> {
    match serde_json::from_str(serialized_string.as_str()) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => return Ok(s),
    };
}

pub fn loads_sketches(serialized_string: String) -> Result<Vec<Sketch>, String> {
    match serde_json::from_str(serialized_string.as_str()) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => return Ok(s),
    };
}
