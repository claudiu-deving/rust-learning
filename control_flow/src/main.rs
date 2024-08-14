use std::io;
mod fibonacci;

fn main() {
    let mut input=String::new();
    println!("Enter the nth fibonacci number");
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");
    let input:u32=input.trim().parse().expect("Please enter an positive integer");

    let result =fibonacci::get_fibonacci_number(input);
    println!("The {0}th fibonacci number is: {1}",input,result);
}

