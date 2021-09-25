
use std::collections::HashMap;

fn main(){


    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];//vec! 宏 
    v.push(5);//必须是 mut 
    v.push(6);

    let third: &i32 = &v[2]; //如超范围造成 panic
    println!("The third element is {}", third);
    
    match v.get(2) { //返回一个 Option<&T>,如超范围返回None
        Some(third) => println!("The third element is {}", third), //自带Some,None
        None => println!("There is no third element."),
    }
  
    //v.push(6); //前面有 &v[2]，不能修改
    //println!("The first element is: {}", third);

    for i in &v { //遍历
        println!("{}", i);
    }
    //遍历改变
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; //解引用运算符（*）获取 i 中的值
    }
    //-------hashmap
    //所有的键必须是相同类型，值也必须都是相同类型。
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
 
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    //<_, _> 类型注解是必要的，因为可能 collect 为很多不同的数据结构
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效 
    // println!("{}={}", field_name，field_value);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); 
    scores.entry(String::from("Yellow")).or_insert(50);//没有键时才插入
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    
    let text = "hello world wonderful world"; 
    let mut map = HashMap::new(); 
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);//or_insert 返回这个键的值的一个可变引用
        *count += 1; //map的值0 + 1
    } 
    println!("{:?}", map); 
}