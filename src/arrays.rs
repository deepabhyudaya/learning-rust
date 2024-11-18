fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", array);
    println!("{}", array[0]);
    println!("{}", array.len());
    println!("{:?}", &array[1..4]);
}
