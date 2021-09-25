use std::io;
fn main() { 
    //vscode 不能从标准输入 ,只能用命令行才行
//Settings->Run Code configuration -> 复选 Run In Terminal 组下的唯一的一个Wheter to run code in Integerated Terminal -> ctrl+s保存 测试不行？？？
    println!("Please input your guess."); 
    let mut guess = String::new(); 
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");  
    println!("You guessed: {}", guess);

}
