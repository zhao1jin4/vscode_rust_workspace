fn main(){

    let v1 = vec![1, 2, 3]; 
    let v1_iter = v1.iter(); 
    for val in v1_iter {
        println!("Got: {}", val);
    }

     let v1 = vec![1, 2, 3]; 
    let mut v1_iter = v1.iter(); 
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();//聚合计算
    assert_eq!(total, 6);

    let v1: Vec<i32> = vec![1, 2, 3]; 
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); //像java ,scala 
    assert_eq!(v2, vec![2, 3, 4]);


    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    for val in in_my_size {
        println!("size: {}", val.size);
    }

     let mut counter = Counter::new(); 
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
   
//    let sum: u32 = Counter::new().zip(Counter::new().skip(1)) //没有实现skip方法
//                                  .map(|(a, b)| a * b)
//                                  .filter(|x| x % 3 == 0)
//                                  .sum();
//     assert_eq!(18, sum);
//同scala
/*
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
*/

} 
pub trait Iterator {//源码结构
    type Item; 
    fn next(&mut self) -> Option<Self::Item>; 
    // 此处省略了方法的默认实现
} 
struct Shoe {
    size: u32,
    style: String,
} 
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)//filter
        .collect()
}
struct Counter {
    count: u32,
} 
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> { //自定义迭代器
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}