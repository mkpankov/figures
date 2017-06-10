// прямоугольник
struct Rectangle {
    // ширина
    width: f64,
    // длина
    length: f64,
}

// квадрат
struct Square {
    // сторона
    side: f64,
}

impl Rectangle {
    fn new(width: f64, length: f64) -> Option<Rectangle> {
        if width > 0. && length > 0. {
            Some( Rectangle { length, width } )
        } else {
            None
        }
    }
    fn area(&self) -> f64
    {
        self.width * self.length
    }
}

impl Square {
    fn new(side: f64) -> Option<Square> {
        if side > 0. {
            Some( Square { side } )
        } else {
            None
        }
    }
    fn area(&self) -> f64
    {
        self.side * self.side
    }
}

fn main() {
    let rect1 = Rectangle::new(3., 5.).unwrap();
    let rect2 = Rectangle::new(4., 6.).unwrap();;
    let rect3 = Rectangle::new(-4., 6.).unwrap();;

    let sq1 = Square::new(8.).unwrap();
    let sq2 = Square::new(4.).unwrap();

    let rects = [&rect1, &rect2];
    let squares = [&sq1, &sq2];

    for r in rects.iter() {
        println!("Площадь равна {}", r.area());
    }

    for s in squares.iter() {
        println!("Площадь равна {}", s.area());
    }
}
