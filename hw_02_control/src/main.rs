extern crate core;

use std::thread;
use std::io;
use rand::Rng;
/**
 * 流程控制
 */
fn main() {
    let if_res = if_res();
    println!("res={}",if_res);
    if_else_use();
    loop_use();
    loop_use_res();
    while_use();
    for_range();
    for_iter();
    match_use();
    if_let();
    while_use();
    fn_use();
    fn_closure();
    higher_function_use();

    let a = 11;
    let b = if a == 11 {
        10
    } else {
        // 本来需要返回i32类型的数值，但是我们可以使用发散函数替代，这样程序执行到这里就退出了
        foo()
    };
    println!("函数执行到了最后={}",b);
    guess_number();
}

enum Alphat {
    A,
    B
}

enum Symbol {
    Char(char),
    Number,
}

/*
 * 猜数字游戏
 */
fn guess_number() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("请输入一个数字:");
        let mut guess = String::new();
        // 读取控制台输出
        io::stdin().read_line(&mut guess).unwrap();
        // 将控制台的输入先去掉空格再转换成u32数值
        let guess_num:i32 = match guess.trim().parse() {
            Ok(num) => num,
            // 跳出循环
            Err(err) => continue,
        };
        if secret_number > guess_num {
            println!("输入小了");
        }else if secret_number < guess_num {
            println!("输入大了");
        } else {
            println!("恭喜，正确!");
            break;
        }

    }
}

/**
 * 发散函数简单使用，就是用!号标识返回值的函数就是发散函数，但是发散函数不能有返回值，在函数体里面也不能写返回（其实和没有返回值的函数差不多，就是没有返回值的函数是可以在代码里面写返回的，但是发散函数不行）
 * 发散函数可以绑定到任意类型上，因为它就用不会返回
 * 注意：发散函数必须使用panic结尾，panic就是程序退出
 */
fn foo() -> ! {
    panic!("我是发散函数");
}

fn higher_function_use() {
    // 第二个参数是传递匿名函数，也可以传递一个已有函数的名字
    let c = higher_function_param(10,|a:u32,b:u32| -> u32 {
        a + b
    });
    println!("c={}",c);
    // 调用函数获取函数返回值
    let method = higher_function_res("a");
    // 调用是返回值的函数
    let res = method(10,30);
    println!("res={}",res);
}

/**
 * 高阶函数简单使用（就是将函数当参数传递给另一个函数，或者返回值是函数）
 * 当前这个函数就是返回值是函数，只不过是通过type关键字将函数描述定义成了一个类型了
 */
fn higher_function_res(str: &str) -> fn(u32,u32) -> u32 {
    match str {
        "a" => {
            // 返回函数，也可以直接返回已经定义好了的函数的名称
            |a:u32,b:u32| -> u32 {
                a + b
            }
        }
        // 默认返回一个未实现的函数
        _ => unimplemented!()
    }
}

/**
 * 高阶函数简单使用（就是将函数当参数传递给另一个函数，或者返回值是函数）
 * 当前这个函数就是返回值是函数
 */
fn higher_function_res2() -> Method {
    // 返回函数，也可以直接返回已经定义好了的函数的名称
    |a:u32,b:u32| -> u32 {
        a + b
    }
}

/**
 * 高阶函数简单使用（就是将函数当参数传递给另一个函数，或者返回值是函数）
 * 当前这个函数就是可以接受函数作为参数（第二个参数就是函数）
 */
fn higher_function_param(a:u32,method: fn(u32,u32)->u32) -> u32 {
    let b:u32 = 10;
    return method(a,b);
}

type Method = fn(u32,u32) -> u32;
/**
 * 高阶函数简单使用（就是将函数当参数传递给另一个函数，或者返回值是函数）
 * 当前这个函数就是可以接受函数作为参数（第二个参数就是函数，只不过是通过type关键字将函数描述定义成了一个类型了）
 */
fn higher_function_param2(a:u32,method: Method) -> u32 {
    let b:u32 = 10;
    return method(a,b);
}

/**
 * 函数闭包简单使用（就是匿名函数）
 */
