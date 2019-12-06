// 引入当前目录的Dept模块，它定义在mod.rs文件里面（注意：在非main函数和mod.rs文件里面，引入当前目录模块的方式是从当前模块的上级引入并且使用use关键字）
use super::Dept;
// 引入顶级目录下的test模块（crate关键字代表根目录）（注意：引入上级模块使用use关键字）
use crate::test;

/**
 * 定义模块
 */
pub mod DeptService {
    // 引入函数（注意：crate关键字表示根目录）
    use crate::com::dept::dept_name;
    use crate::com::dept::dept_entity::DeptEntity::getDeptEntity;
    use crate::com::dept::Dept::getDept;
    //use crate::com::dept;     引入到
    //use crate::com::dept::*; 全部引入

    pub fn getDeptName () -> String {
        dept_name();
        getDeptEntity();
        getDept();
        return String::from("毛毛");
    }

}