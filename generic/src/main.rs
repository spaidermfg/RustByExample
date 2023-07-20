/// # 泛型
/// 泛化类型和函数，减少重复代码
/// 泛型类型参数一般用<T>表示
fn main() {
    
    println!("Hello, world!");

    use_generic();
}


//泛型结构体
struct SGen<T>(T);

//接受一个泛型参数
fn generic<T>(_s : SGen<T>) {}

//使用泛型函数
fn use_generic() {
    //显式的指定类型参数
    generic::<i32>(SGen(8));

    //隐式的指定类型参数
    generic(SGen(9));
    
}
