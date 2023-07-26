pub mod loan;

/// 访问数据但并不取得所有权
/// 对象通过引用&T传递，而非值T传递
/// 对象在其作用域中被借用时，后续不能进行销毁操作
pub fn borrow_test() {
    println!("'borrow test");
}

// 取得一个box并销毁
fn destory_box(box32: Box<i32>) {
    println!("Destorying box that contains: {}", box32);
}


// 借用一个i32类型
fn borrow_i32(bor32: &i32) {
    println!("This int32 is : {}", bor32);
}
