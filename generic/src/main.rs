use std::fmt::Debug;

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
