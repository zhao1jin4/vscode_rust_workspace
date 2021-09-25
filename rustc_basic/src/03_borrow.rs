fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);//引用，表示里面只可以读，可以读多次，(像数据库的读写锁，写锁只可有一个，读锁可多个)

    println!("The length of '{}' is {}.", s1, len);//这还继续使用s1 


    let mut s = String::from("hello"); 
    change(&mut s);//可变的引用，里面就可以变

    
    //let r1 = &mut s;//作用域内只能有一个可变引用
    //println!("{}, {}", r1, s);//s只可被可变借用一次，错误
   

    let mut s = String::from("hello"); 
    let r1 = &s; // 没问题 （读锁可多个）
    let r2 = &s; // 没问题 （读锁可多个）
    let r3 = &mut s; // 大问题
    println!("{}, {}, and {}", r1, r2, r3);
    //不能在拥有不可变引用的同时拥有可变引用（有读锁了，也不能有写锁）
 
} 

fn calculate_length(s: &String) -> usize {//这里不能修改值
     println!("s= {}", s);
   // s.push_str(", world!");//error 
    s.len()
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s//s会被回收的，不能返回 ， dangle悬荡
}
fn no_dangle() -> String {
    let s = String::from("hello"); 
    s
}
//要么 只能有一个可变引用，要么 只能有多个不可变引用。


