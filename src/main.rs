mod figures {
    pub struct Rectangle {
        width: f64,
        length: f64,
    }

    pub struct Square {
        side: f64,
    }

    impl Rectangle {
        pub fn new(width: f64, length: f64) -> Option<Rectangle> {
            if width > 0. && length > 0. {
                Some( Rectangle { length, width } )
            } else {
                None
            }
        }
        pub fn area(&self) -> f64
        {
            self.width * self.length
        }
    }

    impl Square {
        pub fn new(side: f64) -> Option<Square> {
            if side > 0. {
                Some( Square { side } )
            } else {
                None
            }
        }
        pub fn area(&self) -> f64
        {
            self.side * self.side
        }
    }
}


fn main() {
    let rect1 = figures::Rectangle::new(3., 5.).unwrap();
    let rect2 = figures::Rectangle::new(4., 6.).unwrap();;

    let sq1 = figures::Square::new(8.).unwrap();
    let sq2 = figures::Square::new(4.).unwrap();

    let rects = [&rect1, &rect2];
    let squares = [&sq1, &sq2];

    for r in rects.iter() {
        println!("Площадь равна {}", r.area());
    }

    for s in squares.iter() {
        println!("Площадь равна {}", s.area());
    }
}
