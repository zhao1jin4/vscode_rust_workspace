
fn main(){
    use crate::front_of_house::hosting; //引入，默认是私有的，可使用 pub use
    use std::io::Result as IoResult; //别名
    use std::{cmp::Ordering, io as myio}; //多引用 
    use std::io::{self, Write}; //self表示引用std::io
    use std::collections::*; //所有 公有项引入作用域

    hosting::add_to_waitlist();
}
 
mod front_of_house {
    pub mod hosting {
    // Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。
        pub fn add_to_waitlist() {} //pub 关键字来创建公共项
    }
} 

pub fn eat_at_restaurant() {
    //绝对路径（absolute path）从 crate 根开始
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); //父模块中的
    }

    fn cook_order() {}
}
 





