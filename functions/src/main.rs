fn main() {
    another_function(5);

    print_label_measurement(5, 'h');

    let x = five();
    println!("the value of x is: {x}");

    let y = plus_one(x);
    println!("the value of y is: {y}");
}

fn another_function(x: i8) {
    println!("the value of x is: {x}");
}

fn print_label_measurement(x: i8, y: char) {
    println!("the value of x is: {x} and the value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}