fn fn_closure() {
    // 定义一个匿名函数，它的参数是n，返回值类型是u32
    let time = |n:i32| -> u32 {
        20
    };
    let res = time(10);
    println!("匿名函数返回的内容等于={}",res);

    let str = String::from("maomoa");

    // 传递匿名函数简单使用（注意：这个创建线程执行体，还有move关键字表示主线程里面的变量可以在子线程里面用）
    thread::spawn(move || {
        println!("str={}",str);
    }).join();
}

/**
 * 函数与方法简单使用
 * 函数使用fn开头定义，当前这个叫函数使用 A::xx() 方式调用（类似于Java的静态方法）。写在 impl 结构里面参数不带self的叫方法，带self的叫函数
 */
fn fn_use() {
    let user = User::new(String::from("maomao"));
    println!("user={:#?}",user);
}

/**
 * while let 语法糖简单使用
 */
fn while_let() {
    let mut alphat = Alphat::A;
    // 只要alphat=Alphat::A就一直循环下去
    while let Alphat::A = alphat {
        println!("循环");
        alphat = Alphat::B;
    }
}

#[derive(Debug)]
struct User {
    name:String,
}

impl User {
    // 构造函数（注意：Rust里面的构造函数和别的函数没有区别，就是约定俗成的函数名字叫new）
    fn new(name:String) -> User {
        User {
            name
        }
    }
}

/**
 * if let 语法糖简单使用
 * 应用场景就是match匹配单个选项
 */
fn if_let() {
    let alphat = Alphat::A;
    if let Alphat::A = alphat {
        println!("匹配到单个match结果");
    }

    let symbol = Symbol::Char('A');
    // 匹配单个结果
    if let Symbol::Char(x) = symbol {
        println!("Symbol::Char枚举char的值={}",x);
    }
}


/**
 * match 匹配简单使用
 */
fn match_use() {
    let alphat = Alphat::A;
    // match 匹配枚举
    match alphat {
        Alphat::A => {
            println!("Alphat=a");
        }
        _ => {}
    }
    // match 直接匹配数字
    let num = 10;
    match num {
        10 => {
            println!("num=10");
        }
        _=>{}
    }
}

/*
 * for迭代器简单使用
 */
fn for_iter() {
    let mut array = [1,2,3];
    // 迭代遍历数组的每一个元素
    for item in array.iter() {
        print!("i={}",item);
    }
    println!("");
    // 迭代数组的每一个元素并修改元素的值
    for item in array.iter_mut() {
        *item = *item + 2;
        //*item += 2;
    }
    println!("array={:#?}",array);
}

/**
 * for range 循环简单使用
 */
fn for_range() {
    // i < 5 的循环
    for i in 0..5 {
        print!("i={}",i);
    }
    println!("");
    // i <= 5 的循环
    for i in 0..=5 {
        print!("i={}",i);
    }
    println!("");
}

/**
 * while循环简单使用
 */
fn while_use() {
    let mut n = 1;
    while n < 101 {
        if n % 3 == 0 {
            print!("n是3的倍数");
        } else if n % 5 == 0 {
            print!("n是5的倍数");
        }
        n += 1;
    }
    println!("");
}

/**
 * loop循环简单使用（逻辑：1 + 2 + 3.. 一直加到100）
 */
fn loop_use() {
    let mut sum = 0;
    let mut count = 1;
    loop {
        sum += count;
        count += 1;
        if count > 100 {
            break;
        }
    }
    println!("1+2+3...+10={}",sum);
}

/**
 * loop循环带返回值简单使用
 */
fn loop_use_res() {
    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            // 这个代码的意思是执行break之前先执行后面的表达式
            break counter = counter * 2
        }
    };
    println!("loop循环语句带返回值={}",counter);
}

/**
 * if else简单使用
 */
fn if_else_use() {
    let count = 10;
    if count > 5 {
        println!("count大于5");
    } else if count > 6 {
        println!("count大于6");
    } else {
        println!("其他情况");
    }
}

/**
 * if语句带返回值简单使用
 */
fn if_res() -> i32 {
    let is = true;
    let res = if is {
        10
    } else {
        20
    };
    return res;
}
