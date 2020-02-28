/**
 * if分支语句简单使用
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
