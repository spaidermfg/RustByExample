use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// 处理复杂错误类型
pub fn complex() {
    println!("------------------------------complex Err");
    // let vec1 = vec!["12","34","56"];
    // let vec_empty = vec![];
    // let vec2 = vec!["12","e","33"];
    //
    // println!("vec1: {:?}", double_first(vec1));
    // println!("vec1: {:?}", double_first(vec_empty));
    // println!("vec1: {:?}", double_first(vec2));

    let vec3 = vec!["12","34","56"];
    let vec_empty1 = vec![];
    let vec4 = vec!["12","e","33"];

    print(double_first1(vec3));
    print(double_first1(vec_empty1));
    print(double_first1(vec4));

}

// fn double_first(vec: Vec<&str>) -> Option<Result<i32,ParseIntError>> {
//     vec.first().map(|first| {
//         first.parse::<i32>().map(|n| 2 * n)
//     })
// }
//
// fn double_first_v2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
//     let opt = vec.first().map(|first| {
//         first.parse::<i32>().map(|n| 2 * n)
//     });
//
//     opt.map_or(Ok(None), |r| r.map(Some))
// }

/// 自定义错误类型

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug,Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for DoubleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn double_first1(vec: Vec<&str>) -> Result<i32> {
    vec.first().ok_or(DoubleError).and_then(|s| {
        s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i)
    })
}