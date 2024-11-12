#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method definition.
impl Rectangle {
    // self would work like the 'this' keyword in other languages.
    // &self is a reference to the calling object.
    // The fist parameter of any rust method is 'self'
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Associate function that will be accessed using the Object
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let square = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    ); // Output: Area: 1500
}
