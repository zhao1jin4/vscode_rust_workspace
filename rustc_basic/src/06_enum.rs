 
fn main() {
    let four = IpAddrKind::V4; //两个冒号
    
    let home = IpAddr::V4(127, 0, 0, 1); 
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);//不需要 Option:: 
    let some_string = Some("a string");   

    let x=value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}",x);

    let some_u8_value = 0u8;
    match some_u8_value { //像scala
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
       // _ => (), //其它通配
       _ => println!("others"),
    }

    //---
    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("others1"),
    }
    //效果同上 if let 只能用于enum,是match的简写
    if let Some(3) = some_u8_value {
        println!("three");
    }else {
        println!("others2");
    }


    let coin = &Coin::Quarter(UsState::Alabama); 
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    if let Coin::Quarter(state) = coin {//if let 效果同上，state可以接受值
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}
 
enum IpAddrKind {
    V4,
    V6,
}

 
enum IpAddr {
    //枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据
    V4(u8, u8, u8, u8),
    V6(String),//关联了 String 值
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, //一个匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}



enum Option<T> { //同Java,C++
    Some(T),
    None,
}
 

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska 
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), //某一项，可以是另一个enum
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin { //match
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
       // Coin::Quarter => 25,
       Coin::Quarter(state) => { //state是接受到的enum值
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

 