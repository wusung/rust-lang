fn main() {
    let mut _s = String::from("Hello world!");

    println!("{}", first_word(&_s));

    let _hello = &_s[0..5];
    let _world = &_s[6..11];
    println!("{}, {}", _hello, _world);

    let _s10 = String::from("hello");
    let _slice = &_s10[0..2];
    println!("{}", _slice);
    let _slice = &_s10[..2];
    println!("{}", _slice);

    let _len = _s10.len();
    let _slice = &_s10[3.._len];
    println!("{}", _slice);
    let _slice = &_s10[.._len];
    println!("{}", _slice);

    let _slice = &_s10[0.._len];
    println!("{}", _slice);
    let _slice = &_s10[..];
    println!("{}", _slice);

    println!("{}", first_word_v2(&_s10));

    let mut _s11 = String::from("hello world");
    let _word = first_word_v2(&_s11);
    _s11.clear();

    let _s12 = String::from("hello world");
    let _hello = &_s12[0..5];
    let _world = &_s12[6..11];
    println!("{} {}", _hello, _world);

    let _s13 = String::from("hello");
    let _slice1 = &_s13[0..2];
    let _slice2 = &_s13[..2];
    println!("{}{}", _slice1, _slice2);

    let _s14 = String::from("hello");
    let _len14 = _s14.len();
    let _slice3 = &_s14[3.._len14];
    let _slice4 = &_s14[3..];
    println!("{} {}", _slice3, _slice4);

    let _my_string = String::from("hello world");
    // let _word2 = first_word_v2(&_my_string[..]);
    // let _my_string_literal = "hello world";
    // let _word2 = first_word_v2(&_my_string_literal[..]);
    // let _word2 = first_word_v2(_my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}