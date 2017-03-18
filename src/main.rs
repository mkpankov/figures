#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Figure {
    origin: Point,
}

#[derive(Debug)]
struct Rectangle {
    figure: Figure,
    width: f64,
    length: f64,
}

#[derive(Debug)]
struct Square {
    rectangle: Rectangle,
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.length
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.rectangle.width * self.rectangle.length
    }
}

fn main() {
    let origin = Point { x: 0., y: 0. };
    let rect1 = Rectangle {
        figure: Figure {
            origin: origin
        },
        width: 5.,
        length: 7.,
    };
    let rect2 = Rectangle {
        figure: Figure {
            origin: origin
        },
        width: 3.,
        length: 9.,
    };
    let sq1 = Square {
        rectangle: Rectangle {
            figure: Figure {
                origin: origin
            },
            width: 4.,
            length: 4.,
        }
    };

    println!("rect1 = {:?}", rect1);
    println!("area of rect1 = {}", rect1.area());
    println!("rect2 = {:?}", rect2);
    println!("area of rect2 = {}", rect2.area());
    println!("sq1 = {:?}", sq1);
    println!("area of sq1 = {}", sq1.area());
}
