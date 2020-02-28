/**
 * if分支语句简单使用以及使用if判断进行赋值简单使用
 */
fn main() {
    let op = Some(5);
    if Some(5) == op {
        println!("5==5");
    }
    // 这个写法和上面的写法所表示的含义是一样的
    if let Some(5) = op {
        println!("5==5");
    }
    let c = Coin::Nickel;
    if let Coin::Quarter(state) = c {
        println!("等于c")
    }else {
        println!("不等于c")
    }

    if 2 == 2 {
        println!("2==2");
    }

    // 使用if判断后的返回值，进行赋值（注意：最后结束要加分号，否则报错）
    let number = if true {
        5
    }else{
        6
    };
    println!("number={}",number);
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
