/// 错误处理
fn main() {
    give_princess("juice");
    //give_princess("hello");  //panic

    use_unwrap();

}

fn use_unwrap() {
    let hello = Some("hello");
    let flower = Some("rose");
    let none = None;

    gift_commoner(hello);
    gift_commoner(flower);
    gift_commoner(none);

    let water = Some("coke");
    let void = None;

    gift_princess(water);
    gift_princess(void);
}


/// panic
/// 用来处理不可恢复的错误
fn give_princess(gift: &str) {
    if gift == "hello" {
        panic!("shit hello");
    }

    println!("{} is beautiful!", gift);
}

/// Option and unwrap
/// Option<T>
/// 有选项的枚举类型，适用于有不存在的可能性的情况。Some(T)、None
/// 可通过match显式的处理，或者适用unwrap隐式的处理(要么返回Some内的元素，要么panic)
/// 使用unwrap隐式的处理
fn gift_princess(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "hello" {
        panic!("shit hello");
    }

    println!("{} is beautiful!", inside);
}

/// 使用match显式的处理
fn gift_commoner(gift: Option<&str>) {
    match gift {
        Some("hello") => println!("hello world"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("shit man"),
    }
}
