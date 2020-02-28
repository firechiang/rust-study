use std::borrow::Borrow;

/**
 * Rust枚举类型用法和配合match（匹配）简单使用
 */
fn main() {
    let ipAddr = IpAddr::new(IpAddrKind::V4,String::from("127.0.0.1"));
    println!("ipAddr={:?}",ipAddr);
    let ipAddr1 = IpAddr::newIpAddr1();
    println!("ipAddr1={:?}",ipAddr1);

    let ww = Message::Write(String::from("测试复杂枚举"));
    println!("是否是Message::Write={}",ww.isWrite());
    println!("ww={:?}",&ww);
    println!("是否是Message::Write={}",isWrite(ww));


    let coin = Coin::Quarter(State::One);
    let coinValue = value_in_coin(coin);
    println!("coinValue={}",coinValue);

    /******************枚举匹配带if简单使用***************/
    let array1 = [
        King::Animal(12,"毛毛"),
        King::Plant(16,"天剑")
    ];
    let mut oldSize: u32 = 13;
    for item in &array1 {
        match *item {
            King::Plant(size,name) if size > oldSize => {
                println!("打印1");
            },
            King::Animal(size,name) => println!("打印2"),
            // King::Animal或者是King::Plant也可以看成是默认情况，点代表参数站位符，2个点就代表2个参数（默认情况也可以像最下面那样写）（注意：一定要写默认情况，否则上面的匹配里面不能写if）
            King::Animal(..) | King::Plant(..) => ()
            // 其它情况或默认情况
            //_ => ()
        }
    }
}

/**
 * 定义一个枚举类型
 */
enum King {
    // 注意： & 'static str 表示一个静态字符串
    Plant(u32,& 'static str),
    Animal(u32,& 'static str)
}

/**
 * 定义一个枚举类型
 */
#[derive(Debug)]
enum IpAddrKind  {
    V4,
    V6
}

/**
 * 定义一个枚举
 */
#[derive(Debug)]
enum IpAddr1 {
    V4(u32,u32,u32,u32),
    V6(String)
}

/**
 * 定义一个枚举
 */
#[derive(Debug)]
enum Message {
    Quit,
    //Move { x: i32, y: i32 },这个不能用(因为这个是匿名类，用了这个下面的match会报错)
    Write(String),
    ChangeColor(i32, i32, i32),
}

/**
 * 定义一个枚举
 */
#[derive(Debug)]
enum State {
    One,
    Two
}

/**
 * 定义一个枚举
 */
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

fn value_in_coin (c: Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("我是State的One，state={:?}",state);
            return 4;
        }
    }
}

/**
 * 定义枚举类Message里面的函数
 */
impl Message {
    //判断是不是枚举类型里面的Write类型
    fn isWrite(&self) -> bool {
        match *self {
            // _ 表示默认占用参数位的情况
            Message::Write(_) => true,
            // _ 表示默认占用参数位的情况
            Message::ChangeColor(_,_,_) => false,
            //Message::Move => false,//这个不能用（因为Move是匿名对象）
            Message::Quit => false
        }
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String
}

impl IpAddr {

    fn newIpAddr1() -> IpAddr1 {

        return IpAddr1::V4(127,0,0,1);
    }

    fn new(kind: IpAddrKind,addr: String) -> IpAddr {

        return IpAddr {
            kind: kind,
            addr: addr
        }
    }

}

/**
 * 判断是不是枚举里面的Write类型
 */
fn isWrite(message: Message) -> bool {
    match message {
        // _ 表示默认占用参数位的情况
        Message::Write(_) => true,
        // _ 表示默认占用参数位的情况
        Message::ChangeColor(_,_,_) => false,
        //Message::Move => false,//这个不能用（因为Move是匿名对象）
        Message::Quit => false
    }
}
