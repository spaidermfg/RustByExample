use core::fmt;
use std::mem;

///原生类型
///标量类型    
///有符号i8，i32，无符号u16，u64， 浮点数f32， f64，char字符类型， bool类型，单元类型(), 空元组     
///复合类型     
///数组[1,2,3], 元组(1, true, 2.4)      
///变量都可以显示的给出类型说明 let a: i32 = 67
///数字还可以通过后缀或默认方式来声明类型。整型默认i32， 浮点类型默认f64。let a = 5i32;
fn main() {
    println!("Hello, world!");

    basic();

    literal();

    tuple();

    array();
}

fn basic() {
    //变量给出类型说明
    let flag: bool = true;

    //后缀说明
    let a = 5i32;

    //可变变量, 值可变但类型不可以变
    let mut mutable = 12;
    println!("{}", mutable);
    
    mutable = 21;
    println!("{}", mutable);
    //使用遮蔽覆盖前面的变量，可以修改类型
    let mutable = true;

    println!("{} {} {}", flag, a, mutable); 
}


//字面量
fn literal() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);


    //布尔运算
    println!("true and false is {}", true && false);

    println!("true or flase is {}", true && false);

    println!("not true is {}", !true);

    //位运算
    println!("0011 and 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 or 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 xor 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);


    //使用下划线改善数字可读性
    println!("One million is written as {}", 1_000_000u32);
}


///元组     
///元组可以包含各种类型的值     
///元组可以拥有任意多个值       
///元组通过下标来访问具体的值       
///创建单元素元组需要额外的逗号 (1,)        
///元组可以被解构   
fn tuple() {
    
    let kind_tuple = (true, 4i32, 2,4f32, 'u', 8u16, 8.9f64);


    //通过下标访问元组中的元素
    println!("tuple first value: {}", kind_tuple.0); 
    println!("tuple last value: {}", kind_tuple.5);

    //元组充当元组的元素
    let tuple_of_tuple = (kind_tuple, (), (1u8, 6.7f32), false);

    //打印元组
    println!("tuple of tuple: {:?}", tuple_of_tuple);

    let pair = (1, true);
    println!("the pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));


    //解构元组
    let color = ("red", true, 254);
    let (red, blue, green) = color;
    println!("{} {} {}", red, blue, green);


    let matrix  = Matrix(1.1, 1.2, 2.1 ,2.2);
    println!("Matrix:\n{}",  matrix);

    println!("the matrix transpose is:\n{}", transpose(matrix));
}

//元组作为函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    //使用let将元组的成员绑定到变量上
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

//为结构体实现fmt::Display trait 
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)        
    }
}


///数组
///数组是一组拥有相同类型的对象的集合
///数组使用中括号[]创建，类型标记为[T; length]
///数组在编译时大小会被确定
///数组使用下标来访问值
///切片
///切片在编译时大小不确定
///切片是一个双字对象
fn array() {

    //定长数组
    let xs: [i32; 4] = [1,2,3,4];


    //100个0
    let ys: [i32; 100] = [0;100];

    println!("first element of the array: {}", xs[0]);
    println!("last element of the array:{} {}",&ys.len() - 1,  ys[&ys.len() - 1]);

    //数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));


    //数组可以被自动借用为slice 
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    //切片可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..10]); 

    //数组越界会导致panic 
    println!("{}", xs[7]);
    
}


//该函数借用一个切片
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
