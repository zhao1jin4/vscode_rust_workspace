use std::thread;
use std::time::Duration;
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }//函数
    let add_one_v2 = |x: u32| -> u32 { x + 1 };//闭包
    let add_one_v3 = |x: u32|        { x +1 }; //新版本参数必须要有类型
    let add_one_v4 = |x: u32|          x +1 ;

    let example_closure = |x:u32| x;

    generate_workout2(
        simulated_user_specified_value,
        simulated_random_number
    );

} 


fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| { //闭包,参数可没有类型
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));//线程暂停
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }

    let mut c = Cacher::new(|a| a); 
    let v1 = c.value(1);
    let v2 = c.value(2); 
    println!("{}",v2==2);//false ,第一次使用 1 调用 c.value，Cacher 实例将 Some(1) 保存进 self.value。在这之后，无论传递什么值调用 value，它总是会返回 1。

    let x = 4; 
    let equal_to_x = |z| z == x;  //闭包比函数强的是可以仿问到外部变量x
    let y = 4;  
    println!("{}",equal_to_x(y));//true
    //FnOnce 闭包必须获取其所有权并在定义闭包时将其移动进闭包
    //FnMut 获取可变的借用值所以可以改变其环境
    //Fn 从其环境获取不可变的借用值 
 
    let x = vec![1, 2, 3]; 
    let equal_to_x = move |z| z == x; 
   // println!("can't use x here: {:?}", x);//移动的，这里不能再使用x 
    let y = vec![1, 2, 3]; 
    assert!(equal_to_x(y));
    //Fn 开始，而编译器会根据闭包体中的情况告诉你是否需要 FnMut 或 FnOnce。
}

//---闭的包泛型 
struct Cacher<T>
    where T: Fn(u32) -> u32 //所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个trait  
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}


fn generate_workout2(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

 