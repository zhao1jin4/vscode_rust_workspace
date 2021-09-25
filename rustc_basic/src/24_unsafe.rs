/*
不安全的超能力有： 
    解引用裸指针
    调用不安全的函数或方法
    访问或修改可变静态变量
    实现不安全 trait
    访问 union 的字段 

 */
fn main(){
    let mut num = 5;  
    let r1 = &num as *const i32; //不可变的 裸指针
    let r2 = &mut num as *mut i32; //可变的 裸指针  
    //裸指针不受控于rust的安全检查，使用同C的指针
    unsafe {//必须放在unsafe中仿问内存
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6]; 
    let r = &mut v[..]; 
    //let (a, b) = r.split_at_mut(3);  //split_at_mut 函数
    let (a, b) =split_at_mut( r,3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

 
    use std::slice; 
    let address = 0x01234 ;//不安全的
    let r = address as *mut i32; 
    let slice: &[i32] = unsafe {  //无需将 split_at_mut 函数的结果标记为 unsafe
        slice::from_raw_parts_mut(r, 10000)
    };
  
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    
    add_to_count(3); 
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}


// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//      (&mut slice[..mid],   //借用了同一个 slice 两次，Rust安全检查不通过
//       &mut slice[mid..])  
// }


fn split_at_mut(tmpSlice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = tmpSlice.len();
    let ptr = tmpSlice.as_mut_ptr(); //as_mut_ptr() 访问 slice 的裸指针

    assert!(mid <= len);

    use std::slice;
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),  //from_raw_parts_mut 要裸指针和一个长度来创建一个 slice
         slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

extern "C" { //调用其它语言
    fn abs(input: i32) -> i32; //C标准库的abs函数
}


#[no_mangle] //不修改函数名，mangle(损坏)
pub extern "C" fn call_from_c() {//从其它语言调用 Rust 函数
    println!("Just called a Rust function from C!");
}
//extern 的使用无需 unsafe。


static mut COUNTER: u32 = 0; //静态变量只能储存拥有 'static 生命周期的引用,访问和修改可变静态变量都是 不安全 的。

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}



unsafe trait Foo { 
}

unsafe impl Foo for i32 {  //实现也必须标记为 unsafe
}


//union 和 struct 类似，但是在一个实例中同时只能使用一个声明的字段。
//联合体主要用于和 C 代码中的联合体交互。访问联合体的字段是不安全的，因为 Rust 无法保证当前存储在联合体实例中数据的类型
