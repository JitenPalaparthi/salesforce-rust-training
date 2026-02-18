static mut GLOBAL: i32 = 100; // Data segment , static lifetime

fn main() {
    const PI: f32 = 3.14; // Data segment

    // let v=2.0;

    const PI2: f32 = ((PI * PI) * 2.0) / 2.0; // This is evaluated by the compiler

    let mut s1 = "hello world".to_string(); // s1 is on stack and data is on heap

    let mut num = 100; // stack 

    let mut num_box = Box::new(100); // num_box is on stack and the data is on heap
    {
        let mut s = sq(100);
        println!("{}", s);

        println!("{:p}", &s);
        println!("{:p}", s.as_mut());
        println!("{:p}", &*s);
    } // it drops the heap as well as s
    // drop(s.ptr)

    let s: Box<i32>;
    {
        s = sq(100);
    }
    println!("{}", s);

    // {
    // let a = 100;
    // }
    // println!("{}",a);

    let mut x = 100;

    let ptr_const: *const i32 = &x; // raw pointer

    let ptr_mut: *mut i32 = &mut x; // mut raw pointer

    unsafe {
        println!("{}", *ptr_const);
        *ptr_mut = 200;
    }
    
    unsafe {
        GLOBAL += 10;
        println!("{}", unsafe { GLOBAL });
    }
}

fn sq(num: i32) -> Box<i32> {
    let result = Box::new(num * num); // create a varialbe 
    return result;
}

// func sq(num int) *int{
//     result:= num * num;  // create a variable
//     return &result // return a pointer
// }
