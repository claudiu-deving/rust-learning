pub fn calculate_area(){
    let square = Shape::Square(5);
    let area = square.area();
    let circumference = square.circumference();
    println!("The area of square with size {} is: {:?}",5,area);
    println!("The circumference of square with size {} is: {}",5,circumference);

    let rectangle = Shape::Rectangle(Rectangle{width:20,heigth:10});
    let area = rectangle.area();
    let circumference =rectangle.circumference();
    println!("The area of rectangle with size {:?} is: {}",rectangle,area);
    println!("The circumference of square with size {:?} is: {}",rectangle,circumference);

    let circle = Shape::Circle(10);
    let area = circle.area();
    let circumference =circle.circumference();
    println!("The area of circle with radius {:?} is: {}",circle,area);
    println!("The circumference of circle with radius {:?} is: {}",circle,circumference);

}
#[derive(Debug)]
struct Rectangle{
    width:u32,
    heigth:u32
}

enum Shape{
    Square(u32),
    Rectangle(Rectangle),
    Circle(u32)
}

impl Shape{
    fn area(&self)->u32{
        match &self{
            Shape::Square(square) => square.pow(2),
            Shape::Rectangle(rectangle) => rectangle.width*rectangle.heigth,
            Shape::Circle(radius)=>(3.14*(radius.pow(2) as f32)) as u32
        }
    }
    fn circumference(&self)->u32{
        match &self{
            Shape::Square(square)=>square*4,
            Shape::Rectangle(rectangle)=>rectangle.width*2+rectangle.heigth*2,
            Shape::Circle(radius)=>((*radius as f32)*2.0*3.14) as u32
        }
    }
}
