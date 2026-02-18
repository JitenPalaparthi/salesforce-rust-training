use std::collections::HashMap;
fn main() {
    let num1: u64 = 0xcc112233ffaabc;
    let num2: u64 = 0b110011000111100011;
    let num3: u64 = 123456;

    let (x, y) = (897, 456);
    println!("{} {}", x % 4, y % 8);

    // creating a map

    // ==

    //    let arr1 = [1,2,3];
    //    let arr2 = [1,2,3];
    //    if arr1==arr2{

    //    }
    let mut map: HashMap<u32, String> = HashMap::new();

    map.insert(560086, "Bangalore-1".to_string());
    map.insert(522001, "Guntur-1".to_string());
    map.insert(695011, "Trivandrum".to_string());
    map.insert(560096, "Bangalore-2".to_string());

    let v: Option<&String> = map.get(&695011);
    // None
    // Some

    match v {
        None => println!("No value from the map"),
        Some(s) => println!("{}", s),
    }

    if let Some(v) = map.get(&560086) {
        println!("{}", v);
    }

    for (k, v) in &map {
        println!("key:{} value:{}", k, v);
    }
    
}

// HashFunc
