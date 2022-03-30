use std::fmt::write;

/**
 * 泛型简单使用
 */
fn main() {
    let res = largest(10,10);
    println!("res={}",res);
    let p = Print {
        x:10,
        y:20,
        z:10,
    };
    println!("p={:#?}",p);
}

// 结构体中使用泛型
// 注意：Debug这个注解实在编译期间完成的，也就是在编译期间为该结构体里面的每个属性实现std::fmt::Display接口
// 注意：PartialEq这个注解实在编译期间完成的，也就是在编译期间为该结构体里面的每个属性实现std::cmp::PartialEq接口,这样属性就可以使用 == 符号进行比较，否则将结构体里面的属性将无法使用 == 符号进行比较
// 注意：Default这个注解实在编译期间完成的，结构体就可以使用 Print::default() 初始化
#[derive(Debug,PartialEq,Default)]
struct Print<T,U> {
    x: T,
    y: T,
    z: U,
}

impl<T: std::fmt::Display,U> std::fmt::Display for Print<T,U> {

    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({},{})",self.x,self.y)
    }
}

// 给泛型结构体实现函数（给泛型T指定两个类型，注意：多个类型用加好分割）
impl<T: std::cmp::PartialOrd + Clone,U> Print<T,U> {

    fn largest(&self) -> T {
        if self.x > self.y {
            self.x.clone()
        }else{
            self.y.clone()
        }
    }

}

// 直接给泛型执行类型
impl Print<u32,u32> {

}

fn show<T: std::fmt::Display>(a: T) {
    println!("show {}",a);
}
// 给a指定一个大范围的类型，只要是实现某个接口的类型就可以
fn show1(a: impl std::fmt::Display) {
    println!("show {}",a);
}

// 接口指定泛型
trait UserService {

}

// T 指定a，b参数和返回值的类型
fn largest<T:std::cmp::PartialOrd>(a: T,b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
