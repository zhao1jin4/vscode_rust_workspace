 
use std::fs;
use std::env;

//结构体及内部成员，函数都要是pub的

#[derive(Debug)]  //可以被打印
pub  struct Config { 
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}
 
impl Config {
    
     pub   fn new(args: &[String]) -> Config {
        let query = args[1].clone();//避免使用 clone 来解决所有权问题
        let filename = args[2].clone();

        Config { query, filename,case_sensitive:false}
    }
     pub   fn new2(args: &[String]) -> Result<Config, &'static str> {//泛型第一个参数是成功的类型，第二个是失败的类型
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();//读环境变量,如果未设置就是有错误,返回true

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

pub  fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}


use std::error::Error;
//返回 unit 类型 (), trait 对象 Box<dyn Error>
pub fn run2(config: Config) -> Result<(), Box<dyn Error>> {//Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型
    let contents = fs::read_to_string(config.filename)?;   //?号，错误让调用者来处理它

    println!("With text:\n{}", contents);
    
   let results =  search(&config.query, &contents); 
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
     
    for line in results {
        println!("{}", line);
    }

    Ok(())
} 







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
    

 #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
} 
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new(); 
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // } 
    // results
    //更简单的方式,没有性能影响
     contents.lines()
        .filter(|line| line.contains(query))
        .collect()
    //   vec![]
} 
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
