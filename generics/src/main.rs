fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest} in number_list: {number_list:?}");

    let p = Point { x: 5, y: 10 };
    println!("p.x = {} p.y = {}", p.x(), p.y);
}

struct Point<T> {
    x: T,
    pub y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
