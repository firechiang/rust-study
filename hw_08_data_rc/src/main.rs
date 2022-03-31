use std::rc::Rc;

/**
 * Rc 智能指针简单使用
 * 该智能指针用于引用计数，也就是当没有东西再引用该数据时该数据将会在堆上被自动删除
 */
fn main() {
    // 创建一个数据并使用引用计数器记录其引用次数
    let four = Rc::new(List::Cons(4,Rc::new(List::Nil)));
    // four 变量第一次被引用（注意：Rc::clone 只是添加引用计数次数）
    let zero_one = List::Cons(0,Rc::new(List::Cons(1,Rc::clone(&four))));
    // four 变量第二次被引用
    let two_three = List::Cons(2,four);
    println!("four={:#?}",two_three);
}

#[derive(Debug)]
enum List {
    Cons(i32,Rc<List>),
    Nil,
}
