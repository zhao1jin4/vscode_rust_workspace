use std::env;
use std::fs;
use std::process;
fn main() {
     //要传参数的方式运行 (myproject中类似功能)
     
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);// 下标0 是命令的全路径
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // let query = &args[1];
    // let filename = &args[2];
    
    let (query, filename) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", filename); 

    let config = parse_config2(&args);
    let config = Config::new(&args); 
 
    let config = Config::new2(&args).unwrap_or_else(|err| { //如有错误会把Err的消息给闭包||中有err
        println!("Problem parsing arguments: {}", err);
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

   //  run(config);

    if let Err(e) = run2(config) {
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

    Config { query, filename ,case_sensitive:false}
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}


use std::error::Error;
//返回 unit 类型 (), trait 对象 Box<dyn Error>
fn run2(config: Config) -> Result<(), Box<dyn Error>> {//Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型
    let contents = fs::read_to_string(config.filename)?;   //?号，错误让调用者来处理它

    println!("With text:\n{}", contents);

    Ok(())//()
}


#[derive(Debug)]  //可以被打印
struct Config {
    query: String,
    filename: String,
    pub case_sensitive: bool,
}
 
impl Config {
    
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();//避免使用 clone 来解决所有权问题
        let filename = args[2].clone();

        Config { query, filename ,case_sensitive:false}
    }
    fn new2(args: &[String]) -> Result<Config, &'static str> {//泛型第一个参数是成功的类型，第二个是失败的类型
        if args.len() < 3 {
            //return Err("not enough arguments");
            eprintln!("Problem parsing arguments");
            process::exit(1);
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();//读环境变量
        println!("CASE_INSENSITIVE = {}\n", case_sensitive);

          
        Ok(Config { query, filename,case_sensitive })
    }
     //std::env::Args 实现了 Iterator 
    pub fn new3(mut args: std::env::Args) -> Result<Config, &'static str> {
         args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}