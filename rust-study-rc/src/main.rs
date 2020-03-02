/**
 * Rc智能指针包装变量或数据可让一个值有多个所有者，让变量或数据不会出作用域就销毁（Rc可获取被引用数量）
 * rc数据类型简单使用（rc可获被引用数量）
 * Rc让一个值有多个所有者，调用clone产生一个指针指向该值，当Rc指针全部销毁时，该值也销毁
 *
 * 注意：不能通过Rc获得可变引用，Rc是非原子引用，只能用于单线程，多线程用Arc
 * 注意：如果a引用b，b再引用a，会造成栈溢出
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

    let five = Rc::new(5);
    // 调用downgrade函数获取five的弱引用（注意：Weak对象就是当前five的弱引用，但是没有其所有权）
    let weak_five = Rc::downgrade(&five);
    // 调用upgrade函数,获取five的值（注意：这个值是 Optical<Rc<T>> 类型）
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    // 获取five引用实际的值
    println!("{}", strong_five.unwrap());
    println!("弱引用数量 weak = {}, 强引用数量 strong = {}", Rc::weak_count(&five), Rc::strong_count(&five));
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;



