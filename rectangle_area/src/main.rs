#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 25,
        height: 45,
    };

    println!("The area of the regtangle is {} square pixels.", rect1.area());
}
