use std::fmt::{Debug};
mod bor;
/// # 作用域
/// rall（resource acquistion is initiallization）资源获取即初始化
/// 任何函数在离开作用域时，都会被调用析构函数释放资源
/// rust的析构函数通过Drop trait实现，离开作用域时便会调用，无需为每个都实现
/// ## 所有权
/// 所有资源只能拥有一个所有者，通过值传递所有权会发生转移，通过引用传递所有权不会转移
/// 当所有权发生转移时，数据的可变形可能发生改变
/// 部分移动
/// 复合变量解构后，可以进行部分变量移动，其他部分保留，移动后不可以整体使用父级变量
fn main() {
    println!("Hello, world!");

    create_box();

    let _x = DoDrop;
    println!("Made a DoDrop.");

    owner_ship();
    
    //借用
    bor::borrow_test();
    bor::loan::borrow_demo();
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

    //当所有权发生转移时，数据的可变形可能发生改变
    let nomut_box = Box::new(8i32);

    println!("{} is not mut box", nomut_box);

    //not change
    //*nomut_box = 7;
    
    let mut mut_box = nomut_box;

    println!("{} is mut box", mut_box);

    *mut_box = 7;

    println!("mut box change after: {}", mut_box);

    //部分移动
    let person = Person {
        name: String::from("mark"),
        age: 21,
    };
    
    println!("person's {:?}", person);

    //name被移走，age被引用
    let Person { name,ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    //error, person被部分借用，不能完整使用
    //println!("person's {:?}", person);
    
    println!("The person's age from person struct is {}", person.age);
}

fn destroy_box(b: Box<i32>) {
    println!("Destroying a box that contains {}", b);

    //b被释放
}

struct Person {
    name: String,
    age: i32,
}


impl Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is the {} years old", self.name, self.age)
    }
}
