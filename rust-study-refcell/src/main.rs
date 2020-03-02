use std::rc::{Rc, Weak};
use std::cell::RefCell;

/**
 * 弱引用简单使用（如果a引用b，b再引用a，恰巧又是强引用的话就会造成栈溢出。使用弱引用就没有这个问题）
 */
fn main() {

    // 孩子对象
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 父亲对象
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        // 孩子是 leaf
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 注意：downgrade函数是获取branch的弱引用
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}