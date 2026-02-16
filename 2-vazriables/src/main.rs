fn main() {
    //let s1 = "Hello World!".to_string();

  
   // println!("{}",s1);
    println!("About tuple");

   let mut t1: (i32, i32, bool, &str)=(10,20,true,"Hello WOrld");
   
   let (a,b,c) = (10,20,30);

   println!("sum:{}",a+b+c);

   println!("a:{} b:{} ok:{} str:{}",t1.0,t1.1,t1.2,t1.3);

   let (a,b,c,d)=t1;

    let mut s1: String = String::from("Hello World");
    let l: u32 = str_mut_get_len1(s1, ".How are you doing");

    let mut s2 = "Hello World".to_string();

    let mut l: u32 =0;

    (s2,l) = str_mut_get_len2(s2, " How are you doing");

    println!("{} {}",s2,l);

//     let a = 100;

//     let a = 200;

//     let a = true;

//     let a: &str = "hello Wrold";

//     let a = 1.34 as f32;
// {
//     let a = "Hello Salesforce";
//     println!("{}",a);
// }


}

fn str_mut_get_len1(mut s: String, mutstr: &str) -> u32 {
    s.push_str(mutstr);
    s.len() as u32
    // dropped here
}

fn str_mut_get_len2(mut s: String, mutstr: &str) -> (String,u32) {
    s.push_str(mutstr);
    let l = s.len();
    (s,l as u32)
    // dropped here
}

