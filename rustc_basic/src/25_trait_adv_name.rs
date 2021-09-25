
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
fn main() {
    let person = Human;
    person.fly();//用 Human 中的同名方法
    Pilot::fly(&person);
    Wizard::fly(&person); 

    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}",Animal::baby_name());  //不知道使用哪个同名函数
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); //类型转换
 
   let p= Point{x:3,y:4};
   p.outline_print();

}
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}



use std::fmt;

trait OutlinePrint: fmt::Display { //: 类似继承,或者  类型定义
    fn outline_print(&self) {
        let output = self.to_string();//因有fmt::Display可以to_string()
        let len = output.len();
        println!("{}", "*".repeat(len + 4)); //repeat() 函数
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {} //只实现了一部分，还要实现 fmt::Display

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { //函数格式 
        write!(f, "({}, {})", self.x, self.y)//write!
    }
}


//孤儿规则（orphan rule） 实现 trait时要求一定要在(crate )本地作用域中(当前包下)。 绕开这个限制的方法是使用 newtype 模式
//缺点是，因为 Wrapper 是一个新类型 ,即代理，Deref另一个方案
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))//self.0来得到
    }
}
