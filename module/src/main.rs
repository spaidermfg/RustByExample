/// # 模块
/// ## 可见性
/// 模块中可以拥有很多项，函数、结构体、枚举、模块等，使用关键字pub来改变默认可见性
/// 模块中拥有私有可见性和公有可见性
fn main() {
    println!("Hello, world!");


    
}

mod my_mod {
    use std::thread::panicking;

    fn private_fn() {
        println!("called `my_mod::private_fn()`");
    }

    //使用pub关键字改变默认可变性
    pub fn pub_fn() {
        println!("called `my_mod::pub_fn()`");
    }

    //在同一模块中，项可以访问任何其他项
    pub fn indirect_access() {
        println!("called `my_mod::indirect_access()`, that> ");
        private_fn();
    }

    // 模块也可以嵌套
    pub mod nested {
       pub fn pub_fn() {
            println!("called `my_mod::nested::pub_fn()`");
       } 
    

        fn private_fn() {
            println!("called `my_mod::nested::private_fn()`");
        }
        
        // 使用pub(in path) 语法定义的函数只能在给定的路径中可见
        // path必须是父模块或祖先模块
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called `my_mod::nested::public_function_in_my_mod()`, that> ");
            public_function_in_my_nested()
        }

        // 使用pub(self) 语法定义的函数只能在当前模块中可见
        pub(self) fn public_function_in_my_nested() {
            println!("called `my_mod::nested::public_function_in_my_nested()`");
        }

        // 使用pub(super) 语法定义的函数只在父模块中可见
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }


    pub fn call_public_function_in_my_mod() {
        println!("called `my_mod::nested::call_public_function_in_my_mod()`, that > ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }
}




