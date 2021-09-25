fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { //新的match(if let)
        if age > 30 {
            println!("Using purple as the background color");//print
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }


    
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    //只要模式匹配就一直进行 while 循环
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }



    let point = (3, 5);
    print_coordinates(&point);
 
     
   // 不可反驳的( irrefutable )   let x = 5;  x 可以匹配任何值所以不可能会失败
   if let x = 5 { //会有  irrefutable 的警告
        println!("{}", x);
    };

    let x = Some(5);
    let y = 10; 
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),//此处y是内部的临时变量，不同于外部的y=10
        _ => println!("Default case, x = {:?}", x),//其它模式
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);


    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // | 或者的意思
        3 => println!("three"),
        _ => println!("anything"),
    }
    match x {
        1..=5 => println!("one through five"),  //..=是范围  1-5
        _ => println!("something else"),
    }
    let x = 'c'; 
    match x {
        'a'..='j' => println!("early ASCII letter"), 
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }


    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; //创建了变量 a 和 b 来匹配结构体 p 中的 x 和 y 字段
    assert_eq!(0, a);
    assert_eq!(7, b);
  
    let Point { x, y } = p; //同x:x,y:y
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 }; 
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x), //x任何值 
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    } 

    
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });//元组嵌套
   
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10); 
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { //_ match中项的忽略
            println!("Can't overwrite an existing customized value");
        }
        _ => { //其它
            setting_value = new_setting_value;
        }
    } 
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => { // _ match中项的忽略
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }


    let _x = 5; //下划线开头的未使用变量,去掉未使用变量警告,仍然会绑定值，它可能会获取值的所有权
    
    let s = Some(String::from("Hello!"));
    if let Some(_) = s { //s 没有被移动进 _
        println!("found a string");
    }
    println!("{:?}", s);


    let numbers = (2, 4, 8, 16, 32); 
    match numbers {
        (first, .., last) => {  //.. 忽略剩余值,不能两边都用，歧义，如(.., second, ..)
            println!("Some numbers: {}, {}", first, last);
        },
    }



    let num = Some(4); 
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), //if条件也要同时成立
        Some(x) => println!("{}", x),
        None => (),
    }
    

    let x = 4;
    let y = false; 
    match x {
        4 | 5 | 6 if y => println!("yes"), //等同于 (4 | 5 | 6) if y => 
        _ => println!("no"),
    }


    enum Message {
        Hello { id: i32 },
    }
    
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => { //@ 设置变量保存值 
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }

}

fn print_coordinates(&(x, y): &(i32, i32)) { //元组 做函数参数 指定类型
    println!("Current location: ({}, {})", x, y);
}
struct Point {
    x: i32,
    y: i32,
}
fn foo(_: i32, y: i32) { // _ 函数忽略
    println!("This code only uses the y parameter: {}", y);
}
 

