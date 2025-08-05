//! # 项目树模块
//! 该模块为 `ARUI-CORE` 的核心，负责根据配置生成项目树、获取节点总结信息。
//! 而在该模块中，`root` 模块为项目树入口，`node` 模块为树节点，`summary` 模块为总结信息。
pub mod config;
pub mod node;
pub mod root;
pub mod summary;
pub mod visible;
