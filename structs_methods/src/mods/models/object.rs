// object.rs
// *******************************************************

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Object {
    pub width: u32,
    pub height: u32,
}
// *******************************************************

// implementation
impl Object {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
// *******************************************************

impl fmt::Display for Object {
    // @override
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        write!(format, "*. ({}, {}) & Area: {}", self.width, self.height, self.area())
    }
}
// *******************************************************
