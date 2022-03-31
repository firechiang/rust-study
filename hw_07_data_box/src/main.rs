/*
 * Box 智能指针简单使用
 * 注意：Rust默认创建的数据都是在栈上面的，想要把数据放到堆上就需要使用Box智能指针
 * Box 应用场景
 * 1，当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型的时候使用（就是链表数据使用Box结构）
 * 2，当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候使用
 * 3，当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
 */
use std::fmt::Debug;

fn main() {
    boxx();
}


fn boxx() {
    // 将i32数据放到堆上
    let b = Box::new(5);
    println!("b={}",b);
    let list = List::Cons(0,Box::new(List::Cons(1,Box::new(List::Bil))));
    println!("list={:#?}",list);
    // 定义一个数组，，每个元素的值是0，长度是 1024 * 2
    // 注意：该数组数据默认是存在栈上面的
    let array = [0;1024 * 2];
    // 将数组的数据拷贝到堆上
    let array_box = Box::new(array);
    println!("array_box={:#?}",array_box);

    // 现在栈上分配内存数据，再将数据拷贝到堆上
    let array_box1 = Box::new([0;1024*2]);
    println!("array_box1={:#?}",array_box1);

    let man = person_factory(1);
    println!("man={:?}",man);

    let woman = person_factory(2);
    println!("woman={:?}",woman);
}

#[derive(Debug)]
enum List {
    Cons(i32,Box<List>),
    Bil
}

// 定义一个接口（注意：该接口继承了Debug）
trait Person:Debug{
    fn get_name(&self) -> &str;
}

#[derive(Debug)]
pub struct Man;

#[derive(Debug)]
pub struct Woman;

// 男人实现接口
impl Person for Man {

    fn get_name(&self) -> &str {
        "man"
    }

}
// 女人实现接口
impl Person for Woman {
    fn get_name(&self) -> &str {
        "woman"
    }
}

// 多态根据类型返回不同的实现
fn person_factory(typex: u32) -> Box<dyn Person> {
    match typex {
        1 => Box::new(Man),
        2 => Box::new(Woman),
        _ => unimplemented!()
    }
}

// ConsLint

