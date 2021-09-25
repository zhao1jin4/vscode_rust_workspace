
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration; 
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
   sender: mpsc::Sender<Message>,
}


type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool { 
  pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

     pub fn execute<F>(&self, f: F)
        where  F: FnOnce() + Send + 'static //thread::spawn 的签名 ,Send 来将闭包从一个线程转移到另一个线程
        //FnOnce只会执行闭包一次， ()代表一个没有参数也没有返回值的闭包
        //'static 是因为并不知道线程会执行多久
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
        
    }
}

struct Worker {
    id: usize,
     thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  
       fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
} 

impl Drop for ThreadPool {
   fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();//监听地址 127.0.0.1:7878,bind 函数返回 Result<T, E>
    // 80 端口需要管理员权限（非管理员用户只能监听大于 1024 的端口)

    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) { //循环最多头 2 次
        let stream = stream.unwrap();
        
        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
        
    }
}
fn handle_connection(mut stream: TcpStream) {//读取了多于我们请求的数据并保存它们以备下一次请求数据。因此它需要是 mut 
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));//函数名的 “lossy”(有损耗的) 部分来源于当其遇到无效的 UTF-8 序列时的行为
    
    let get = b"GET / HTTP/1.1\r\n"; 
    if buffer.starts_with(get) {

    }else{

    }
     //--响应区
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}