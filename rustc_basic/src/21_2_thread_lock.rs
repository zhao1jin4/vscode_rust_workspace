 
use std::thread;  
use std::rc::Rc;
use std::sync::{Mutex, Arc};
fn main()
{
    let m = Mutex::new(5);
     //Mutex<T> 是一个智能指针。更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。这个智能指针实现了 Deref 来指向其内部数据；其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁
     //也是会死锁的
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

 
    //let counter = Mutex::new(0);
    //let counter = Rc::new(Mutex::new(0));//Rc不能用于多线程
    let counter = Arc::new(Mutex::new(0));
    //Arc<T> 类似 Rc<T> 安全的用于并发环境的类型。字母 “a” 代表 原子性（atomic）
    //可以使用 Mutex<T> 来改变 Arc<T> 中的内容。
    let mut handles = vec![];

    for _ in 0..10 {
        //let counter = Rc::clone(&counter);//如不克隆会被移动
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    } 
    println!("Result: {}", *counter.lock().unwrap()); 
}
//Send 标记 trait 表明类型的所有权可以在线程间传递。几乎所有的 Rust 类型都是Send 的，不过有一些例外，包括 Rc<T>
//Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用,即如果 &T是 Send 的话， T 就是 Sync 的，
//基本类型是 Sync 的，Rc<T> 也不是 Sync 的,RefCell<T>和 Cell<T> 系列类型不是 Sync 的,Mutex<T> 是 Sync 的
//通常并不需要手动实现 Send 和 Sync trait,手动实现 Send 和 Sync 是不安全的