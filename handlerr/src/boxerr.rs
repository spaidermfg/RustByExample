use std::fmt::Display;
use std::{error, fmt};
use error::Error;
use std::fmt::Formatter;

/// 将错误存放在Box中
/// 被包装的错误类型只能在运行时了解，不能被静态判别

pub fn box_err() {
    println!("-----------------------------------box err");
    let vec1 = vec!["12","34","56"];
    let vec_empty = vec![];
    let vec2 = vec!["t","e","33"];

    print(double2first(vec1));
    print(double2first(vec_empty));
    print(double2first(vec2));
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug,Clone)]
struct EmptyVec;

impl Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double.")
    }
}

impl Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first double is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn double2first(vec: Vec<&str>) -> Result<i32> {
    vec.first().ok_or_else(|| EmptyVec.into()).and_then(|s| {
        s.parse::<i32>().map_err(|e| e.into()).map(|i| 2 * i)
    })
}

