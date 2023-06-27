use std::future::PollFn;

#[allow(unreachable_code)]
/// # 流程控制
/// ifelse后条件无需小括号
/// loop 表示一个无限循环
///     嵌套循环需要使用标签来声明内外层循环
///     将操作值放在break之后，即可被返回
/// while条件循环，当条件满足时循环
/// match匹配，类似switch，比对每一个分支
///     在match中间接的访问一个变量，需要使用@进行重新绑定来在分支中再次使用
/// 指针解引用用*，解构用&，ref，ref mut, 可以使用ref创建引用  
fn main() {
    println!("Hello, world!");

    ifelse();

    loops();

    whiles();

    fors();

    matchs();

    match_deconstruction();

    point_deconstruction();

    struct_deconstruction();

    if_let();
}


fn ifelse() {
    let n = 5;
    if n > 0 {
        println!("{} is positive", n);
    } else if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is zero", n);
    }

    let h = 
        if n > 10 {
        10 * n
    }else {
        n / 2
    };

    println!("{} -> {}", n , h);
}

fn loops() {
    let mut num = 0;

    println!("Let's get loop life.");

    loop {
        num += 1;

        if num == 5 {
            println!("five minutes!");

            continue;
        }
        
        println!("num: {}", num);

        if num == 10 {
            println!("Ok, fucking life, that's enough.");

            break;
        }
    }


    //嵌套循环
    //使用标签声明循环
    'outer: loop {
        println!("Entered the outer loop.");

        'inner: loop {
            println!("Entered the inner loop.");
            
            //just中断内部循环
            //break;

            //中断外部循环
            break 'outer;
        }

        println!("This point will never be reached.");
    }

    println!("Exited the outer loop.");


    //loop返回值
    let mut counter = 10;
    let result = loop {
        counter += 1;

        if counter == 67 {
            break 5 * 2 * 67; 
        }
    };

    println!("resutl == 670 ? {:?} {}", result == 670, result);
}


fn whiles() {
    let mut n = 1;
    while n < 1000 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

fn fors() {
    //遍历区间
    //for n in 1..=100 
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }


    //遍历迭代器
    //需要先将集合转化为一个迭代器
    //iter：每次迭代中借用集合中的一个元素，集合本身不会被改变，循环之后仍然可以使用。
    //into_iter: 会消耗集合， 循环之后集合无法再使用
    //iter_mut: 可变的借用集合中的每个元素，允许集合被就地修改
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
        &"Ferris" => println!("There is a rustacean among us!"),
        _ => println!("hello {}", name),
        }
    }

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }
}


fn matchs() {

    let num = 67;
    println!("Tell me about {}", num);
    match num {
        1 => println!("One"),
        2 | 3 | 4 | 5 | 11 => println!("This is a prime"),
        12..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }


    let flag = true;
    let binary = match flag {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", flag, binary);


    //卫语句 guard
    let pair = (6, 7);
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("x: {} = y: {}", x, y),
        (x, y) if x + y == 13 => println!("x: {} + y: {} = {}", x, y, x + y),
        (x, _) if x % 2 == 0 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    //使用绑定
    println!("Tell me what type of person you are.");
    match age() {
        0  => println!("I haven't celebrated my first  birthday yet."),
        n @ 1..=12 => {
            println!("I'm  a child of age {}",n)
        },
        n @ 13..=19 =>println!("I'm a teen of age {}", n),
        n => println!("I'm a old person of age {}", n),
    }

}

fn age() -> u32 {
    17
}

//解构
fn match_deconstruction() {
    //解构元组
    let triple = (2,3,4);
    println!("Tell me about {:?}", triple);

    match triple {
        (2, x, y) => println!("First is `2`, x = {:?}, y = {:?}", x, y),
        (1, ..) => println!("First is `1`, and the rest doesn't matter，..表示忽略元组的其余部分"),
        _ => println!("表示不将值绑定到变量")
        
    }

    //解构枚举
    use Color::*;
    let color = RGB(2,4, 6 );
    match color {
        Red => println!("The color is red."),
        Blue => println!("The color is blue."),
        Green => println!("The color is Green"),
        RGB(x,y ,z ) => println!("Red: {}, Blue: {}, Green: {}", x, y,z),
        HSL(h,s ,l ) => println!("h: {}, s: {}, l: {}", h,s,l),
        CMY(c,m ,y ) => println!("c: {}, m: {}, y: {}", c,m,y),
    }
}

//指针解构和解引用
fn point_deconstruction() {
    //创建一个i32类型的引用
    let reference = &67;

    match reference {
        //使用&匹配引用
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    //解引用
    match *reference {
        val => println!("Got a value via defeferencing: {:?}", val),
    }

    //无引用
    let _not_a_reference = 4;
    //使用ref创建引用
    let ref _is_a_reference = 8;

    let value = 9;
    let mut mut_value = 6;

    match value {
        //使用ref创建引用
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        //ref获得引用
        ref mut m => {
            //*解引用后改变变量值
            *m += 10;
            println!("Got a mut value add 10 is : {:?}", m);
        },
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSL(u32,u32,u32),
    CMY(u32,u32,u32),
}

fn struct_deconstruction() {
    struct Foo{ x: (u32, u32), y: u32}

    //解构结构体
    let foo = Foo { x: (1, 2), y : 3};
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {}, y = {}", a, b ,y );

    //顺序不重要
    let Foo {y: i, x : j} = foo;
    println!("i: {:?}, j: {:?}", i, j);

    //省略某些字段
    let Foo {y, ..} = foo;
    println!("y = {}", y);   
}

//一些使用match不方便的情况下使用if let
fn if_let() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("number is {:?}", i);
    }

    if let Some(i) = letter {
        println!("letter is {:?}", i);        
    } else {
        println!("Don't match a letter. Let's go with a letter.")
    }

    let is_like_letters: bool = false;

    if let Some(i) = emoticon {
        println!("emotion is {:?}", i);
    } else if is_like_letters {
        println!("is like a letter.");
    } else {
        println!("Let's go with a emoticon.");
    }
    

    //匹配枚举值
    let bar = Tony::Bar;
    let baz = Tony::Baz;
    let qux = Tony::Qux(99);

    if let Tony::Bar = bar {
        println!("bar is tony bar");
    }

    if let Tony::Bar = baz {
        println!("baz is tony bar");
    }

    if let Tony::Qux(i) = qux {
        println!("qux is {}", i);
    }
}

enum Tony {
    Bar,
    Baz,
    Qux(u32), 
}
