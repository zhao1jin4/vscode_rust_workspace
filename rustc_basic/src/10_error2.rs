
 use std::error::Error;
 use std::fs::File;
 
  //main 函数的一个有效的返回值是 ()
 fn main() -> Result<(), Box<dyn Error>> {
     let f = File::open("hello.txt")?;
 
     Ok(())
 }
 