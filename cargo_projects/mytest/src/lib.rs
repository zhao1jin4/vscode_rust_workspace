
//文件名不叫lib.rs,单元测试不能跑？？
//编译提示 either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present

fn  main(){
    println!("hello")
}

//cargo new mytest --lib 生成的项目有lib.rs代码 , cargo test 来运行
//不希望测试并行运行 cargo test -- --test-threads=1
 
#[cfg(test)]// cargo test 时才编译和运行测试代码,cargo build则忽略
//单元测试位于与源码相同的文件中，所以你需要使用 #[cfg(test)] 来指定他们不应该被包含进编译结果中。
mod tests {
    #[test] //cargo test 会执行有这个标志的
    fn it_works() {  //cargo test it_works 只测试这个用例 ，也可是名字中的一部分做匹配
        println!("2+2!=???"); //cargo test -- --nocapture 才能显示这个值 
        assert_eq!(2 + 2, 4);
    }

     #[ignore] //忽略, cargo test -- --ignored  只运行有忽略标志的
     #[test]
     fn another() {  //cargo test an ,名字中有an的
        panic!("Make this test fail"); //使用panic! 做失败
    }

    
    use super::*;  //导入父模块中的所有内容.就可以认Rectangle
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));//为true
    }

    
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }
 
    #[test]
    #[ignore] //忽略
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );//如失败显示自已定义的错误消息
    }

    #[should_panic]//测试有异常的情况
    #[test]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));//可以测试私有函数，因已经 use
    }
}
 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
//没pub 的函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}


