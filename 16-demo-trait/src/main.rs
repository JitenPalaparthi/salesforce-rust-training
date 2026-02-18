pub mod shapes;

use crate::shapes::{IShape,Rect,Square};


fn main() {
let r1 = Rect::new(12.4, 11.9);
let s1= Square::new(25.35);

shape1(r1); // shape_rect(r:Rect)
shape1(s1); // shape_square(s:Square)


shape2(&r1); // 
shape2(&s1); // 


shape3(r1); // shape_rect_1(r:Rect)
shape3(s1); // shape_square_1(s:Square)

shape4(r1); // shape_rect_2(r:Rect)
shape4(s1); // shape_square_2(s:Square)

shape5(Box::new(r1));
shape5(Box::new(s1));

let mut shapes: Vec<Box<dyn IShape>> = Vec::new();

shapes.push(Box::new(r1));
shapes.push(Box::new(s1));


for s in shapes{
    shape5(s);
    // shape2(s.as_ref());
}

//println!("r1:{:?}",r1);
}

fn shape1(ishape: impl IShape){
    println!("Area:{:.2}",ishape.area());
    println!("Perimeter:{:.2}",ishape.perimeter());
}

fn shape2(ishape: &dyn IShape){
    println!("Area:{:.2}",ishape.area());
    println!("Perimeter:{:.2}",ishape.perimeter());
}

fn shape3<T:IShape>(ishape: T){ // Is it a static or dynamic
    println!("Area:{:.2}",ishape.area());
    println!("Perimeter:{:.2}",ishape.perimeter());
}

fn shape4<T>(ishape: T) where T:IShape{ // Is it a static or dynamic
    println!("Area:{:.2}",ishape.area());
    println!("Perimeter:{:.2}",ishape.perimeter());
}


fn shape5(ishape: Box<dyn IShape>){
    println!("Area:{:.2}",ishape.area());
    println!("Perimeter:{:.2}",ishape.perimeter());
}