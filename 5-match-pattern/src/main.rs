fn main() {
    let day = 4 as u8;

    match day {
        1 => println!("Sunday"),
        2 => println!("Mondy"),
        3 => println!("Tuesday"),
        4 => println!("Wednesday"),
        5 => println!("Thursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => println!("no day"),
    }

    // match patten as an expression

    let reuslt = match day {
        1 => "Sunday",
        2 => "Mondy",
        3 => "Tuesday",
        4 => "Wednesday",
        5 => "Thursday",
        6 => "Friday",
        7 => "Saturday",
        _ => "no day",
    };

    let ch: char = 'h';

    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => println!("char is vovel"),
        _ => println!("consonent or special char"),
    }

    let x: u8 = 123;

    match x {
        0..=50 => println!("{x} is between 0-50"),
        51..=100 => println!("{x} is between 51-100"),
        101..=200 => println!("{x} is between 101-200"),
        _ => println!("{x} is above 200"),
    }

    let num = 123;

    match num {
        x if x % 2 == 0 => {
            println!("{} is even number", x)
        }
        _ => println!("{} is odd number", x),
    }

    let d1: Direction = Direction::South;

    match d1 {
        Direction::East => println!("East direction"),
        Direction::West => println!("West direction"),
        Direction::South => println!("South direction"),
        Direction::North => println!("North direction"),
    }

    let mut s1 = Shape::NoShape;

    s1 = Shape::Square(12.4 as f32);

    match s1 {
        Shape::NoShape => println!("no shape nothing"),
        Shape::Square(s) => println!("Area of Square:{}", s * s),
        Shape::Rect(l, b) => println!("Area of Rect:{}", l * b),
        Shape::Cuboid(l, b, h) => {
            let a = l * b * h;
            println!("Area of Cuboid:{:.2}", a);
        }
    }

    let mut num:Option<i32>=None;
   
    num = Some(12312);

    // let num2 = Some(12312);

    // let float1 = Some(123.123);

    match num{
        Some(n)=>println!("using match:{}",n*n),
        None => {println!("Nothing to calculate")},
    }

    if let Some(n)=num{
        println!("Using if let:{}",n*n);
    }

    // write if let on shape

    let d1 = Direction::South;

    if let Direction::East=d1{
        println!("It is direction of east");
    }else if let Direction::West=d1{
        println!("It is direction of west");    
    }else{
        println!("It is direction of south or north :)")
    }
}

enum Direction {
    East,
    West,
    South,
    North,
}

enum Message {
    Quit,
    Move(i32, i32),
    Write(String),
}

enum Shape {
    Square(f32),
    Rect(f32, f32),
    Cuboid(f32, f32, f32),
    NoShape,
}
