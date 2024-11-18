fn main() {
    let mut vex = vec![1, 2, 3, 4, 5];
    println!("{}", vex[0]);

    for i in &vex {
        println!("{} ", i);
    }

    vex.push(23);
    vex.push(45);
    vex.pop();

    for i in &vex {
        println!("{}", i)
    }
}
