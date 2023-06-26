#[allow(unreachable_code)]
/// # 流程控制
/// ifelse后条件无需小括号
/// loop 表示一个无限循环
///     嵌套循环需要使用标签来声明内外层循环
///     将操作值放在break之后，即可被返回
/// while条件循环，当条件满足时循环
/// match匹配，类似switch，比对每一个分支
fn main() {
    println!("Hello, world!");

    ifelse();

    loops();

    whiles();

    fors();

    matchs();

    match_deconstruction();
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

}
