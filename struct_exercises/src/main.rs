mod area_calculator; 
fn main() {
    let immutable_user = User{
        user_name : String::from("Someone"),
        email : String::from("some@one.com"),
        active : false,
        sign_in_count :5
    };
    let mut mutable_user = User{
        user_name : String::from("Xulescu"),
        email : String::from("Someone@some.com"),
        active : true,
        sign_in_count:55
    };
    mutable_user.user_name = String::from("Popesculescu");
    let new_user= build_user(String::from("Some_one"),String::from("some@one.com"));
    println!("The user with usernam {} has the email {}",new_user.user_name, new_user.email);
    let user_2= User{
        email:String::from("different@email.com"),
              ..new_user
    };
    //At this point the new_user User struct is no longer valid since we move the user_name from
    //the new_user to user_2 using the above update syntax.
    //If we were to also modify the username when updating then the new_user would remain valid
    //since isActive and sign_in_count are value types (Copy trait)
    println!("The second user with username {} has the email {}",user_2.user_name, user_2.email);
    let color = Color(0,0,0);
    let origin = Point(0,0,0);
    area_calculator::calculate_area();
}

struct User{
    active:bool,
    user_name:String,
    email:String,
    sign_in_count:u64
}
fn build_user(user_name:String,email:String)->User{
    User{
        active:true,
        sign_in_count:1,
        user_name,
        email}
}
//Tuple structs

struct Point(i32,i32,i32);
struct Color(i32,i32,i32);
