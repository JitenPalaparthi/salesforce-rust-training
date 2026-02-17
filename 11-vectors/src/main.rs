fn main() {
    let mut vec1 = vec![10, 20, 30];
    println!("len:{} cap:{}", vec1.len(), vec1.capacity());
    vec1[0] = 12312;
    vec1[1] = 3232;

    (&mut vec1).push(132); // no need to call like this now
    vec1.push(9879);
    vec1.push(6786);

    println!("len:{} cap:{}", vec1.len(), vec1.capacity());
    vec1.push(6786);

     println!("len:{} cap:{}", vec1.len(), vec1.capacity());
    let mut sum = 0;
   
    for v in &mut vec1{ 
        *v = *v*2;
        sum+=*v;
    }

    println!("sum:{}",sum);

     for v in &vec1{ 
        sum+= *v;
    }

      println!("sum:{}",sum);

    //vec1[0]=9999;

    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(100);
    vec1.push(200);
    vec1.push(300);
    vec1.push(400);
    
    vec1.pop();

    vec1.remove(2);

    let mut vec1 = Vec::with_capacity(100);
    
    vec1.push(100);
    vec1.push(200);
    vec1.push(300);
    vec1.push(400);
    vec1.push(500);

    let slice1:&[i32] = &vec1; // vec is converted to slice

    let sum = sum_of(slice1);

}

/*
struct Vec{
    ptr *mut T,
    len usize,
    cap usize,
}
*/

// vec is stored heap mem
// when things gets in heap memory, copy is not implemented


fn sum_of(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for v in arr {
        sum += v;
    }
    return sum;
}