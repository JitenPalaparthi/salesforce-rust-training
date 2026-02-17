fn main() {

    let mut  arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // 40 bytes
    let sum = sum_of(&arr1);
    println!("sum:{}", sum);

    let slice1 = &mut arr1[2..=3]; 

   // i32, i64

    
    slice1[0]=12312;
    let num: i32 = 100;
    let float: f64 = 12.34;

    println!("{} {}",num,float);
    println!("{:?}",slice1);

    println!("{:?}",arr1);
}


fn sum_of(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for v in arr {
        sum += v;
    }
    return sum;
}