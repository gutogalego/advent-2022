#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height

    }

    fn area51(&mut self) {
        self.width = self.width*2;
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:?}", rect1);
    dbg!(rect1);

    let rect2 = Rectangle::square(3);

    dbg!(rect2);
}
