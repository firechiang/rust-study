/**
 * Rust对空值的处理，Option的用法（就是null对象的处理）
 */
fn main() {
    // 初始化一个有值的Option（<u32>表示泛型）
    let x: Option<u32> = Some(5);
    // 初始化一个没有值的Option，就是一个空的Option（<u32>表示泛型）
    let y: Option<u32> = None;
    // 获取Option对象里面的值
    println!("获取x的值={}",x.unwrap());
    let plusValue1 = plus_one(x);
    println!("plusValue1={:?}",plusValue1);
    let plusValue2 = plus_one(y);
    println!("plusValue2={:?}",plusValue2);
    // 调用函数传递null值
    flus_match(None);
    flus_match(Some(1));
    // 如果x != null
    if x != None {
        println!("x != null");
    }

    let coin = Coin::Quarter(State::One);
    let coinValue = value_in_coin(coin);
    println!("coinValue={}",coinValue);

    let op = Some(5);

    if Some(5) == op {
        println!("5==5");
    }
    // 这个写法和上面的写法所表示的含义是一样的
    if let Some(5) = op {
        println!("5==5");
    }

    if 2 == 2 {
        println!("2==2");
    }
}
/**
 * Option和JAVA的Optional一样，主要用来处理空值
 */
fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        // 如果是空值的话就返回空的Option
        None => None,
        // 如果不是空值的话就加1再返回
        Some(x) => Some(x + 1)
    }
}

/**
 * Option的简单使用
 */
fn flus_match (x: Option<u32>) -> Option<u32> {
    match x {
        Some(1) => println!("等于1"),
        Some(2) => println!("等于2"),
        // _ 可以表示default的情况
        _ => println!("默认情况"),
        //_ => (),
    }
    return Some(1);
}

fn value_in_coin (c: Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("state={:?}",state);
            return 4;
        }
    }
}

#[derive(Debug)]
enum State {
    One,
    Two
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}
