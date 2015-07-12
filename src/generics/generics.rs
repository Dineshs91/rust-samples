struct Rectangle {
    l: i32,
    b: i32
}

struct Square {
    s: i32
}

trait Area {
    fn find_area(&self) -> i32;
}

impl Area for Rectangle {
    fn find_area(&self) -> i32 {
        self.l * self.b
    }
}

impl Area for Square {
    fn find_area(&self) -> i32 {
        self.s * self.s
    }
}

fn calculate_area<T: Area>(val: T) -> i32 {
    val.find_area()
}

fn main() {
    let c = Rectangle{ l: 10, b: 20 };
    let s = Square{ s: 10 };
    
    println!("{}", calculate_area(c));
    println!("{}", calculate_area(s));
}