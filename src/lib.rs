//! # ARUI-CORE
//! 一个简单的库，用于分析指定目录下的文件树。
//!
//! ## 目标基础功能：
//! - [x] 生成指定目录下的文件树结构
//! - [x] 遍历文件树生成各节点的总结信息
//! - [ ] 自定义错误类型
//! - [ ] 提供配置字段，用于控制文件树生成的行为
//! - [ ] 提供更多总结信息的获取
#![allow(dead_code)]

/// 项目配置相关
pub use tree::config::ProjectConfig;
/// 项目树节点相关
pub use tree::node::TreeNode;
/// 项目树与公开 API
pub use tree::root::ProjectTree;
/// 项目树节点总结信息相关
pub use tree::summary::NodeSummary;
/// 项目树可视化
pub use tree::visible::ProjectTreeVisible;

pub mod errors;
pub mod tree;
pub mod utils;
