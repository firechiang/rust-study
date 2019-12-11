use std::ops::Deref;
/**
 * 使用引用回调，销毁内存回调
 */
fn main() {
    // 使用引用就回调
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    // 销毁内存就回调
    let c = CustomSmartPointer { data: String::from("my stuff") };
    // 强制销毁内存
    //drop(c);
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


/**
 * 实现Deref接口，只要使用了MyBox引用就会调用deref函数（注意：这个特性可做拦截器）
 */
impl<T> Deref for MyBox<T> {
    type Target = T;

    // 只要代码里面使用了MyBox的引用就会调用这个函数
    fn deref(&self) -> &T {
        println!("使用引用了");
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

/**
 * 实现Drop接口，只要销毁CustomSmartPointer引用就会调用drop函数（注意：这个特性可做拦截器）
 */
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("销毁了 `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
