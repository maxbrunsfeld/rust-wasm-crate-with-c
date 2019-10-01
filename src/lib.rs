use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(C)]
pub struct Point {
    pub line: u32,
    pub column: u32,
}

extern "C" {
    fn add_points_in_c(a: Point, b: Point) -> Point;
}

#[wasm_bindgen]
pub fn add_points_in_rust(a: Point, b: Point) -> Point {
    unsafe { add_points_in_c(a, b) }
}

#[wasm_bindgen]
pub fn new_point(line: u32, column: u32) -> Point {
    Point { line, column }
}
