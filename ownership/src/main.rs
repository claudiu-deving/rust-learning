use std::io;
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");
    move_drop_implementing();
    move_copy_implementing();
    ownership_test();
    reference_as_argument_test();
    mutable_references();
    get_first_word();
    get_nth_word_by_input();
}
fn move_drop_implementing(){
    let s1 = String::from("hello");
    //The contents will move from s1 to s2
    let s2 = s1;//At this point s1 is no longer considered valid, this is done so that when drop is called it won't be performed on the same reference

    //println!("{s1}, world!");  <- This won't work
    println!("{s2}, world! from s2");
    let s3 = s2.clone();
    println!("{s3}, world! from a copy of s2");
    println!("{s2}, world! from s2 again.");//This is permitted since we copied s2 when creating s3so the s3 is variable is pointing to another space than s2 (is it basically a new object)


}
fn move_copy_implementing(){
    let x =5;
    let y =x;
    //This is permitted because x can be copied since its size is know at compile time
    println!("value of x is {x}");
    println!("value of y is {y}");
}
fn ownership_test(){
     let s = String::from("New hello");
     takes_ownership(s);//From this point onward, s is no longer valid

     let x =5;

     makes_copy(x);
     println!("The x variable is still valid {x}");
 
}
fn takes_ownership(some_string:String){
    println!("{some_string}");
}

fn makes_copy(some_integer:i32){
    println!("Some copied integer {some_integer}");
}
fn reference_as_argument_test(){
    let s = String::from("This is a new string");
    let len = calculate_length(&s);
    println!("The string: {s} is {len} characters long.");
}
//Calculates the length of a given string
//Param: s -> string reference
//Returns: the length
fn calculate_length(s:&String)->usize{
    s.len()
}//Since the input string is passed only as reference,the function doesn't own it so it won't be dropped

fn mutable_references(){
    //   let s = String::from("Hello");
    //   change(&s);   //This won't compile, the reference is not mutable
    let mut s_mutable = String::from("Hello");
    let reference = &s_mutable;
    println!("{:p}",&reference);
    let r1 = &mut s_mutable;
    //I am trying to change the address of the r1 reference in the call stack but it seems that the
    //runtime is using some form of optimisation and is using always the same stack position for
    //the just_print frame
    println!("The address of r1 in main fn is {:p}",&r1);
    just_print(r1);
    let local = [0u8; 10000];
    println!("The address of the local array: {:p}",&local);
    let r2 = r1;
    just_print(r2);
}
fn change(some_string_to_change:&mut String){
    some_string_to_change.push_str(", world?");
}
fn just_print(non_mutable_reference:&String){
    let local = [0u8; 10000];
    println!("The address of the local array: {:p}",&local);

    println!("The address of r1 in just_print fn is {:p}",&non_mutable_reference);
}

//write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
fn get_nth_word(input_string:&str,word_number:i32)->(usize,usize){
   let bytes = input_string.as_bytes();
   let mut word_counter=0;
   let mut last_space_index=0;
   for (i,&byte) in bytes.iter().enumerate(){
       if byte == b' '{
           word_counter+=1;
           if word_counter ==word_number {
               return (last_space_index,i);
           }else{
               last_space_index =i;
           }
       }
   }
   return (0, input_string.len());
}
fn get_nth_word_by_input(){
   let mut input_string=String::new();
   println!("Enter the text to find the word for");
   io::stdin()
       .read_line(&mut input_string)
       .expect("Invalid input");
   input_string= (input_string.trim()).to_string();
   let mut word_number=String::new();
   println!("Enter the word number you want to extract");
   io::stdin()
       .read_line(&mut word_number)
       .expect("Invalid input");
   let word_number:i32 = word_number.trim().parse().expect("Please inser a number");
   let (first_index,last_index) = get_nth_word(&input_string,word_number);
   let word = &input_string[first_index..last_index];
   println!("The {}th number of {} is {}",word_number,input_string,word);
    
}
fn get_first_word(){
   let mut input_string=String::new();
   println!("Enter the text to find the first word for");
   io::stdin()
       .read_line(&mut input_string)
       .expect("Invalid input");
   input_string= (input_string.trim()).to_string();
   let (first_index,last_index) = get_nth_word(&input_string,1);
   let first_word = &input_string[first_index..last_index];
   println!("The first word of {} is '{}'",input_string,first_word);
}
