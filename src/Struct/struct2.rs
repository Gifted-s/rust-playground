#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn canhold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect1 = Rectangle {
        width: 50,
        height: 60,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };
    let canhold = rect1.canhold(&rect2);
    let area = rect.area(); 
    println!("Area {}", area); 
    println!("Rect: {:#?} ", rect);
    println!("Can Rect1 hold Rect2:  {}", canhold);
}
