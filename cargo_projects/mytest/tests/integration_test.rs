//集成测试，需要在项目根目录创建一个 tests 目录，与 src 同级,Cargo 会将每一个文件当作单独的 crate 来编译
//也是用cargo test 运行，有三部分：单元测试、集成测试和文档测试
//cargo test --test integration_test ，只集成测试，参数是文件名
use mytest;//项目名，tomal文件中的name的值

#[test]
fn it_adds_two() {
    assert_eq!(4, mytest::add_two(2));
}
 
mod common;  //模块声明
#[test]
fn it_adds_two2() {
    common::setup();
    assert_eq!(4, mytest::add_two(2));
}
//cargo test --test integration_test  -- --nocapture