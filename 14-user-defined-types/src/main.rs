use std::{fmt, mem, ops::Not};
fn main() {
    let c1 = Circle(25.34);

    println!("{:?}", c1);
    let c2 = Circle::new(123.5);
    println!("{:?}", c2);

    let a1 = c2.area(); // the ownership of c2 is going to area method
    let p1 = c2.perimeter();

    println!("area:{:.2} perimeter:{:.2}", a1, p1);

    let mut r1 = Rect::new(12.4, 15.6);

    let a1 = r1.area();
    let p1 = r1.perimeter();
    println!("area:{:.2} perimeter:{:.2}", a1, p1);
    println!("rect:{}", r1);

    let r2 = r1.clone(); //

    println!("{:?}", r1.a);

    let r3: &mut Rect = &mut r1; // mutable borrow

    let a1 = r3.area(); // mutable borrow
    let p1 = r3.perimeter();
    println!("area:{:.2} perimeter:{:.2}", a1, p1);
    println!("rect:{}", r3);

    let r4 = r1; // no ownership transfer, bcz copy has been implemented

    //let n1:Nothing = Nothing { };

    Nothing::greet();
    Empty::greet();

    let n1 = Nothing{};
    let e1= Empty{};

    println!("Nothing: {}",mem::size_of::<Nothing>());
    println!("Empty: {}",mem::size_of::<Empty>());

    println!("pointer: {:p}",&n1);
    println!("pointer: {:p}",&e1);

    let r_box1 = Box::new(Rect::new(10.4,13.5));


}

#[derive(Debug)] // What does this trait does, it implemens display trainitn automatically
struct Circle(f32); // Unit Structure

impl Circle {
    // fn new(s:f32)->Circle{
    fn new(s: f32) -> Self {
        // Self the same type with values
        Circle(s)
    }

    fn area(&self) -> f64 {
        (self.0 * self.0) as f64
    }

    fn perimeter(&self) -> f64 {
        (self.0 * 4.0) as f64
    }
}

struct Rect {
    l: f32,
    b: f32,
    a: f64,
    p: f64,
}

impl Rect {
    fn new(l: f32, b: f32) -> Self {
        Rect {
            l,
            b,
            a: 0.0,
            p: 0.0,
        }
    }

    fn defaults() -> Self {
        Rect {
            l: 0.0,
            b: 0.0,
            a: 0.0,
            p: 0.0,
        }
    }

    fn area(&mut self) -> f64 {
        self.a = (self.l * self.b) as f64;
        return self.a;
    }

    fn perimeter(&mut self) -> f64 {
        self.p = 2.0 * (self.l + self.b) as f64;
        return self.p;
    }
}

impl std::fmt::Display for Rect {
 //   use std::fmt::Result;
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(l:{}, b:{} a:{:.2} p:{:.2})", self.l, self.b, self.a, self.p)
    }
}

impl Clone for Rect{
  fn clone(&self) -> Self{
    Self { l: self.l, b: self.b, a: self.a, p: self.p }
  }
}

impl Copy for Rect{} // trait bound for this


struct Nothing{} // empty structure

struct Empty{}

impl Nothing{
    fn greet(){
        println!("Hello Nothing structure")
    }
}

impl Empty{
    fn greet(){
        println!("Hello Empty structure")
    }
}