use std::fmt;

pub struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point { x: 10, y: 20 };
    println!("{}", point); // 输出: Point(10, 20)
}
