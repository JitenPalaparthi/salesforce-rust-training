fn main() {
    let str1 = "你好 World";

    for c in str1.chars() {
        println!("{}", c);
    }

    println!("bytes array");
    for c in str1.bytes() {
        println!("{}", c as char);
    }

    let mut s1 = "你好 World".to_string();

    for c in (&s1).chars() {
        println!("{}", c);
    }

    for c in s1.bytes() {
        println!("{}", c as char);
    }

    s1.push_str(".How are you doing");
    println!("{}", s1);

    let mut done = false;
    for i in 1..=5 {
        if done {
            break;
        }
        for j in 1..=5 {
            println!("i:{} j:{}", i, j);
            if j == 3 {
                done = true;
                break;
            }
        }
    }

    // to use a label
    println!("\nUsing a label");

    'outer: for k in 1..=5 {
        for i in 1..=5 {
            for j in 1..=5 {
                println!("k:{}  i:{} j:{}", k, i, j);
                if j == 3 {
                    break 'outer;
                }
            }
        }
    }
}
