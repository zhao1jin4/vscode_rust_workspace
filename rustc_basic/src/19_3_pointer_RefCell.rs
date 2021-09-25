//不同于 Rc<T>，RefCell<T> 代表其数据的唯一的所有权 ,RefCell<T> 只能用于单线程场景
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

//----test
 use std::cell::RefCell;
  struct MockMessenger {
        //sent_messages: Vec<String>,
         sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            //MockMessenger { sent_messages: vec![] }
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //self.sent_messages.push(String::from(message)); //self在定义时为 不可变的
            self.sent_messages.borrow_mut().push(String::from(message));//borrow_mut()不可变，修改为 可变
         

            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();//不能创建两个可变借用，编译时没有错误，link时有
            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));

 
        }
    }
fn main(){
     let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    //assert_eq!(mock_messenger.sent_messages.len(), 1);
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);//取出用borrow()
}