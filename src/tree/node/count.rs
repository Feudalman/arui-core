/// 处理节点拥有文本行数相关问题
use std::io::Error;

// --------------------- 文件相关 ---------------------

/// 获取文件中文本行数
pub fn get_file_count(path: &str) -> Result<u64, Error> {
    Ok(std::fs::read_to_string(path)?.lines().count() as u64)
}

// --------------------- 单元测试 ---------------------

#[cfg(test)]
mod tests {
    use crate::tree::node::count::get_file_count;

    #[test]
    fn test_get_line_count_1() {
        let path = "./tests/examples/tree/node/count/get_file_count/1.txt";
        let count = get_file_count(path).unwrap();
        println!("{}", count);
        assert_eq!(count, 11);
    }

    #[test]
    fn test_get_line_count_2() {
        let path = "./tests/examples/tree/node/count/get_file_count/test.rs";
        let count = get_file_count(path).unwrap();
        println!("{}", count);
    }
}
