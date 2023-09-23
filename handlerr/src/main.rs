mod who_people;
mod result;

/// 错误处理
fn main() {
    give_princess("juice");
    //give_princess("hello");  //panic

    use_unwrap();

    use_and_then();

    result::r#use()
}

fn use_unwrap() {
    // match
    let hello = Some("hello");
    let flower = Some("rose");
    let none = None;

    gift_commoner(hello);
    gift_commoner(flower);
    gift_commoner(none);

    // unwrap
    let water = Some("coke");
    //let void = None;

    gift_princess(water);
    //gift_princess(void);  //panic

    // ?
    //let age = None;
    let age1 = Some(77);
    println!("{:?}", next_birthday(age1));

    let p = who_people::Person {
        job: Some(who_people::Job{
            phone_number: Some(who_people::PhoneNumber {
                area_code: Some(77),
                number: 328674983,
            }),
        }),
    };

    println!("{:?}", p.work_phone_area_code());

    //--------map
    use_combinator()
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

/// 使用？， 空则返回None， 有值则返回Some(x)
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age = current_age?;
    Some(format!("Next year I will be {}", next_age))
}

/// 组合算子 combinator
/// 以模块化的风格来管理控制流
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

struct Peeled(Food);
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

// 如果是Food，就削皮
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// 如果是削好皮的Food，就切块
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// 如果是切好块的Food，就烹饪
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// 一条龙
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))

}

// 如果是烹饪好的Food，就恰
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Emm, I love {:?}", food),
        None => println!("Oh no, It wasn't edible.")
    }
}

fn use_combinator() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

/// and_then() 使用被Option包裹的值来调用其输入的函数并返回结果
#[derive(Debug)]
enum Foods {
    CordonBleu,
    Steak,
    Sushi,
}


#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

fn have_ingredients(foods: Foods) -> Option<Foods> {
    match foods {
        Foods::Sushi => None,
        _ => Some(foods),
    }
}

fn have_recipe(foods: Foods) -> Option<Foods> {
    match foods {
        Foods::CordonBleu => None,
        _ => Some(foods),
    }
}

fn cookable_v1(foods: Foods) -> Option<Foods> {
    match have_ingredients(foods) {
        None => None,
        Some(foods) => match have_recipe(foods) {
            None => None,
            Some(foods) => Some(foods),
        }
    }
}

fn cookable_v2(foods: Foods) -> Option<Foods> {
    have_ingredients(foods).and_then(have_recipe)
}

fn eats(foods: Foods, day: Day) {
    match cookable_v2(foods) {
        Some(foods) => println!("Yay! On {:?} we get to eat {:?}", day, foods),
        None => println!("Oh no!, we don't get to eat on {:?}?", day),
    }
}

fn use_and_then() {
    let (cordon_bleu, steak, sushi) = (Foods::CordonBleu, Foods::Steak, Foods::Sushi);

    eats(cordon_bleu, Day::Monday);
    eats(steak, Day::Tuesday);
    eats(sushi, Day::Wednesday);
}

