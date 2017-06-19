use figures::{Figure, Rectangle, Square, Ellipse, Circle};

mod figures {
    use std::fmt;

    pub trait Rect {
        fn width(&self) -> f64;
        fn length(&self) -> f64;
    }

    pub trait Area {
        fn area(&self) -> f64;
    }

    pub trait Figure: Area + fmt::Display { }

    pub struct Rectangle {
        width: f64,
        length: f64,
    }

    pub struct Square {
        side: f64,
    }

    pub struct Ellipse {
        a: f64,
        b: f64,
    }

    pub struct Circle {
        radius: f64,
    }

    impl Figure for Rectangle { }

    impl Figure for Square { }

    impl Figure for Ellipse { }

    impl Figure for Circle { }

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

    impl fmt::Display for Ellipse {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "эллипс({}, {})", self.a, self.b)
        }
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "круг({})", self.radius)
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

    impl Area for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.width
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

    let ellipse1 = Ellipse::new(1., 2.).unwrap();
    let ellipse2 = Ellipse::new(2., 4.).unwrap();

    let circle1 = Circle::new(1.).unwrap();
    let circle2 = Circle::new(2.).unwrap();

    print_figures_and_areas(
        &[&rect1, &rect2, &sq1, &sq2, &ellipse1, &ellipse2, &circle1, &circle2]);
}
