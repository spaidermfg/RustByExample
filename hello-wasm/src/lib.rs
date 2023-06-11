pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//引入外部库
extern crate wasm_bindgen;

//引入prelube模块下的所有方法
use wasm_bindgen::prelude::*;

//调用外部定义的函数
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

//给外部使用的函数，可以被javascript调用
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

