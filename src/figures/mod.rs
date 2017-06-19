mod rectangle;
mod square;
mod ellipse;
mod circle;

pub use self::rectangle::Rectangle;
pub use self::square::Square;
pub use self::ellipse::Ellipse;
pub use self::circle::Circle;

use std::fmt;

pub trait Rect {
    fn width(&self) -> f64;
    fn length(&self) -> f64;
}

pub trait Area {
    fn area(&self) -> f64;
}

pub trait Figure: Area + fmt::Display { }

impl<T: Area + fmt::Display> Figure for T { }
