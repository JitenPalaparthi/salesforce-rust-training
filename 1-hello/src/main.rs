
fn main() {
    println!("Hello, world!");

    let x:i32 = 9999;

    println!("{}",x);

    let y = x; // type inference, copy

    let ok = true;

    let z = x; // it is a copy

    let mut hello: &str = "你好 World";
    
    hello = "Hello World, How are you doing";

    let mut s1: String = "Hello World".to_string(); // heap memory
{
    let s2: String = s1; // not a copy trait 

} // Drop fn --> instead of GC , the drop trait is imoplemented so that the heap is deallocated from the memory at this point


   // let s3 = s2;

    let hello1: &str = hello; //L en

    println!("{}",x);

    //let mut pi:f32 = 3.145;

    //println!("{}",pi);

}

fn greet(){
    println!("Hello SalesForce folks");
}

// isize usize i8 u8 ... i128 u128 
// f32 f64 f128 
// bool 
// &str

// String

// trait

// monomorphization

// marcros
// code generated
// dead code elimination
// costants evaluated
// borrow checker --> ensure all the rules of rust memory management are followed , if there is any problem, rust does not compile 
