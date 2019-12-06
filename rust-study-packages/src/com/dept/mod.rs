/**
 * Rust的多模块，最后都会聚合到一个模块里面。
 * 所以如果是多模块的话，每一级目录里面必须有一个mod.rs文件，用来引入当前目录的模块
 * 注意：当前目录的模块使用文件名引入，mod.rs文件也是可以直接写逻辑代码（不推荐在mod.rs里面写逻辑代码）
 */
// 引入当前目录dept_service.rs文件里面的模块
pub mod dept_service;
pub mod dept_entity;
/**
 * 定义模块（推荐在文件里面都定义一个顶级模块，再在顶级模块里面写函数）
 */
pub mod Dept {

    pub fn getDept() -> String {
        return String::from("Dept");
    }
}
/**
 * 直接定义函数
 */
pub fn dept_name () {
    println!("调用了dept模块里面的dept_name函数")
}