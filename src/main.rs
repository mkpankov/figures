use figures::{Figure, Rectangle, Square};

mod figures {
    use std::fmt;

    pub trait Rect {
        fn width(&self) -> f64;
        fn length(&self) -> f64;
    }

    pub trait AreaRect: Rect {
        fn area(&self) -> f64
        {
            self.width() * self.length()
        }
    }

    pub trait Figure: AreaRect + fmt::Display { }

    pub struct Rectangle {
        width: f64,
        length: f64,
    }

    pub struct Square {
        side: f64,
    }

    impl Figure for Rectangle { }

    impl Figure for Square { }

    impl fmt::Display for Rectangle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "прямоугольник({}, {})", self.width, self.length)
        }
    }

    impl fmt::Display for Square {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "квадрат({}, {})", self.side, self.side)
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

    impl AreaRect for Rectangle { }

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

    impl AreaRect for Square { }
}

fn print_figures_and_areas(figures: &[&Figure])
{
    for f in figures.iter() {
        println!("Площадь {} равна {}", f, f.area());
    }
}

fn main() {
    let rect1 = Rectangle::new(3., 5.).unwrap();
    let rect2 = Rectangle::new(4., 6.).unwrap();

    let sq1 = Square::new(8.).unwrap();
    let sq2 = Square::new(4.).unwrap();

    print_figures_and_areas(
        &[&rect1, &rect2, &sq1, &sq2]);
}
