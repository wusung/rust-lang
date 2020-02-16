fn main() {
    let _number = 3;

    if _number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let _number2 = 3;
    if _number2 != 0 {
        println!("number was something other than zero");
    }

    let _number6 = 6;
    if _number6 % 4 == 0 {
        println!("number is divisible by 4");
    } else if _number6 % 3 == 0 {
        println!("number is divisible by 3");
    } else if _number6 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let _condition = true;
    let _number_with_condition = if _condition { 5 } else { 6 };
    println!("The value of number is :{}", _number_with_condition);


    let mut _counter = 0;
    let _result = loop {
        _counter += 1;
        if _counter == 10 {
            break _counter * 2;
        }
    };
    println!("The value of number is: {}", _result);

    let mut _number3 = 3;
    while _number3 != 0 {
        println!("{}!", _number3);
        _number3 -= 1;
    }
    println!("LIFTOFF!");

    let _a = [10, 20, 30, 40, 50];
    let mut _index = 0;
    while _index < 5 {
        println!("the value is: {}", _a[_index]);
        _index += 1;
    }

    for element in _a.iter() {
        println!("the value is: {}", element);
    }

    for _number in (1..4).rev() {
        println!("{}", _number);
    }
    println!("LIFTOFF!!!");
}
