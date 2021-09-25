//宏能够接受不同数量的参数,宏可以在编译器翻译代码前展开,调用宏 之前 必须定义它,函数则可以在任何地方定义和调用。
//缺点是宏定义要比函数定义更复杂

//查看vec! 宏定义来
#[macro_export] //说明宏应该是可用的
macro_rules! myvec { //使用 macro_rules!(过时的方式) 定一个宏名为vec
    ( $( $x:expr ),* ) => { //和match 表达式的结构类似，$x:expr匹配任意表达式，并将该表达式名为$x。* 说明该模式匹配零个或更多个
        //当以 myvec![1, 2, 3]; 调用宏时，$x 模式与三个表达式 1、2 和 3 进行了三次匹配。
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*//匹配次数对应前面的*,示例3次
            temp_vec
        }
    };
}
fn main(){
    let my=myvec![1, 2, 3]; 
    println!("{:?}",my);

   // 第二种形式的宏被称为 过程宏（procedural macros）
   //有三种类型的过程宏（自定义派生（derive），类属性和类函数）,见cargo项目
}