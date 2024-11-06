#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        return Self { width, height };
    }

    // This method does not take ownership of the struct.
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    // This method does not take ownership of the struct.
    fn sides(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    // This method takes ownership of the struct.
    fn double(self) -> Self {
        Self {
            width: self.width * 2,
            height: self.height * 2,
        }
    }

    // This method updates struct without taking ownership.
    fn update(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

fn main() {
    let rectangle = Rectangle::new(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    println!(
        "The sides of the rectangle are {:?} pixels.",
        rectangle.sides()
    );

    let mut doubled = rectangle.double();
    println!(
        "The area of the doubled rectangle is {} square pixels.",
        doubled.area()
    );

    doubled.update(30, 50);
    println!(
        "The area of the doubled rectangle after update is {} square pixels.",
        doubled.area()
    );
}