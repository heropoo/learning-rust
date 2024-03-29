#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn perimeter(self: &Self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The value of rect1 is \n{:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        //area(&rect1)
        rect1.area()
    );

    println!("The perimeter of the rectangle is {} .", rect1.perimeter());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    dbg!(&rect2);

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("The value of sq is {:?}", sq);

    println!("The area of sq is {}", _area(&sq));
}

fn _area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
