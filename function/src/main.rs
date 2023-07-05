/// # 1. 函数方法
/// 方法是依附于对象的函数，在impl块中定义
/// &self是self: &self的语法糖，其中Self是调用者的类型。   
/// self是self: Self的语法糖
/// &mut self 是self: &mut Self的语法糖
/// 静态方法使用::访问， 实例方法使用.访问
/// ## 闭包closure
/// 能够捕获周围作用域中变量的函数 |val| val + x
/// 输入和返回类型都可以自动推导，但输入参数名必须指明.
/// |val| 替代 （val） 
///
fn main() {
    println!("Hello, world!");

    use_method();

    closure();
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    //静态方法
    //一般用作构造器
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    //实例函数
    //使用语法糖&self
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    //可变参数
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;

        self.p2.x += x;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    //消耗调用者的资源
    //离开作用域后资源销毁
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {}) success.", first, second);
    }
}

fn use_method() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(6.0, 7.0),
    };

    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(6.0, 7.0),
    };

    square.translate(8.0, 9.0);
    println!(
        "Rectangle translate p1: {:?} p2: {:?}",
        square.p1.x, square.p2.y
    );

    let pair = Pair(Box::new(8), Box::new(8));
    pair.destroy();
}


fn closure() {
    let closure_annotated = |i: i32| -> i32 {i + 1};
    let closure_inferred = | i | {i + 1};

    let i = 6;
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
}
