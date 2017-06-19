use super::Area;

use std::fmt;

pub struct Circle {
    radius: f64,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "круг({})", self.radius)
    }
}

impl Circle {
    pub fn new(radius: f64) -> Option<Circle>
    {
        if radius > 0. {
            Some( Circle { radius } )
        } else {
            None
        }
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        ::std::f64::consts::PI * self.radius * self.radius
    }
}
