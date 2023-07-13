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
/// ## 闭包可以作为参数
/// Fn表示捕获方式为通过引用的&T的闭包，FnMut表示捕获方式为通过可变引用&mut T的闭包
/// FnOnce表示捕获方式为通过值为T的闭包
/// 闭包作为参数时，要求闭包必须时泛型的
/// 函数也可以作为函数参数，只要满足该闭包的trait约束即可
/// ## 函数作为返回值
/// rust目前只支持非泛型的返回值，只有使用impl trait才能返回一个闭包，并且返回值必须要声明闭包的有效特征
/// ## 高阶函数High\er Order function
/// 输入一个或多个函数，产生一个更有用的函数的函数
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

use std::mem;
fn closure() {
    let closure_annotated = |i: i32| -> i32 {i + 1};
    let closure_inferred = | i | {i + 1};

    let i = 6;
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));


    //通过引用&T捕获变量
    let color = String::from("green");
    let print = || println!("color: {}", color);

    print();


    //通过可变引用&mut T来捕获变量
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    inc();
    inc();

    let movable = Box::new(4);
    let consume = || {
        println!("movabble: {:?}", movable);
        mem::drop(movable);
    };

    consume();


    /* 将闭包作为参数 */
    let greeting = "hello";
    //不可复制类型
    //从借用的数据创建有所有权的数据
    let mut farewall = "goodbye".to_owned();

    let dairy = || {
        //Fn 
        println!("I said {}", greeting);

        //FnMut
        farewall.push_str("!!!!!");
        println!("Then I screamed {}.", farewall);
        println!("Now I can sleep. zzzzzzzzzzzzzz");

        //FnOnce
        mem::drop(farewall);
    };

    apply(dairy);

    let double = |x| x * 3;

    println!("double is {}", apply_to_3(double));


    /* 函数作为参数 */
    let doller = || println!("I'm they boss.");
    call_me(doller);
    call_me(function);

    /* 函数作为返回值 */
    let fn_pain = create_fn();
    let mut fnmut_pain = create_fnmut();
    let fnonce_pain = create_fnonce();

    fn_pain();
    fnmut_pain();
    fnonce_pain();


    //Iterator::any
    literator_any();

    //Iterator::find
    liteator_find();


    //高阶函数
    higher_order();
}


//将闭包作为参数并调用
fn apply<F>(f: F) where 
    F: FnOnce() {
    f();
}

//将闭包作为参数并返回一个整型
fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {

    f(3)
}

fn call_me<F: Fn()>(f: F) {
    f()
}

fn function() {
    println!("I'm your boss.");
}


fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {

    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}


fn literator_any() {
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    //对vec iter() 解构出&i32
    println!("in vec1: {}", vec1.iter().any(|&x| x == 2));
    //对vec into_iter()解构出i32
    println!("in vec2: {}", vec2.into_iter().any(|x| x == 2));
    
    let array1 = [1,2,3];
    let array2 = [4,5,6];

    println!("in array1: {}", array1.iter().any(|&x| x == 4));
    println!("in array2: {}", array2.into_iter().any(|x| x == 4));
}

// 接收一个迭代器，返回option类型
fn liteator_find() {
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];
    
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    //迭代器自身元素是&i32类型，将迭代器元素的引用传递给闭包，就是&&i32类型
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1,2,3];
    let array2 = [4,5,6];

    println!("Find 4 in array1: {:?}", array1.iter().find(|&&x| x == 4));
    println!("Find 4 in array2: {:?}", array2.into_iter().find(|&x| x == 4));
}


//记录奇数
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}


fn higher_order() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;


    //命令式编程
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n* n;

        if n_squared >= upper {
            break;
        } else if is_odd(n) {
            //统计奇数
            acc += n_squared;
        }
    }

    println!("imperative style: {}", acc);

    //函数式写法
    let sum_of_squared_odd_numbers: u32 = 
        (0..).map(|n| n * n)     //取平方
        .take_while(|&n| n < upper)  //取小于上限的
            .filter(|&n| is_odd(n))  //取奇数
            .fold(0, |sum, i| sum + i);   // 累加

    println!("functional style: {}", sum_of_squared_odd_numbers);
}
