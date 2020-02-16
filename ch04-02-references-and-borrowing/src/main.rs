fn main() {
    let _s1 = String::from("hello");
    
    let _len = calculate_length(&_s1);

    println!("The length of '{}' is {}.", _s1, _len);

    let mut _s = String::from("hello");
    change(&mut _s);
    println!("The value of string is: {}", _s);

    {
        let _r1 = &mut _s;
        println!("The value of r1 is: {}", _r1);
    }
    let _r2 = &mut _s;
    println!("The value of r2 is: {}", _r2);

    let mut _s10 = String::from("hello");
    let _r11 = &_s10;
    let _r12 = &_s10;
    println!("{}, {}", _r11, _r12);

    let _r13 = &mut _s10;
    println!("{}", _r13);

    println!("{}", dangle());
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn dangle() -> String {
    return String::from("dangle()");
}