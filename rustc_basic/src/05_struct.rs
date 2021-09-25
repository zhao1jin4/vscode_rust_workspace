 
fn main() {
     
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
  
    //元组结构体 
    struct Point(i32, i32, i32); 
    let origin = Point(10,20,30);
    println!(".0={}",origin.0);


    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1); //{:?}是一行的，
    println!("rect1 is {:#?}", rect1); //{:#?}是多行的

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let sq = Rectangle::square(3);//不以self 作为参数
}

//增加注解来派生 Debug trait
#[derive(Debug)]  //可以被打印
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle { //结构体的方法impl
    fn area(&self) -> u32 {  //&self 也可为 &mut self,无参方法
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool { //有一个参数，要在&self后
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle { //可以有多个impl
    //不以self 作为参数被称为 关联函数,如String::from 
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    } 
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
} 
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
 
 