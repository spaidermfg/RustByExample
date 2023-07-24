/// # 作用域
/// rall（resource acquistion is initiallization）资源获取即初始化
/// 任何函数在离开作用域时，都会被调用析构函数释放资源
/// rust的析构函数通过Drop trait实现，离开作用域时便会调用，无需为每个都实现
/// ## 所有权
/// 所有资源只能拥有一个所有者，通过值传递所有权会发生转移，通过引用传递所有权不会转移
fn main() {
    println!("Hello, world!");

    create_box();

    let _x = DoDrop;
    println!("Made a DoDrop.");

    owner_ship();
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


fn owner_ship () {
    //在栈上分配内存
    let x = 7i32;

    //copy
    let y = x;
    println!("x is {}, y is {}", x, y);


    //在堆上分配内存
    let a = Box::new(6i32);
    println!("a contains: {}", a);

    //所有权转移到b
    let b = a;

    //error
    //println!("a contains: {}", a);
    
    //释放b
    destroy_box(b);  

}

fn destroy_box(b: Box<i32>) {
    println!("Destroying a box that contains {}", b);

    //b被释放
}


