fn main() {
    let rect = Rect {
        width: 40f64,
        height: 40f64
    };


    println!("Area of w:{}, h:{}", rect.width, rect.height);

    let area = rect.calculate_area();
    println!("is {}", area);
}

struct Rect {
    width: f64,
    height: f64
}

trait HasArea {
    fn calculate_area(&self) -> f64;
}

impl HasArea for Rect {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
}