use std::env;
use std::fs;
use std::process;
use myproject::Config; //对应于Cargo.toml 中 [package] 下的 name = "myproject"
  
fn main() {
    //要传参数的方式运行,类似rustc_basc里的io.rs文件的内容，引用lib.rs
  

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);// 下标0 是命令的全路径
    
//    if args.len() < 3 {
//        panic!("not enough arguments");
//        process::exit(1);
//    }
    // let query = &args[1];
    // let filename = &args[2];
    
    let (query, filename) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", filename); 

    let config = parse_config2(&args);
    let config = Config::new(&args); 
 
    let config = Config::new2(&args).unwrap_or_else(|err| { //如有错误会把Err的消息给闭包||中有err
        eprintln!("Problem parsing arguments: {}", err);//eprintln 标准错误
        process::exit(1);//导入的
    });
   
     //env::args 函数返回一个迭代器！
    let config = Config::new3(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("config {:?}", config); ///{:#?}是多行的

  
    // let contents = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file"); 
    // println!("With text:\n{}", contents);

    // run(config);

    if let Err(e) =myproject::run2(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
fn parse_config2(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename,case_sensitive:false }
}
