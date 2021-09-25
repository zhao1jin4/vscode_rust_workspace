fn main(){
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5 
    s.clear(); // 这清空了字符串，使其等于 "" 
    // word 在此处的值仍然是 5，

 
    let s = String::from("hello world"); 
    let hello = &s[0..5];//或者用&s[..5];
    let world = &s[6..11];//或者用&s[6..];
    let slice = &s[..];
    
    let a = [1, 2, 3, 4, 5]; 
    let slice = &a[1..3];
    //这个 slice 的类型是 &[i32]


    let s = "Hello, world!"; //这里 s 的类型是 &str
    //指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。


    let mut s = String::from("hello world"); 
    let word = first_word2(&s); 
    //s.clear(); // 错误! 
    println!("the first word is: {}", word);


 

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word2(&my_string_literal[..]);
    println!("1the first word is: {}", word); 
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word2(my_string_literal);
    println!("2the first word is: {}", word);

}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
//fn first_word2(s: &String) -> &str {
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
