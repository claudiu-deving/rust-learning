pub fn get_fibonacci_number(number:u32)->f32{
    //based on the wikipedia page: https://en.wikipedia.org/wiki/Fibonacci_sequence#Binet's_formula

    let fi= (1.0+f32::sqrt(5.0))/2.0;
    let psi = (1.0- f32::sqrt(5.0))/2.0;
    (fi.powf(number as f32) - psi.powf(number as f32))/(f32::sqrt(5.0))
}
