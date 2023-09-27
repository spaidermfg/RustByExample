use std::num::ParseIntError;

/// Result 描述的是可能是可能的错误而不是可能的不存在
/// OK<T>  Err<E>
/// 如果某个Result可能被重用，可以为其取一个别名

type AliiasedResult<T> = Result<T, ParseIntError>;

pub fn r#use() {
    println!("---------------------------result");

    let i = "2".parse::<i32>().unwrap();
    println!("{}", i);

    let twenty = multiply_v1("10", "2");
    print(twenty);

    let tt = multiply_v1("4", "m");
    print(tt);

    let t = multiply_v2("5", "4");
    print(t);
}

fn multiply_v1(first_num_str: &str, second_num_str: &str) -> AliiasedResult<i32> {
    match first_num_str.parse::<i32>() {
        Err(e) => Err(e),
        Ok(first_num) => {
            match second_num_str.parse::<i32>() {
                Err(e) => Err(e),
                Ok(second_num) => {
                    Ok(first_num * second_num)
                }
            }
        }
    }
}

fn multiply_v2(first_num_str: &str, second_num_str: &str) -> AliiasedResult<i32> {
     first_num_str.parse::<i32>().and_then(|first_num| {
         second_num_str.parse::<i32>().map(|second_str|
         first_num * second_str)
     })
}

fn print(result: AliiasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


