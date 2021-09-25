use std::fs::File;
use std::io::ErrorKind;
fn main(){

    /*当出现 panic 时，程序默认会开始 展开（unwinding）,清理它遇到的每一个函数的数据,另一种选择是直接 终止（abort），这会不清理数据就退出程序
    Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止

    如果你想要在release模式中 panic 时直接终止：
    [profile.release]
    panic = 'abort'
    */

    // panic!("crash and burn");

    let v = vec![1, 2, 3]; 
    //v[99]; //引发panic


    //提示环境变量 RUST_BACKTRACE=1 ，backtrace 是一个执行到目前位置所有被调用的函数的列表
    //为了获取带有这些信息的 backtrace，必须启用 debug 标识。会默认启用(除非用 --release 参数运行 cargo build 或 cargo run 时 debug 标识)

    
 
    let f = File::open("hello.txt");
/*

    //panic时 vscode debug 控制台中文乱码???  ,rustc命令行运行正常
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };

*/

/*
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => 
                match File::create("hello.txt") { 
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
*/
    //如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
    //let f = File::open("hello.txt").unwrap();

    use std::net::IpAddr; 
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    
    //let f = File::open("hello.txt").expect("Failed --- to open hello.txt");//追加自己的错误消息
    
    
    read_username_from_file();
    read_username_from_file2();
    read_username_from_file3();
   let res:Result<String, io::Error> =  read_username_from_file4();
    println!("res={:?}",res);
 
}


use std::io;
use std::io::Read; 
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new(); 
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 传播错误  提供了? 问号运算符来使其更易于处理。如果出现了错误，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;//?必须放在返回Result 或 Option 的函数中
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new(); 
    File::open("hello.txt")?.read_to_string(&mut s)?; //?也可链式调用 
    Ok(s)
}

use std::fs;  
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}