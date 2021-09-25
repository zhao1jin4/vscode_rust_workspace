 

fn main() { 
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
} 

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    //方法泛型
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//使用泛型时没有运行时开销,只是编译生成特定类型的代码
