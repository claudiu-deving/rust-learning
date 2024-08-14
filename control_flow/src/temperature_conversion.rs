

pub fn temperature_convertion(){

    loop{
        println!("Type 1 for converting celcius into fahrenheit");
        println!("Type 2 for converting fahrenheit into celcius");
        println!("Type quit to exit");
        let mut input_type = String::new();
        io::stdin()
            .read_line(&mut input_type)
            .expect("Failed");        
        match input_type.trim() {
            "1"=>{
               println!("Enter the temperature in celcius to be converted");
               let mut temperature=String::new();
               io::stdin()
                    .read_line(&mut temperature)
                    .expect("Failed");
               let temperature:f32= temperature.trim().parse().expect("Please input a valid temperature");

               println!("{0} degrees celcius is {1} degrees fahrenheit.",temperature,celcius_to_fahrenheit(temperature));
               },
            "2"=> {
                println!("Enter the temperature in fahrenheit to be converted");
                let mut temperature=String::new();
                io::stdin()
                     .read_line(&mut temperature)
                     .expect("Failed");
                let temperature:f32 = temperature.trim().parse().expect("Please input a valid temperature");

                println!("{0} degrees fahrenheit is {1} degrees celcius ",temperature,fahrenheit_to_celcius(temperature));
            },
            "quit"=>{
                break;
            },
            _=> println!("Enter a valid option")
        }
    }
}

fn fahrenheit_to_celcius(temperature_in_fahrenheit:f32)->f32{
    (temperature_in_fahrenheit-32.0)/1.8
}

fn celcius_to_fahrenheit(temperature_in_celcius:f32)->f32{
    (temperature_in_celcius*1.8)+32.0
}
