//! 测试 `tree` 模块的基础使用
//! 并不包含错误、配置等测试，因为此处仅测试 `tree` 模块的基础 API 在正常情况下的行为
use arui_core::tree::{root::ProjectTree, visible::ProjectTreeVisible};

#[test]
/// 测试创建项目，并分步构建项目树、获取总结信息
fn test_build_projet_tree() {
    let name = "test";
    let path = "./tests/examples";
    let mut tree = ProjectTree::new(name, path, None);
    assert_eq!(tree.name, "test".to_string());
    assert_eq!(tree.path, path.to_string());

    tree.build().expect("build failed");
    tree.summarize().expect("summarize failed");

    println!("-----------------");
    tree.show();
    println!("-----------------");
    tree.print_tree();
    println!("-----------------");
}

#[test]
fn test_plant_project_tree() {
    let name = "test";
    let path = "./tests";
    let tree = ProjectTree::plant(name, path, None);
    println!("-----------------");
    tree.show();
    println!("-----------------");
    tree.print_tree();
    println!("-----------------");
}
