fn main() {
    let mut s1: String = String::from("Hello World");

    {
        let s2 = &s1; // ownership transfer 

        let s3 = &s1;

        let s4 = &s1;
        println!("{}", s2);
    }
    
        let mut s5 = &mut s1;

        let l = str_mut_get_len_borrow(&mut s5, "Hello World");
       // println!("{}", s1);
    

        let mut s6 = &mut s1;

        let l = str_mut_get_len_borrow(&mut s6, "Hello World");

       // println!("{}", s1);
    
}

fn str_mut_get_len_take_ownership(mut s: String, mutstr: &str) -> u32 {
    s.push_str(mutstr);
    s.len() as u32
    // dropped here
}

fn str_mut_get_len_return_ownership(mut s: String, mutstr: &str) -> (String, u32) {
    s.push_str(mutstr);
    let l = s.len();
    (s, l as u32)
    // dropped here
}

fn str_mut_get_len_borrow(mut s: &mut String, mutstr: &str) -> u32 {
    s.push_str(mutstr);
    s.len() as u32
}

// Immutable borrow
// There can be any number of immutable borrows

// Mutable borrow
// or can only be one mutable borrow per a scope
