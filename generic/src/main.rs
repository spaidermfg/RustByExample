use std::fmt::{Debug, Display};

/// # 泛型
/// 泛化类型和函数，减少重复代码
/// 泛型类型参数一般用<T>表示
fn main() {
    
    println!("Hello, world!");

    use_generic();
}


//泛型结构体
struct SGen<T>(T);

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

//接受一个泛型参数
fn generic<T>(_s : SGen<T>) {}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}


//实现泛型
impl <T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

//使用泛型函数
fn use_generic() {
    //显式的指定类型参数
    generic::<i32>(SGen(8));

    //隐式的指定类型参数
    generic(SGen(9));
    
    let x = Val {val: 3.4};
    let y = GenVal { gen_val: 67i32};
    println!("{} {}", x.value(), y.value());
    
    //泛型trait 
    let empty = Empty;
    let null = Null;

    //释放empty和null
    empty.double_drop(null);

    //约束
    let rectangle = Rectangle { length: 7.9, height: 6.3};
    print_debug(&rectangle);

    println!("{}", area(&rectangle));


    //多重约束
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![4, 5,6];

    compare_prints(&string);

    compare_types(&array, &vec);

    //关联项
    let a = 3;
    let b = 8;

    let container = Container(a, b);
    println!("{} to {}: {}", &a, &b, container.contains(&a, &b));
    println!("First num: {}", container.first());
    println!("Last num: {}", container.last());

    println!("the difference is: {}", difference(&container));
    
}


struct Empty;
struct Null;

//泛型trait
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

//为T和U实现trait
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

//约束
//规定类型必须实现哪些功能
trait HasArea {
    fn area(&self) -> f64;
}

//为Rectangle类型实现HasArea trait 
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

//为类型实现Debug trait 
#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

//调用者必须实现HasArea trait 
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

//调用者必须实现Debug 
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

//关联约束，使用+号, 类型之间使用逗号分隔
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

//关联项
struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, a: &i32, b: &i32) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
       self.1 
    }
}

//使用where子句
fn difference<A, B, C>(container: &C) -> i32 where 
    C: Contains<A, B> {
    container.last() - container.first()
}
