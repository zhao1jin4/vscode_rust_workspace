 
//&[i32] ,寻找 slice 中最大的 i32
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
 


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);



    let p = Point { x: 5, y: 10 }; 
    println!("p.x = {}", p.x());

}
struct Point<T> {
    x: T,
    y: T,
}

//impl<T>  ,Point 的尖括号中的类型是泛型而不是具体类型。
impl<T> Point<T> {  
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}



 