use std::num::ParseIntError;

/// 处理复杂错误类型
pub fn complex() {
    println!("------------------------------complex Err");
    let vec1 = vec!["12","34","56"];
    let vec_empty = vec![];
    let vec2 = vec!["12","e","33"];

    println!("vec1: {:?}", double_first(vec1));
    println!("vec1: {:?}", double_first(vec_empty));
    println!("vec1: {:?}", double_first(vec2));

    let vec3 = vec!["12","34","56"];
    let vec_empty1 = vec![];
    let vec4 = vec!["12","e","33"];

    println!("vec1: {:?}", double_first_v2(vec3));
    println!("vec1: {:?}", double_first_v2(vec_empty1));
    println!("vec1: {:?}", double_first_v2(vec4));
}


fn double_first(vec: Vec<&str>) -> Option<Result<i32,ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn double_first_v2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
}

/// 自定义错误类型
//
// type Result<T> = std::result::Result<T, DoubleError>;
//
// #[derive(Debug,Clone)]
// struct DoubleError;
//
// impl fmt::Display for DoubleError {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "invalid first item to double")
//     }
// }
//
// impl Error for DoubleError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }
// }