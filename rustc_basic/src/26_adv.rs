fn main(){
    type Kilometers = i32; //类似C的typedef,同义词（synonym）,用于定义长的类型，减少重复

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    //std::io 有这个类型别名声明：
    type MyResult<T> = std::result::Result<T, std::io::Error>;
    // 动态大小类型（dynamically sized types）DST,str 是一个 DST
    // let s1: str = "Hello there!";//不知道实际大小，报错，
    let s1:  &str = "Hello there!";//&str 则是 两个值,str的地址和其长度，在编译时可以知道的大小, Box<str> 或 Rc<str>, &dyn Trait 或 Box<dyn Trait>（Rc<dyn Trait> 也可以 
  

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    //函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce）
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())//map参数为闭包  
    .collect();
     
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)//map参数为函数指针 , ::
        .collect();

     enum Status {
        Value(u32),//元组
        Stop,
    }

    let list_of_statuses: Vec<Status> =
        (0u32..20)
        .map(Status::Value)//元组有map函数
        .collect();
}
fn generic<T>(t: T) { //泛型函数默认只能用于在编译时已知大小的类型
} 
//实际上被当作如下处理：
fn generic1<T: Sized>(t: T) {//是 Sized trait  在编译时就知道大小的类型实现。另外，Rust 隐式的为每一个泛型函数增加了 Sized bound
}


//?Sized ,T 可能是也可能不是 Sized 的”
fn generic2<T: ?Sized>(t: &T) {  
}

fn add_one(x: i32) -> i32 {
    x + 1
} 
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { //fn 被称为 函数指针（function pointer）, Fn 闭包 
    f(arg) + f(arg)
}

//返回闭包 放在 Box中 ，dyn约束类型
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}








// fn bar() -> ! { //! 返回 无类型，类似void ,报错??
   
// }

