//! # 项目树可视化实现
//! 该模块主要针对树结构实现可视化接口，而不额外封装任何导出结构
use crate::tree::node::TreeNode;
use crate::tree::root::ProjectTree;
use std::fmt::{Display, Formatter};

// --------------------- visible trait ---------------------

pub trait ProjectTreeVisible {
    /// 列出项目根节点信息：
    /// ID： 项目根节点的唯一标识
    /// Valid：是否为有效树，即树结构存在于目标路径下
    /// Name： 项目名
    /// Path： 项目根路径
    fn show(&self);
    /// 打印整个树结构
    fn print_tree(&self);
    /// 递归打印节点
    fn print_node(node: &TreeNode, depth: usize);
}

// --------------------- ProjectTreeVisible ---------------------

impl ProjectTreeVisible for ProjectTree {
    fn show(&self) {
        println!("Project Tree:");
        println!("ID: {}", self.id);
        println!("Valid: {}", self.is_valid());
        println!("Name: {}", self.name);
        println!("Path: {}", self.path);
    }

    fn print_tree(&self) {
        if let Some(ref root) = self.root {
            Self::print_node(root, 0);
        } else {
            println!("Tree is empty");
        }
    }

    fn print_node(node: &TreeNode, depth: usize) {
        // 根据深度缩进
        let indent = "  ".repeat(depth);

        // 打印当前节点信息
        let node_type = if node.is_dir { "DIR" } else { "FILE" };
        println!("{}- {} [{}]", indent, node.path, node_type);

        // 递归打印子节点
        if let Some(ref children) = node.children {
            for child in children {
                Self::print_node(child, depth + 1);
            }
        }
    }
}

impl Display for ProjectTree {
    // TODO：优化一下
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProjectTree {{\n\tID: {},\n\tValid: {},\n\tName: {},\n\tPath: {}\n}}",
            self.id,
            self.is_valid(),
            self.name,
            self.path
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::root::ProjectTree;
    use crate::tree::visible::ProjectTreeVisible;

    #[test]
    fn test_show_project() {
        let name = "test".to_string();
        let path = ".".to_string();
        let tree = ProjectTree::new(name, path, None);
        tree.show();
        print!("{:}", tree);
    }

    #[test]
    fn test_show_project_tree() {
        let name = "test".to_string();
        let path = "./src".to_string();
        let mut tree = ProjectTree::new(name, path, None);
        tree.build().expect("panic");
        // 顺便打印一下结构看看
        tree.print_tree();
    }
}
