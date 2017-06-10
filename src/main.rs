use figures::{Area, Rectangle, Square};

mod figures {
    pub trait Area {
        fn area(&self) -> f64;
    }

    pub struct Rectangle {
        width: f64,
        length: f64,
    }

    pub struct Square {
        rectangle: Rectangle,
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

    impl Area for Rectangle {
        fn area(&self) -> f64
        {
            self.width * self.length
        }
    }

    impl Square {
        pub fn new(side: f64) -> Option<Square>
        {
            if side > 0. {
                Some( Square { rectangle: Rectangle { length: side, width: side } } )
            } else {
                None
            }
        }
    }

    impl Area for Square {
        fn area(&self) -> f64
        {
            self.rectangle.area()
        }
    }
}


fn main() {
    let rect1 = Rectangle::new(3., 5.).unwrap();
    let rect2 = Rectangle::new(4., 6.).unwrap();

    let sq1 = Square::new(8.).unwrap();
    let sq2 = Square::new(4.).unwrap();

    let figures_with_area: [&Area; 4] = [&rect1, &rect2, &sq1, &sq2];

    for f in figures_with_area.iter() {
        println!("Площадь равна {}", f.area());
    }
}
