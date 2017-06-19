use super::Area;

use std::fmt;

pub struct Ellipse {
    a: f64,
    b: f64,
}

impl fmt::Display for Ellipse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "эллипс({}, {})", self.a, self.b)
    }
}

impl Ellipse {
    pub fn new(a: f64, b: f64) -> Option<Ellipse>
    {
        if a > 0. && b > 0. {
            Some( Ellipse { a, b } )
        } else {
            None
        }
    }
}

impl Area for Ellipse {
    fn area(&self) -> f64 {
        ::std::f64::consts::PI * self.a * self.b
    }
}
