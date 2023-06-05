///  # 1. 格式化输出     
/// format!将格式化文本写到字符串   
/// print!,将文本输出到控制台   
/// fmt::Display需要通过手动实现
/// write!会将格式化后的字符串写入缓冲区f中
fn main() {

    println!("Hello, world!");

    print!("{} world", "hello");

    //使用位置参数
    println!("{0}, this is {1}. {1} this is {0}", "mark", "bob");

    //使用命名参数
    println!("{subject} {verb} {object}", object="the lazy dog", subject="math", verb="go to school");

    //在:后指定特殊的格式
    println!("{} of {:b} people know binary, the other half don't", 6, 7);

    //指定宽度对齐文本
    println!("{number:>width$}", number=1, width=6);

    //补齐数据
    println!("{number:>0width$}", number=1, width=6);


    #[derive(Debug)]
    struct Structrue(i32);
    println!("This struct `{:?}` won't print...", Structrue(67));

    debug();

    display();

    list();

    format();
}

///调试
fn debug() {

    //derive 属性会自动创建所需的实现，使这个struct可以使用fmt::Debug打印
    #[derive(Debug)]
    struct Structrue(i32);

    #[derive(Debug)]
    struct Deep(Structrue);


    println!("Now {:?} will print!", Structrue(67));
    println!("Now {:?} will print!", Deep(Structrue(233)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Mark";
    let age = 17;
    let mark = Person{ name, age};

    //{:#?}美化打印
    println!("{:#?}", mark);
}

//导入fmt模块
use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);


//为类型实现fmt::Display trait 
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    } 
}

#[derive(Debug)]
struct Point{
    x: f64,
    y: f64,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn display() {

    let minmax = MinMax(6, 7);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);


    let big = MinMax(200, 233);
    let small = MinMax(400, 788);
    println!("The big range is {big} and the small is {small}", small=small, big = big);
    let point = Point{
        x: 6.7,
        y: 5.7,
    };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}


struct List(Vec<i32>);

//为具有多个元素的类型实现fmt::Display
impl fmt::Display for List {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //使用元组的下标获取值， 并创建一个vec的引用
        let vec = &self.0;
        write!(f, "[")?;

        //迭代vec，并记录迭代次数
        for (count, v) in vec.iter().enumerate() {
            //对每一个元素加上逗号，第一个元素除外
            if count != 0 {
                write!(f, ", {}: ",count)?;
            }
            write!(f, "{}: {}",count, v)?;
        }

        write!(f, "]")
    }
}

fn list() {
   let v = List(vec![1,2,3,4,5,6,7,8,9,0]);
    println!("Display: {}", v);
}


#[derive(Debug)]
struct Color { 
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) {:#04x}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

fn format() {
    
    for color in [
        Color {red: 128, green: 255, blue:90},
        Color {red: 0, green: 3, blue: 254},
        Color {red: 0, green: 0, blue: 0},
    ].iter() {
        println!("{}", *color);
    }
}
