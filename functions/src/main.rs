fn main() {
    another_function(five(), 6);
}

fn five() -> i32 {
    return 5;
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
