use std::thread;
use std::time::Duration;

fn main() { 
    //新线程,调用 thread::spawn 函数并传递一个闭包
    let handle = thread::spawn(|| { 
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //主线程退出，子线程也会退出（未执行完）
    //join 阻塞当前线程直到 handle 所代表的线程结束。
    handle.join().unwrap();
 

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {  //move 闭包获取其使用的值的所有权，如是只读引用子线程不知道v能活多久
        println!("Here's a vector: {:?}", v); 
    });
    handle.join().unwrap();

    //--通道 像go 语言
    //mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();//返回 第一个元素是发送端，而第二个元素是接收端
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //println!("val is {}", val);//val被移动了，这不能使用
    });
   let received = rx.recv().unwrap();//recv 会阻塞主线程执行直到从通道中接收一个值,try_recv 不会阻塞
    println!("Got: {}", received);
 

    //-- 
    let (tx, rx) = mpsc::channel();  
    let tx1 = tx.clone(); //多个生产者
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }); 
    for received in rx { //接收方式，也是阻塞的
        println!("Got: {}", received);
    }

}