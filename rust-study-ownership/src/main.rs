use std::str::from_boxed_utf8_unchecked;

/**
 * 变量的有效范围简单测试和值传递对象传递说明，以及函数返回多个值的简单使用
 * 基础数据类型都是值传递，对象包括数组都是对象传递（注意：如果将一个变量的对象赋给另一个变量，那么前一个变量将无法使用，因为它的对象已经没有了）
 * 变量出作用域以后内存立即释放（Rust自动调动drop函数释放内存）
 */
fn main() {
    test_simaple_string();
    test_invalid_string();
    test_clone_string();
    test_u32();
    let s = String::from("测试参数");
    // 注意：变量s是通过对象传递给函数，当函数执行完成以后s将被释放
    test_string(s);
    //println!("测试变量s已经无法使用，因为内存已被释放，s={}",s)


    let value = 5;
    test_i32(value);
    println!("测试基础数据类型，值传递。在函数调用以后还可以使用。value={}",value);
    // 函数返回多个值（这个是返回2个）
    let (s,length) = test_string_array(String::from("测试数组传递"));
    println!("测试数组传递s={},length={}",s,length);

    let aaa = "sdasdas";
    let aaa = "我是修改后的值";
    // 可以使用定义一个同名变量来避免变量被释放
    println!("aaa={}",aaa);
    let bbb = String::from("阿三大苏打萨达");
    test_simepl_string1(&bbb);
    // 这里bbb还可以使用是因为上面的函数使用的是引用传递，而引用传递效率也最高（推荐使用）
    println!("bbb={}",bbb);
}

/**
 * 基础数据类型值传递
 */
fn test_i32 (value: u32) {
    println!("数字参数 value={}",value);
}

/**
 * 参数s出作用域（当前函数）以后将无法使用（因为内存被释放）
 */
fn test_string (s: String) {
    println!("参数s={}",s);
}

/**
 * 值传递
 */
fn test_u32 () {
    let i1 = 5;
    // 基础数据类型值传递
    let i2 = i1;
    // i1和i2读可以用是因为基础类型是值传递（如果不是基础数据类型就不行了）
    println!("i1={},i2={}",i1,i2);
}

/**
 * 对象传递
 */
fn test_invalid_string () {
    // 定义变量s1
    let s1 = String::from("我是变量的值");
    // 将s1的对象赋给s2（注意：赋完对象以后s1变量将不可用（因为s1变量的对象已经没有了））
    let s2 = s1;
    // 放开注释报错，因为s1变量的对象已经没有了（赋给了s2）
    //println!("看看s1变量是否还可用s1={}"，s1);
    println!("变量s2={}",s2);
}

fn test_string_array (s: String) -> (String,usize) {
    let length  = s.len();
    return (s,length);
}

fn test_clone_string () {
    let s1 = String::from("我是变量的值");
    // 复制一份s1的值赋给s2
    let s2 = s1.clone();
    println!("s1={},s2={}",s1,s2);
}

fn test_brock_string () {
    {
        let s = String::from("测试String");
        println!("作用域s={}",s);
    }
    // 编译失败
    //println!("s变量是不可用的，会产生错误，因为s已经出作用域，且内存也被释放了{}",s);
}

fn test_simaple_string() {
    let mut s = String::from("测试String");
    s.push_str("后面拼接的内容");
    println!("测试:{}",s);
}

fn test_simepl_string1(s: &String) {
    println!("最后引用传递s={}",s);
}
