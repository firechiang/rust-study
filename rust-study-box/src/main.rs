/**
 * Box智能指针（堆上分配内存）和使用*号获取引用地址值的方法（注意：*号和&号获取引用都是调用Deref接口的deref函数取值）
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
        // y指向的东西加1（*y标识y引用所指向的值，y引用实际指向的是x=5，加1就是6，最后x就等于6）
        *y += 1;
    }
    println!("x={}",x);
    // 创建MyBox
    let mut myBox = MyBox::new(12);
    // *myBox表示调用了MyBox实现Deref接口的deref函数，所返回的值
    println!("myBox={:?}",*myBox);

    let m = MyBox::new(String::from("Rust"));
    // &m表示调用了MyBox实现Deref接口的deref函数，所返回的值
    hello(&m);

}

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 定义一个class
#[derive(Debug)]
struct MyBox<T>(T);

// class里面的函数
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

// MyBox类实现Deref接口
impl<T> Deref for MyBox<T> {
    type Target = T;

    // 返回MyBox第一个值的引用（注意：只要获取MyBox的引用都会调用这个函数取值，比如*MyBox,&MyBox实际调用的就是这个函数）
    fn deref(&self) -> &T {
        println!("Deref接口的deref函数被调用了");
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}







