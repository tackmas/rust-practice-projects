struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        return self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 12
    };

    println!("{}", rect.area());
}   
