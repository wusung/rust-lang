fn main() {
    let mut _s = String::from("hello");
    _s.push_str(", world");

    println!("{}", _s);

    // Ways Variables and Data Interact: Move
    let _s1 = String::from("hello");
    let _s2 = _s1;
    println!("{}, world", _s2);

    // _s1 will be drop
    // println!("{}, world", _s1);

    // Ways Variables and Data Interact: Clone
    let _a1 = String::from("hello");
    let _a2 = _a1.clone();
    println!("_a1={}, _a2={}", _a1, _a2);

    // Stack-Only Data: Copy
    let _x = 3;
    let _y = _x;
    println!("_x = {}, _y = {}", _x, _y);

    takes_ownership(_s);
    makes_copy(_x);

    let _s11 = gives_ownership(); // gives_ownership moves its return
                                  // value into s1

    let _s12 = String::from("hello"); // s2 comes into scope

    let _s13 = takes_and_gives_back(_s12); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3

    let _s21 = String::from("hello");
    let (_22, len) = calculate_length(_s21);
    println!("The length of '{}' is {}.", _22, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
