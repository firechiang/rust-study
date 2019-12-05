/**
 * 枚举类型简单使用
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
 * 定义枚举类Message里面的函数
 */
impl Message {
    //判断是不是枚举类型里面的Write类型
    fn isWrite(&self) -> bool {
        match *self {
            Message::Write(_) => true,
            // _ 表示占用参数位置
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
        Message::Write(_) => true,
        // _ 表示占用参数位置
        Message::ChangeColor(_,_,_) => false,
        //Message::Move => false,//这个不能用（因为Move是匿名对象）
        Message::Quit => false
    }
}
