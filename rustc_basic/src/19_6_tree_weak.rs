/*
树，  leaf.parent 将会指向 branch 而 branch.children 会包含 leaf 的指针，这会形成引用循环，解决方法用Weak
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
*/
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
     //如果父节点被丢弃了，其子节点也应该被丢弃
     //然而子节点不应该拥有其父节点：如果丢弃子节点，其父节点应该依然存在
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    /*
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
*/

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); //调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);//Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),//weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
      
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}