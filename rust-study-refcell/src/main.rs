use std::rc::{Rc, Weak};
use std::cell::RefCell;

/**
 * 弱引用简单使用（如果a引用b，b再引用a，恰巧又是强引用的话就会造成栈溢出。使用弱引用就没有这个问题）
 */
fn main() {

    // 孩子对象
    let leaf = Rc::new(Node {
        value: 3,
        // 父亲是一个弱引用
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // 调用upgrade函数,获取five的值（注意：这个值是 Optical<Rc<T>> 类型）
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 父亲对象
    let branch = Rc::new(Node {
        value: 5,
        // 父亲的父亲是一个弱引用
        parent: RefCell::new(Weak::new()),
        // 孩子是 leaf
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 注意：downgrade函数是获取branch的弱引用
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // 调用upgrade函数,获取five的值（注意：这个值是 Optical<Rc<T>> 类型）
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    //------------------------------详细的引用流程-------------------------------------//

    // 定义孩子对象
    let leaf = Rc::new(Node {
        value: 3,
        // 父亲是一个弱引用
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("孩子对象的强引用数 leaf strong = {},孩子对象的弱引用数 weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
    // 新的作用域
    {
        // 父亲对象
        let branch = Rc::new(Node {
            value: 5,
            // 父亲的父亲是一个弱引用
            parent: RefCell::new(Weak::new()),
            // 引用孩子对象
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        // Rc::downgrade(&branch)是获取父亲对象的弱引用
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("父亲对象的强引用 branch strong = {}, 父亲对象的弱引用 weak = {}",Rc::strong_count(&branch),Rc::weak_count(&branch));
        println!("孩子对象的强引用数 leaf strong = {}, 孩子对象的弱引用数 weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf),);
    }
    // 调用upgrade函数,获取five的值（注意：这个值是 Optical<Rc<T>> 类型）
    // 注意：这个值是空的，那是因为到这里已经出了 父亲对象 branch 的作用域，所以branch对象的值是空的
    println!("孩子对象的父亲 leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("孩子对象的强引用数 leaf strong = {}, 孩子对象的弱引用数 weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}