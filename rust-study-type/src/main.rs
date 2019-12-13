/**
 * 自定义类型简单使用
 */
fn main() {
    // 定义Kilometers的类型是i32类型
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // 定义一个复杂类型（它是回调函数，它是Send类型，它是'static类型）
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {

        Box::new(|| println!("hi"))
    }

}
