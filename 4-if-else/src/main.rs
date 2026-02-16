fn main() {
    let mut gender: char = 'F';
    gender = '😊';

    let mut gender_byte: u8 = 'M' as u8;

    gender_byte = '😊' as u8;

    println!("{} {}", gender, gender_byte);

    let x: u32 = 12312312;
    println!("{:b}", x); // 1011101111011110 11111000

    let y: u8 = x as u8; //                   11111000
    println!("{:b}", y);

    let c = {
        let (a, b) = (10, 20);
        (a + b) + 10 + (b - a) * 2 + (a * 10) / 2 + 10 // may be split into 200-300 ASM 
    };

    println!("{}", c);

    let (age, gender) = (41 as u8, 'M');

    if age >= 18 && (gender == 'F' || gender == 'f') {
        println!("She is eligible for marriage")
    } else if age >= 21 && (gender == 'M' || gender == 'm') {
        println!("He is eligible for marriage")
    } else {
        println!("invalid gender or not elibible for marriage")
    }

    // if else can be used as expressions

    let reuslt = if age >= 18 && (gender == 'F' || gender == 'f') {
        "She is eligible for marriage"
    } else if age >= 21 && (gender == 'M' || gender == 'm') {
        "He is eligible for marriage"
    } else {
        "invalid gender or not elibible for marriage"
    };

    let num = 123;

    let result = if num%2==0{
        "even"
    }else{
        "ood"
    };
 
}

// There is no implicit type casting in rust
