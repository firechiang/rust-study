// 引入当前目录的模块（注意：当前目录的模块是直接使用文件名引入）
pub mod test;
// 引入当前目录的模块(注意：这个模块里面还包含有其它模块)
pub mod com;
// 只能使用枚举类型里面的2个类型
use King::{One,Two};
// 全部诉可以使用
//use King::*;

/**
 * 引入内部包简单使用
 */
fn main() {
    // 调用test模块里面的函数
    test::test_test();
    let deptName = com::Dept::getDept();
    println!("deptName={}",deptName);

    let deptEntity = com::dept_entity::DeptEntity::getDeptEntity();
    println!("deptEntity={}",deptEntity);

    let comDao = com::com_dao::ComDao::getComDao();
    println!("comDao={}",comDao);

    // 限制枚举的使用
    let one = One;
    let two = Two;
    //let three = Three; 这个不能直接使用
    println!("one={:?},two={:?}",one,two);

}

#[derive(Debug)]
enum King {
    One,
    Two,
    Three
}
