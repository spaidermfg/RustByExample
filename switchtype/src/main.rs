/// # 类型转换
/// rust常使用From和Into来进行类型转换  
/// 如果类型实现了From trait，那么将自动实现into trait，into就是再转换回去
/// 使用into时需要指定要转换的类型
///
fn main() {
    println!("Hello, world!");

    from_into();
}

fn from_into() {
    // 将str转换为string类型
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("str: {}, string: {}", my_str, my_string);

    //使用自定义类型转换机制
    let num = Number::from(67);
    println!("num: {:?}", num);

    //使用Into
    let i = 5;
    let k: Number = i.into();
    println!("into: {:?}", k);
}
//为自定义类型实现转换机制
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value: value }
    }
}
