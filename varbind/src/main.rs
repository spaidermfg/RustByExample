///变量绑定
///在声明变量时说明类型
///变量默认绑定不可变，使用mut关键字使其可变
///变量绑定拥有作用域，只能在一个代码块中生存
///使用遮蔽进行变量绑定，可以覆盖同名变量的值，数据类型也可以更改
///变量先声明但未初始化，会报错
///冻结：
///当数据被相同的名称不变的绑定时，就会被冻结。在为超出不可变绑定作用域之前，冻结变量不可以修改
fn main() {
    println!("Hello, world!");

    bind();
}


fn bind() {
    //使用let将字面量绑定到变量上
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value {:?}", unit);


    //use mut keywords
    let mut immutable_binding = 2;

    println!("Before mutation: {}", immutable_binding);

    immutable_binding *= 2;

    println!("After mutation: {}", immutable_binding);

    //out scopt k
    let out_value  = 1;

    {

        //inner scope 
        let inner_value = 2;

        println!("inner value: {}", inner_value);

        let out_value = 3_f32;

        println!("inner(out value): {}", out_value);
    }

    println!("out (out value): {}", out_value);

    // 使用遮蔽进行变量绑定，覆盖掉变量之前的值
    let out_value = true;

    println!("shadow value: {}", out_value);

    //declare variable
    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a binding variable: {}", a_binding);

    let another;

    //未初始化变量使用会报错
    //println!("another binding: {}", another);
    
    another = 77;
    println!("another variable: {}", another);


    //freeze
    let mut freeze_variable = 3;

    {
        let freeze_variable = freeze_variable;

        // 会报错， 已冻结变量不可以修改
        // freeze_variable = 77;
        println!("inner freeze variable: {}", freeze_variable);
    }

    freeze_variable = 56;

    println!("freeze variable: {}", freeze_variable);
    
}


