fn main() {
    println!("Hello, world!");

 
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 ; //常量只能被设置为常量表达式,值不可有变量
    println!("{}",THREE_HOURS_IN_SECONDS);  
  
     let x = 5; 
     let x = x + 1;//两次let一个变量，是被覆盖的 
     {
         let x = x * 2;//只是局部覆盖
         println!("The value of x in the inner scope is: {}", x);
     }
 
     println!("The value of x is: {}", x);//6

/*
i8	u8
i16	u16
i32	u32
i64	u64
i128	u128
isize	usize
*/
    let c0 = 'z';
    println!("{}",c0);
   let c:u8 =b'A'; //b 二进制节字符,只能u8,u=unsign
   let c1:u16 =0b1111_0000;  //二进制
   let c2:i32 = 98_222;//十进制
   let c3:i64 =  0o77;//八进制
   let c4:i128 = 0xff;//十六进制
   let c5:isize=64;//64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的

   let x = 2.0; // 默认是 f64 
   let y: f32 = 3.0; // f32 
   let f: bool = false;  
   let tup: (i32, f64, u8) = (500, 6.4, 1);//元组,约束类型

   let tup = (500, 6.4, 1); 
   let (x, y, z) = tup;
   println!("{},{},{}",tup.0,tup.1,tup.2);  //0开始像数组

   let a = [1, 2, 3, 4, 5]; 
   let a: [i32; 5] = [1, 2, 3, 4, 5];//i32 是每个元素的类型。分号之后，数字 5 表明该数组包含五个元素。
   let a = [3; 5];//包含 5 个元素，这些元素的值最初都将被设置为 3，同let a = [3, 3, 3, 3, 3];
    let val=a[0];

    let y = {
        let x = 3;
        x + 1  //没有分号，最后一行是返回值
    };
    plus_one(5);


    let  number = 6; 
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }



    let condition = true;
    let mut number = if condition {// 在 let 语句中使用 if ，mut可变的
        5
    } else {
        6
    };

    //loop 无限循环，可用break;
    let mut count = 0; 
    'counting_up: loop { //标签
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    let mut counter = 0; 
    let result = loop { 
        counter += 1;

        if counter == 10 {
            break counter * 2;//loop 带返回值 
        }
    };

    while number != 0 {
        println!("{}!", number); 
        number = number - 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0; 
    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }


    let a = [10, 20, 30, 40, 50]; 
    for element in a.iter() { //数组的iter(), for in 像JS
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() { //..范围 1-3， 反向
        println!("{}!", number);
    }

}
 
fn plus_one(x: i32) -> i32 { //返回用 ->
    x + 1   //不加;号
  //return   x + 1 ;  
}


