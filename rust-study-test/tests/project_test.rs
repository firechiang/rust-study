/**
 * 项目测试（就是测试项目里面的代码）。注意：这些测试代码一定要写在 tests 文件夹下，这个是规范
 * 在项目根目录下执行 cargo test --test project_test 命令开始测试 project_test.rs 文件
 */

// 引入common模块
mod common;

#[test]
fn test_project () {
    assert!(true);
}

#[test]
fn test_common () {
    // 调用common模块里面的函数
    assert!(common::setup());
}