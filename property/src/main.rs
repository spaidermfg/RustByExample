/// # 属性
/// 作用： 
/// * 条件编译代码
/// * 禁用lint
/// * 设置crate名称、版本和类型
/// * 启用编译器特性
/// * 标记函数为单元测试
fn main() {
    println!("Hello, world!");

    
    in_linux();

    println!("Are you sure: ");
    if cfg!(target_os = "linux") {
        println!("yes, It's definitely linux.");
    } else {
        println!("No, It's definitely not linux.")
    }
}


#[cfg(target_os = "linux")]
fn in_linux() {
    println!("This function compiler just in linux platform.");
}


#[cfg(not(target_os = "linux"))]
fn in_linux() {
    println!("You are not running linux.")
}
