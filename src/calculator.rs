use std::io;

fn main() {
    println!("Enter the first number: ",);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Enter the second number: ",);
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let a: i32 = input.trim().parse().ok().expect("Enter numbers only");
    let b: i32 = input2.trim().parse().ok().expect("Enter numbers only");

    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a / b);
}
