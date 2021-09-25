fn main() {


    let text: Option<String> = Some("Hello, world!".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());//map方法是Option的
    println!("still can print text: {:?}", text);
    println!("still can print text: {:?}", text_length);

   //----
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len()); //maybe_some_string 被移动了
    assert_eq!(maybe_some_len, Some(13));
    //println!("maybe_some_string: {:?}", maybe_some_string);//这不能使用maybe_some_string
   //----
   let text_length: Option<usize> =  Some("Hello, world!".to_string()).map(|s| s.len());//加as_ref() 报错，因没有变量
   println!("still can print text: {:?}", text_length);
   let text_length: Option<usize> =  Some(String::from("Hello, World!")).map(|s| s.len());//error
   println!("still can print text: {:?}", text_length);
    //----
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
 
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { //Option的take()方法
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self) //Option的as_ref()方法, 需要 Option 中值的引用而不是获取其所有权
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>; 
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}