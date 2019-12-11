/**
 * rc数据类型简单使用（rc可获被引用数量）
 */
fn main() {
    let aa = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let bb = Cons(3, Rc::clone(&aa));
    let cc = Cons(4, Rc::clone(&aa));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a创建后，有 {} 个指针指向了a", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("b创建后，有 {} 个指针指向了a", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("c创建后，有 {} 个指针指向了a", Rc::strong_count(&a));
    }
    println!("c销毁后，有 {} 个指针指向了a", Rc::strong_count(&a));
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;



