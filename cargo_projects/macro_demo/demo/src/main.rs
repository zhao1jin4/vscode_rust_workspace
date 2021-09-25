/*
use hello_macro::HelloMacro;
struct Pancakes;
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");//Pancakes名字想要动态的，Rust 没有反射的能力
    }
}
下面的 #[derive(HelloMacro)]实现了上面的功能
*/

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes; 

fn main() {
    Pancakes::hello_macro(); //Hello, Macro! My name is Pancakes
}
 
 

//这里可以成功运行 cargo run
