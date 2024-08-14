use std::io;
mod fibonacci;
mod temperatur_conversion;

fn main() {
    loop{

        println!("What do you want to test?");
        println!("Press 1 for retrieving the nth fibonacci number");
        println!("Press 2 to convert from Celsius to Fahrenheit and vice-versa");
        println!("Write quit to exit.");
       let mut initial_input=String::new();
       io::stdin()
           .read_line(&mut initial_input)
           .expect("Invalid Input");
        if initial_input.trim() == "1" {
           fibonacci();
        }else if initial_input.trim()=="2" {
           temperatur_conversion::temperature_conversion();
        }else if initial_input.trim()=="quit" {
           break;
        }
    }
}

fn fibonacci(){
 let mut input=String::new();
    println!("Enter the nth fibonacci number");
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");
    let input:u32=input.trim().parse().expect("Please enter an positive integer");

    let result =fibonacci::get_fibonacci_number(input);
    println!("The {0}th fibonacci number is: {1}",input,result);

}
