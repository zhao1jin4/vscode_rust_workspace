fn main(){
     
    let data = "initial contents"; 
    let s = data.to_string(); //从&str转换为String,只可是UTF-8 

    let mut s = String::from("lo"); 
    s.push_str("bar");
    s.push('l');


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    //println!("{}", s1);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe"); 
    let s = format!("{}-{}-{}", s1, s2, s3); //format!宏 ，并且不会获取任何参数的所有权。
    println!("{}", s1);
 
    

    let hello = "abcdefg"; 
    let s = &hello[0..4];
    println!("{}", s);

    for c in "abcdefg".chars() {
        println!("{}", c);
    }
    for b in "abcdefg".bytes() {
        println!("{}", b);//是ascii
    }
}
 