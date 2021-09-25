
use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5); //Box<T> 在堆上储存数据
    println!("b = {}", b);
 
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil)))))); //像clojure，lisp这种括号,看着晕，可能是rust作者离开mozilla的原因


    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);//解引用 

    let y = Box::new(x); //Box和&使用一样，用*
    assert_eq!(5, *y);
    
 
    let x = 5;
    let y = MyBox::new(x); //自定义智能指针 实现 Deref 
    assert_eq!(5, x);
    assert_eq!(5, *y);//Rust 底层运行 *(y.deref()) 
    
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    //希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 std::mem::drop。
    std::mem::drop(c); //不能调用c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
} 
enum List {
    Cons(i32, Box<List>),//如不用Box，就无限循环计算所占空间，报  recursive without indirection
    Nil,
}


struct MyBox<T>(T); 

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref; 
impl<T> Deref for MyBox<T> { //实现 Deref 
    type Target = T;

    fn deref(&self) -> &T { //通过 * 运算符访问的值的引用
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
 
//Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。
//Rust 也会将可变引用强转为不可变引用。但是反之是 不可能


struct CustomSmartPointer {
    data: String,
} 
impl Drop for CustomSmartPointer {//值离开作用域时应该执行的代码的方式是实现 Drop
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}