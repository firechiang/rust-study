/**
 * Box智能指针数据类型简单使用
 * Box <T>是一个智能指针，指向在类型为T的堆上分配的数据。Box <T>允许将数据存储在堆而不是堆栈上
 */
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    // 套娃结构
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));

    let mut x = 5;
    {
        let y = &mut x;
        // y指向的东西加1
        *y += 1;
    }
    println!("x={}",x);

}

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}






