fn main(){

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);  //as_str() 切片
    println!("The longest string is {}", result);
    
    //--
    let string1 = String::from("long string is long"); 
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    //--
    /*
    let string1 = String::from("long string is long");
    let result; //生命周期比string2长,不行
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    //i 和 first_sentence 生命周期要相同

/*
    生命周期省略规则
    1.每一个是引用的参数都有它自己的生命周期参数
    2.如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
    3.如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法,那么所有输出生命周期参数被赋予 self 的生命周期
*/


    //所有的字符串字面值都拥有 'static 生命周期
    let s: &'static str = "I have a static lifetime.";//存在程序的二进制文件中

}

/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// &i32        // 引用
// &'a i32     // 带有显式生命周期的引用，   生命周期名字为a
// &'a mut i32 // 带有显式生命周期的可变引用 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {//这些a 生命周期必须存在得一样久
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
 当返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用
fn longest2<'a>(x: &str, y: &str) -> &'a str {//编译失败原因，返回值的生命周期与参数完全没有关联
    let result = String::from("really long string");
    result.as_str()
}
*/


struct ImportantExcerpt<'a> {
    part: &'a str,
}
//ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久



//impl 之后和类型名称之后的生命周期参数是必要的
impl<'a> ImportantExcerpt<'a> { //impl后加生命周期，同泛型使用
    fn level(&self) -> i32 {//因为第一条生命周期规则我们并不必须标注 self 引用的生命周期。
        3
    }
}


impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str { //第三条生命周期省略规则的例子：
        println!("Attention please: {}", announcement);
        self.part
    }
}


use std::fmt::Display;
//泛型和生命周期一起使用
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}