fn main() {
    let tuple = (23, "hi");
    let randTuple: (&str, i8) = ("Vikings", 4);

    println!("{}", randTuple.0);
}
