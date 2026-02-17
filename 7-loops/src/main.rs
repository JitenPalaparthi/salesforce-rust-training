fn main() {
    fib(10);

    let mut i = 0;
    let mut a = 0;
    let mut b = 1;
    loop {
        if i > 10 {
            break;
        }
        let next = a + b;
        a = b;
        b = next;
        println!("{}", next);
        i += 1;
    }
    // loop as an expression

    let mut sum = 0;
    let mut i = 1;
    let result: i32 = loop {
        i += 1;
        sum += i;

        if i > 10 {
            break sum; // the break can be used to return a value
        }
    };

    println!("result:{}",result);
}

fn fib(n: u32) {
    let mut i = 0;
    let mut a = 0;
    let mut b = 1;

    while i <= n {
        let next = a + b;
        a = b;
        b = next;
        println!("{}", next);
        i += 1;
    }
}
