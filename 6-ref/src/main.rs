fn main() {
   
//    a:= 100;
//    ptr:= &a 

// ptr := new(100);

let ref1 = &mut 100;
*ref1 = 200;

let ref2 = &mut 100; // create some memry put 100 in that and gives the ref
*ref2= 300;
{
let ref3 = ref1;
}

//let ref4 = ref1;

// let num = 100;
// let ref2 = &num;

let hash = 0x9F4A7C2B1183DE45 as u64;
let mut capacity = 16 as u64;
println!("{}",hash);

let index = hash & capacity-1;

let index2 = hash%capacity; // % is an arthimetic operations

println!("{}--{}",index,index2);

capacity = 64;

println!("{:b}",capacity); // 1000000

println!("{:b}",capacity-1); 

for i in 1..=64{
    println!("{} -->{:b}",i-1,i-1);
}

}
