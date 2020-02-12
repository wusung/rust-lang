fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    let _x = 2.0;

    let _y: f32 = 3.0;

    let _num = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotation = 56.7 / 32.2;

    let _remainer = 43 % 5;

    let _t = true;

    let _f: bool = false;

    let _c = 'c';
    let _z = 'Z';
    let _heart_eyed_cat = '😻';

    let _tup = (500, 6.4, 1);
    let (_a, _b, _c) = _tup;
    println!("The value of _a: {}", _a);

    let _x1: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = _x1.0;

    let _six_point_four = _x1.1;

    let _one = _x1.2;

    println!("{} {} {}", _five_hundred, _six_point_four, _one);

    let _a1 = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let _a2: [i32; 5] = [1, 2, 3, 4, 5];
    let _a3 = [3; 5];
    println!("_a3={}", _a3[0]);

    let _first = _a1[0];
    let _second = _a1[1];
    println!("_first={}, _second={}", _first, _second);

    let _index = 2;
    let _element = _a2[_index];
    println!("The value of _element is: {}", _element);

}
