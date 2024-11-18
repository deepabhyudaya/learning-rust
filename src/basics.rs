use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // let secret = rand::thread_rng().gen_range(1, 101);
    // println!("secret {}", secret);

    // loop {
    //     println!("Please input your token",);
    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to Read Line");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You entered {}", guess);

    //     match guess.cmp(&secret) {
    //         Ordering::Less => println!("{}", "Too Small".red()),
    //         Ordering::Greater => println!("{}", "Large".red()),
    //         Ordering::Equal => {
    //             println!("{}", "Equal".green());
    //             break;
    //         }
    //     }
    // }
    println!("{}", add(12, 12));
}

fn add(a: u32, b: u32) -> i32 {
    (a + b) as i32
}

// fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
//     // Corner case, early return
//     if rhs == 0 {
//         return false;
//     }

//     // This is an expression, the `return` keyword is not necessary here
//     lhs % rhs == 0
// }

// fn main() {
//     let boolean: bool = true;
//     let character: char = 'a';

//     println!("{} {}", boolean, character);

//     println!("{:b},{:x},{:o}", 10, 10, 10);
//     println!("{ten:>ws$}", ten = 10, ws = 2);
//     println!("{ten:>0ws$}", ten = 10, ws = 2);

//     println!("sin 3.14 = {}", 3.14f64.sin());

//     println!("length of sin {}", "sin".len());

//     let (first, second) = "sin".split_at(0);
//     println!("{},{}", first, second);

//     let mut name = 10;
//     let name = 56;
//     println!("{}", name);
// }

// fn main() {
//     let randSring = "5677 6788";
//     // let mut chars = randSring.chars();
//     // let mut indChars = chars.next();
//     // loop {
//     //     match indChars {
//     //         Some(x) => println!("{}", x),
//     //         None => break,
//     //     }
//     //     indChars = chars.next();
//     // }

//     // let randString2 = "Mumbai is in MahaR\n Bombay is Mumbai";
//     // let mut iter_line = randString2.lines();
//     // let mut indSentence = iter_line.next();
//     // loop {
//     //     match indSentence {
//     //         Some(x) => println!("{}", x),
//     //         None => break,
//     //     };
//     //     indSentence = iter_line.next();
//     // }

//     let mut iter = randSring.split_whitespace();
//     let mut indWord = iter.next();
//     loop {
//         match indWord {
//             Some(x) => println!("{}", x),
//             None => break,
//         };
//         indWord = iter.next();
//     }
// }
