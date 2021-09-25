fn main() {
    let mut s = String::from("hello"); 
    s.push_str(", world!"); // push_str() 在字符串后追加字面值 
    println!("{}", s);  

    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}, world!", s1);//不可再使用s1,一个堆区只能被最后的使用
     
    let s3 = s2.clone();//复制
    println!("{}, world!", s3);
 

    let s = String::from("hello");  
    takes_ownership(s); // s 的值移动到函数里 
    //println!("error {} ", s);  //所以到这里s不再有效

    let x = 5;   
    makes_copy(x);// 但 i32 是 Copy 的，所以在后面可继续使用 x 


    let s1 = String::from("hello"); 
    let (s2, len) = calculate_length(s1); 
    println!("The length of '{}' is {}.", s2, len);
}
fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {  
    println!("{}", some_integer);
}  
 
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度 
    (s, length)
}