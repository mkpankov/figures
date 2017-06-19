mod figures;

use figures::{Figure, Rectangle, Square, Ellipse, Circle};

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
