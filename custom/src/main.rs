
///自定义类型：struct and enum      
///结构体   
///元组结构体：具名元组； C 结构体； 单元结构体，常用于泛型中       
///枚举
///允许创建一个从数个不同取值中选其一的枚举类型
fn main() {
    
    println!("Hello, world!");

    structs();

    use_inspect();
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//单元结构体
struct Unit;

//元组结构体
struct Pair(i32, f32);

//C 风格结构体
struct Point {
    x: f32,
    y: f32,
}

//结构体嵌套,作为另一个结构体的字段
#[allow(dead_code)] // 该属性隐藏对未使用代码的警告
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

//结构体
fn structs() {
    //初始化结构体
    let name = String::from("mark");
    let age = 17u8;
    let mark = Person{name, age};

    //以Debug方式打印结构体
    println!("Person: {:?}", mark);


    //实例化结构体
    let point: Point = Point { x: 10.3, y: 7.9 };
    //访问结构体的字段
    println!("point location: [{} {}]", point.x, point.y);
    

    //使用结构体更新语法使用之前的结构体创建新结构体
    let bottom_right = Point{x: 4.23 , ..point};
    println!("second point: [{} {}]", bottom_right.x, bottom_right.y);


    //使用let绑定来解构结构体
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle{
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    

    //实例化一个单元结构体
    let _unit = Unit;

    //实例化一个元组结构体
    let pair = Pair(2, 3.23);
    //访问元组结构体中的元素
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //解构一个元组结构体
    let Pair(a, b) = pair;
    println!("pair contains {} and {}", a, b);


    //获取一个长方形
    let point1 = Point{ x: 10.4, y: 2.8};
    let f = 5.6;
    let rectangle1 = square(point1, f);

    //获取长方形面积
    let area = rect_area(rectangle1);
    println!("area: {}", area);
    
}

//计算长方形面积
fn rect_area(rectangle: Rectangle) -> f32 {
   let Rectangle{ top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 }} = rectangle; 
    (x1 - x2) * (y2 - y1)
}

fn square(point: Point, f: f32) -> Rectangle {
    Rectangle { 
        top_left: point, 
        bottom_right: Point { x: f, y: f },
    }
    
}


///枚举类型
///定义一个枚举类型
enum WebEvenet {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y:i64}
}

///使用enum作为函数参数
fn inspect(event: WebEvenet) {
    match event {
        WebEvenet::PageLoad => println!("page loaded"),
        WebEvenet::PageUnload => println!("page unloaded"),
        WebEvenet::Paste(s) => println!("pasted: {}", s),
        WebEvenet::KeyPress(c) => println!("user pressed '{}'",c ),
        WebEvenet::Click { x, y } => {
            println!("clicked at x = {}, y = {}", x, y);
        },
    }
}


fn use_inspect() {
    let pressed = WebEvenet::KeyPress('a');
    let paste = WebEvenet::Paste('u'.to_string().to_owned());
    let click = WebEvenet::Click { x: 88, y: 99 };
    let load = WebEvenet::PageLoad;
    let unload = WebEvenet::PageUnload;

    inspect(pressed);
    inspect(paste);
    inspect(click);
    inspect(load);
    inspect(unload);

    //使用use关键字
    //无需指定完整路径名
    
    use WebEvenet::*;

    //等价于 WebEvenet::PageLoad;
    let _loaded = PageLoad;
    let press = KeyPress('s');

    //未使用完整路径
    match press {
        PageLoad => println!("page loaded"),
        KeyPress(s) => println!("second user press {}", s),
        PageUnload => println!("page unload second"),
        Paste(a) => println!("paste the {}", a),
        Click { x, y } => {
            println!("she click the x = {} , y ={}", x, y);
        },
    };
}


