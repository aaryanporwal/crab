// Methods and Associate functions in Rust

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn new(length: u32, width: u32) -> Rectangle {
        Rectangle {
            length,
            width,
        }
    }

    fn clone(&self) -> Rectangle { // &self is instance of our Rectangle
        Rectangle{
            ..*self
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

}

fn main() {
    let rect1 = Rectangle::new(10, 5); // Calling associative function
    let mut rect2 = rect1.clone(); 
    rect2.set_width(20);

    println!("{:?}", rect1);
    println!("{:?}", rect2);
}
