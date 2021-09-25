
enum List {
    Cons(i32, Rc<List>),//Rc<T> 的类型。其名称为 引用计数（reference counting）, 只能用于单线程场景
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
      println!("count after creating a = {}", Rc::strong_count(&a));//引用数
    let b = Cons(3, Rc::clone(&a));//克隆 Rc<T> 会增加引用计数
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}


 