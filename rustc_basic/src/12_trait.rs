 
use std::fmt::Display;

//scala也有trait （特点，特征）,类型Java的 接口

pub trait Summary {
    // fn summarize(&self) -> String;
     fn summarize(&self) -> String { //可以有默认实现
         String::from("(Read more...)")
     }
 }
 
 pub struct NewsArticle {
     pub headline: String,
     pub location: String,
     pub author: String,
     pub content: String,
 }
 
 impl Summary for NewsArticle {
     fn summarize(&self) -> String {
         format!("{}, by {} ({})", self.headline, self.author, self.location)
     }
 }
  
 
 
 pub struct Tweet {
     pub username: String,
     pub content: String,
     pub reply: bool,
     pub retweet: bool,
 }
 
 impl Summary for Tweet {
     fn summarize(&self) -> String {
         format!("{}: {}", self.username, self.content)
     }
 }
 
fn main() { 

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    notify(tweet);


    let char_list = vec!['y', 'm', 'a', 'q']; 
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let number_list = vec![34, 50, 25, 100, 65]; 
    let result = largest(&number_list);
    println!("The largest number is {}", result);



    //----
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    //let p:Pair<Tweet> =Pair::new(tweet,tweet2);//没有实现Display + PartialOrd不能调用cmp_display
    let p:Pair<String> =Pair::new(String::from("def"),String::from("abc"));
    p.cmp_display();

}
 

pub fn notify(item: impl Summary) {//参数也要加 impl
    println!("Breaking news! {}", item.summarize());
}
pub fn notify2<T: Summary>(item: T) {//泛型 类型 为 trait
    println!("Breaking news! {}", item.summarize());
}
pub fn notify3(item: impl Summary + Display) { //+ 多个 trait,Display导入的

}

/* 
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
}
//简化写法，使用 where 从句 
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
            U: Clone + Debug
{
}
*/
fn returns_summarizable() -> impl Summary { //返回也要加impl
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}


//自带的 PartialOrd,Copy 两个trait ,std::cmp::PartialOrd 可以实现类型的比较功能
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//fn largest<T: PartialOrd >(list: &[T]) -> T {
    let mut largest = list[0];//i32 和 char 这样的类型是已知大小的并可以储存在栈上，所以他们实现了 Copy

    for &item in list.iter() {
        if item > largest { //就可以比较了
            largest = item;
        }
    }

    largest
}
//如函数返回值从 T 改为 &T 并改变函数体使其能够返回一个引用，我们将不需要  Copy
 

 

//结构体
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
//两个实现的泛型T是不同，调用这个方法要x和y实现了两个trait
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y { //PartialOrd
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}



 
