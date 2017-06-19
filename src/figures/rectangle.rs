use super::{Area, Rect};

use std::fmt;

pub struct Rectangle {
    width: f64,
    length: f64,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "прямоугольник({}, {})", self.width, self.length)
    }
}

impl Rect for Rectangle {
    fn length(&self) -> f64
    {
        self.length
    }
    fn width(&self) -> f64
    {
        self.width
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

impl Rectangle {
    pub fn new(width: f64, length: f64) -> Option<Rectangle>
    {
        if width > 0. && length > 0. {
            Some( Rectangle { length, width } )
        } else {
            None
        }
    }
}
