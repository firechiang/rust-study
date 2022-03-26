/**
 * 整型
 * i 表示有符号数，u表示无符号
 * i8，u8
 * i16，u16
 * i32，u32
 * i64，u64
 * i128，u128
 *
 * 浮点型
 * f32,f64
 *
 * 布尔型
 * bool
 *
 * 字符型
 * char
 */
use std::mem;

// 定义常量（注意：常量是在编译期间运算），i32表示有符号数值
const NNAME: i32 = 1;

fn main() {
    // 变量默认是不可变的（可以在定义前面加上 mut 关键字使其可变），u32表示无符号数值
    let x: u32 = 5;
    //x = x + 1;
    // 注意：这个x和上面的x不是同一个内存地址
    let x = x + 1;
    // 带感叹号表示调用的是宏
    println!("x={}",x);

    let s = 's';
    println!("s={}",s);

    //let a = u32::max_value();
    let a:u32 = 221;
    let b:u32 = 1;
    let sum = a + b;
    print!("sum={}",overflow(a,b));

    testTuple();
    test_array();
    section();
    testStruct();
    testEnum();
    testConvert();
}

// 类型强制转换简单使用
fn testConvert() {
    // 将字符串强制转换成 u32 类型（注意：unwrap()函数表示程序正常执行获取结果，隐藏了错误处理）
    let a = "1".parse::<u32>().unwrap();
    let aa = 10;
    // 使用as关键字将类型强制转换
    let bb = aa as u32;

    unsafe {
        // 定义一个数组长度是4，每个元素的值是0，类型是u8
        let a = [0u8,0u8,0u8,0u8];
        // 将变量a内存地址的数据强制转换成u32类型（注意：使用transmute直接转换内存里面的数据，需要卸载unsafe代码块里面）
        let b:u32 = mem::transmute(a);
        println!("let.b={}",b);
    }
}

// 定义一个枚举类型
enum Planet {
    Mars,
    Earth,
}

enum IpAddr {
    IPv4(u8,u8,u8,u8),
    IPv6(u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8)
}

// 定义一个元组结构
struct Pp(i32,i32);

// 定义结构体
#[derive(Debug)]
struct User {
    name:String,
    age:i32,
}

// 测试枚举类型
fn testEnum() {
    let local = IpAddr::IPv4(127,0,0,1);
    //let local = IpAddr::IPv6(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    match local {
        IpAddr::IPv4(a,b,c,d) => {
            println!("a={},b={},c={},d={}",a,b,c,d);
        }
        _ =>{}
    }
}

// 测试结构体数据类型
fn testStruct() {
    let name = String::from("sadada");
    // 初始化结构体
    let user = User {
        name: name,
        age:2
    };
    // 初始化元组结构体
    let pp = Pp(10,10);
}


// 测试切片数据类型
fn section() {
    let array = [1,2,3,4,5];
    // 为数组创建切片(获取array的引用，拿到0到3的位置的值创建成一个切片)
    let slice = &array[0..3];
    // 切数组的所有元素
    let slice2 = &array[..];
    println!("slice 0={},length = {},切片是否为空={}",slice[0],slice.len(),slice.is_empty());
    println!("slice2={:#?}",slice2);
}

// 测试数组数据类型
fn test_array() {
    // 声明数组，元素的数组类型是u32，数组的长度是5
    let a1:[u32;5] = [1,2,3,4,5];
    let a2 = [0,2];
    // 声明数组默认初始化
    let array = ["11","222","333"];
    println!("a1={:#?}",a1);
    println!("array={:#?}",array);
    println!("a2.1={}",a2[1]);
    // 初始化一个数组。元素数据类型是u32,长度是5；后半截的意思是每个元素的值是0，生成5个
    let buffer:[u32;5] = [0;5];
}

// 测试元组数据类型
fn testTuple() {

    let a = 10;
    let b = 'A';
    let str = String::from("jiang");

    let myTuple = (a,b,str);

    let (c,d,s) = myTuple;
    println!("元组的值 c={},d={},s={}",c,d,s);

    let myTuple1 = (10,'b',"tian");
    println!("myTuple 0={},1={},2={}",myTuple1.0,myTuple1.1,myTuple1.2);
}

// 值溢出处理
// 使用debug模式编译运行，当值溢出时会报错，但是使用release模式编译运行时值溢出不会报错反而会被忽略掉
fn overflow(a: u32,b:u32) -> u32 {
    // 这样写当两个值的和溢出时，就会有问题
    //let sum = a + b;
    // 使用这种方式相加当值溢出时会有标识
    let(sum,is_overflow) = a.overflowing_add(b);
    if(is_overflow) {
        print!("两个数值的和已经溢出");
    }
    return sum;
}