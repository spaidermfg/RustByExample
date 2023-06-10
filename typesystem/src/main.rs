#![allow(overflowing_literals)] //不显示类型转换产生的溢出警告
///类型系统
///类型转化
///使用as关键字进行显式类型转化 casting
///别名
///使用type语句为已有的类型取别名，命名方式必须是驼峰命名法
fn main() {
    println!("Hello, world!");

    casting();
}

//类型转换
fn casting() {
    let decimal = 65.4321_f32;

    //显示类型转化
    let integer = decimal as u8;
    let characer = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, characer);

    //将任何类型转化为无符号类型T时， 会不断加或减上 std::T::Max + 1, 直到在T的范围内

    //1000在u16范围内
    println!("1000 as a u16 is: {}", 1000 as u16);
    
    //1000 - 256 - 256 -256 = 232
    println!("1000 as a u8 is: {}", 1000 as u8);
    
    //-2 + 256 = 254
    println!("-1 as a u8 is: {}", (-2i8) as u8);
    
    //取模
    println!("1000 mod 256 is : {}", 1000 % 256);

    //当转化为有符号类型时，MSB（二进制的最高位）为1， 则值为负
    println!("128 as a i16 is : {}", 128 as i16);

    //取8位的二进制补码 128: 1000 0000
    println!("128 as a i8 is: {}", 128 as i8);

    println!("1000 as a u8 is: {}", 1000 as u8);

    // 232的二进制补码是-24 232-256  232: 1110 1000
    println!("232 as a i8 is: {}", 232 as i8);


    let friendly = "helloworld";

    //siez_of_val 在mem模块，std crate中，返回一个变量所占用的字节数
    println!("size of helloworld in bytes: {}", std::mem::size_of_val(&friendly));


    //别名
    type NanoSecond = u64;
    type Inch = u64;

    //未使用驼峰命名法，编译器会有警告
    #[allow(non_camel_case_types)]  //屏蔽警告
    type u64_t = u64;

    let nanoseconds: NanoSecond = 5 as u64_t;

    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches)
}

