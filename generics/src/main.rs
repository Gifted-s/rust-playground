// generics are used to prevent duplication

fn main1() {
    let number_list = vec![102, 34, 6000, 89, 54, 2, 42, 8];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let number_list = vec![6766766, 989, 990990, 89, 54, 2, 42, 8];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);
}

//Ise trait to make this function work for multiple types?
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number
        }
    }
    largest
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<U> Point<U> {
//     fn x(&self) -> &U {
//         &self.x
//     }
// }

// impl Point<f64> {
//     fn y(&self) -> f64 {
//         &self.y
//     }
// }

// fn main2() {
//     let p = Point { x: 5, y: 10 };
//     p.x();
//     let p1 = Point { x: 5.0, y: 1.0 };
//     p1.y();

//     enum Option<T> {
//         Some(T),
//         None,
//     }

//     enum Result<T, E> {
//         Some(T),
//         None,
//     }

//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: 5.0, y: 10 };
//     let p3 = Point { x: 5.0, y: 10.4 };
// }


struct Point<T, U>{
    x:T,
    y:U
}

impl<T, U> Point<T, U>{
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main2(){
    let p1 = Point{x:5, y:10.4};
    let p2 = Point{x:"Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

enum Option<T>{
    Some(T),
    None,
}

// Using Option Enum which takes in any param T does not incure any extra cost because at
// compile time rust would create two Option Enum one for integer and the other for float 64. Note: this happens at compile time not runtime.
fn main(){
    let integer = Option::Some(5);
    let float = Option::Some(5.0);


}