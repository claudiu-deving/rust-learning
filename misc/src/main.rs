fn main() {
    let result;
    {
    let string  = String::from("the longer string");        //'a -> 
        let string2 = String::from("the shorter string");   //'b ->
        result = longest(string.as_str(),string2.as_str());                  //'b <-
    }
    //This will panic, the string2 reference is dropped 
    println!("The longer string is {}",result);

}                                                           //'a <-
//This won't compile

//fn longest(x: &str,y: &str)->&str{
//    if x.len() > y.len(){
//        x
//    }else{
//       y
//    }
//}
fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len() > y.len(){
        x
    }else{
       y
    }
}
