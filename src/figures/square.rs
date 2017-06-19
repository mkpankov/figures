use super::{Area, Rect};

use std::fmt;

pub struct Square {
    side: f64,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "квадрат({}, {})", self.side, self.side)
    }
}

impl Square {
    pub fn new(side: f64) -> Option<Square>
    {
        if side > 0. {
            Some( Square { side } )
        } else {
            None
        }
    }
}

impl Rect for Square {
    fn length(&self) -> f64
    {
        self.side
    }
    fn width(&self) -> f64
    {
        self.side
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
