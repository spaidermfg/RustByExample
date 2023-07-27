pub mod loan;

/// 访问数据但并不取得所有权
/// 对象通过引用&T传递，而非值T传递
/// 对象在其作用域中被借用时，后续不能进行销毁操作
/// 可变引用
/// 借用者可读可写对象, 不能可变的借用一个不可变对象
/// ref关键字可以创建对象的引用
pub fn borrow_test() {
    println!("'borrow test");

    use_borrow();

    mut_borrow();

    ref_borrow();
}

// 取得一个box并销毁
fn destory_box(box32: Box<i32>) {
    println!("Destorying box that contains: {}", box32);
}

// 借用一个i32类型
fn borrow_i32(bor32: &i32) {
    println!("This int32 is : {}", bor32);
}

fn use_borrow() {
    let a = 67;
    let b = Box::new(67);

    borrow_i32(&a);
    borrow_i32(&b);

    //不存在引用可以销毁
    destory_box(b);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

//接受一个不可变的引用
fn borrow_book(book: &Book) {
    println!(
        "I borrow a book: {} - {} - {} edition",
        book.title, book.author, book.year
    );
}

//接受一个可变引用，并修改时间
fn borrow_mut_book(book: &mut Book) {
    book.year = 2019;
    println!(
        "I borrow a book: {} - {} - {} edition",
        book.title, book.author, book.year
    );
}

fn mut_borrow() {
    let book = Book {
        author: "Dargon ton",
        title: "《The art of linux》",
        year: 2021,
    };

    //创建一个可变拷贝
    let mut mut_book = book;

    //不可变引用借用一个不可变引用
    borrow_book(&book);

    //不可变引用借用一个可变引用
    borrow_book(&mut_book);

    //可变的借用一个可变引用
    borrow_mut_book(&mut mut_book);

    //不能可变的借用一个不可变引用
    //borrow_mut_book(&mut book);
}

fn ref_borrow() {
    let c = 'C';

    //下面两种写法一致，ref可以创建对象的引用
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("equal?, {}", *ref_c1 == *ref_c2);

    let point = Point { x: 6, y: 7 };

    //解构结构体时使用ref
    let _copy_of_x = {
        //解构结构体
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        *ref_to_x
    };

    let mut mut_point = point;

    {
        //解构结构体
        let Point {
            x: _,
            y: ref mut mut_ref_of_y,
        } = mut_point;
        *mut_ref_of_y = 1;
    }

    println!("mut point is: [{} {}]", mut_point.x, mut_point.y);

    let mut mut_tuple = (Box::new(6u32), 7u32);

    {
        //解构元组
        let (_, ref mut last) = mut_tuple;

        *last = 67;
    }

    println!("tuple is {:?}", mut_tuple);
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
