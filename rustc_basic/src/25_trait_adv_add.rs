
fn main(){
  
   
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });  //运算符重载 + 

   let res = Millimeters(3) + Meters(1);
    println!("{}",res.0);


}
//Iterator源码
pub trait Iterator1 {
    type Item; //type ,这个 trait 的实现者会指定 Item 的具体类型，然而不管实现者指定何种类型, next 方法都会返回一个包含了此具体类型值的 Option

    fn next(&mut self) -> Option<Self::Item>; //大写的Self
}//当 trait 有泛型参数时，可以多次实现这个 trait，每次需改变泛型参数的具体类型,next 方法时，必须提供类型注解来表明希望使用 Iterator 的哪一个实现。


use std::ops::Add; //运算符重载 + 

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}



//Add源码
trait Add1<RHS=Self> {//泛型 默认类型
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

 
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters { //泛型 替代默认类型
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}