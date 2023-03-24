// 定义一个可以计算面积的 trait
trait Area {
    fn area(&self) -> f64;
}

// 定义一个圆形结构体
#[derive(Debug)] 
struct Circle {
    radius: f64,
}

// 为圆形实现 Area trait
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义一个三角形结构体
#[derive(Debug)] 
struct Triangle {
    base: f64,
    height: f64,
}

// 为三角形实现 Area trait
impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 定义一个正方形结构体
#[derive(Debug)] 
struct Square {
    side: f64,
}

// 为正方形实现 Area trait
impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 定义一个打印面积的函数，它接收一个实现了 Area trait 的类型作为参数
fn print_area<T: Area + std::fmt::Debug>(shape: &T) {
    println!("The area of the {:?} is {}", shape, shape.area());
}

pub fn print_test() {
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle { base: 4.0, height: 5.0 };
    let square = Square { side: 2.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}
