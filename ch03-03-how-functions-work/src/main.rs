fn main() {
    let _x = 5;

    let _y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", _y);

    let _five = five();
    println!("The value of _five is: {}", _five);
}

fn five() -> i32 {
    5
}
