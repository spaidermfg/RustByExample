/// # 作用域
/// rall（resource acquistion is initiallization）资源获取即初始化
/// 任何函数在离开作用域时，都会被调用析构函数释放资源
/// rust的析构函数通过Drop trait实现，离开作用域时便会调用，无需为每个都实现
fn main() {
    println!("Hello, world!");

    create_box();

    let _x = DoDrop;
    println!("Made a DoDrop.");
}


fn create_box() {
    //在堆上分配内存
    let _box = Box::new(3i32);
}


struct DoDrop;

impl Drop for DoDrop {
    fn drop(&mut self) {
        println!("DoDrop is being dropped.")
    }  
}
