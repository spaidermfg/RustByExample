#[allow(unreachable_code)]
/// # 流程控制
/// ifelse后条件无需小括号
/// loop 表示一个无限循环
///     嵌套循环需要使用标签来声明内外层循环
///     将操作值放在break之后，即可被返回
/// while条件循环，当条件满足时循环
fn main() {
    println!("Hello, world!");

    ifelse();

    loops();

    whiles();
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
