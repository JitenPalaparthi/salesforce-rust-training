fn main() {
    let mut arr1: [i32; 5] = [10, 12, 13, 14, 15]; // The length to be known to the compiler

    let mut sum = 0;
    for v in arr1 {
        sum += v;
    }
    println!("sum:{}", sum);

    let mut arr2: [i32; 10000] = [0; 10000]; // can assign an array with 10000 elements with 0

    for i in 0..arr2.len() {
        arr2[i] = i as i32;
    }

    for i in arr2 {
        println!("{}", i);
    }

    let arr2d: [[i32; 2]; 2] = [[1, 2], [3, 4]];

    // [i32; 2]

    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = sum_of1(arr1);
    println!("sum:{}", sum);

    let arr2: [i32; 3] = [1, 2, 3];
    let sum = sum_of2(arr2);
    println!("sum:{}", sum);

    let sum= sum_of(&arr1);
      println!("sum:{}", sum);

      let sum= sum_of(&arr2);
      println!("sum:{}", sum);


    let s1 = "Hello World".to_string();

    let s2: &str = "Hello World";

    let l1 = get_len(&s1);
    let l2 = get_len(s2);
    // let arr_ref = &arr1;
}

// len, raw_ptr --> 
fn get_len(s: &str) -> u32 {
    s.len() as u32
}

fn sum_of(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for v in arr {
        sum += v;
    }
    return sum;
}

fn sum_of1(arr: [i32; 10]) -> i32 {
    let mut sum = 0;
    for v in arr {
        sum += v;
    }
    return sum;
}

fn sum_of2(arr: [i32; 3]) -> i32 {
    let mut sum = 0;
    for v in arr {
        sum += v;
    }
    return sum;
}
