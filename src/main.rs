use std::io;
use std::process;

// fn main() {
//     println!("Please enter your name: ");

//     let mut name = String::new();
//     io::stdin().read_line(&mut name);

//     println!("Hello, {}", name)
// }

fn main() {
    loop {
        println!("Please enter your first number");
        let a = read_user_input();

        println!("Please enter your second number");
        let b = read_user_input();

        let sum = sum(a, b);
        println!("{} + {} = {}", a, b, sum)
}
}

fn sum (a:u32, b:u32) -> u32 {
    a + b
}

fn read_user_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    // let a = input.trim().parse().unwrap();
    // let a = input.trim().parse().expect("This is not a valid number");
    let digit:u32;
    match input.trim().parse() {
        Ok(val) => digit = val,
        Err(_err) => {
            println!("This is not a valid number");
            process::exit(0);
        }
    }

    digit
}