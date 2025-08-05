//! # 节点文件特征相关
//! - `get_file_size` 获取文件占用磁盘的大小
use std::io::Error;

// --------------------- 文件相关 ---------------------
// TODO: 当前文件和目录大小计算分开，可以优化为先计算文件大小再计算目录大小

/// 获取文件占用磁盘大小
/// TODO: 错误处理
pub fn get_file_size(path: &str) -> Result<u64, Error> {
    Ok(std::fs::metadata(path)?.len())
}

// --------------------- 单元测试 ---------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 测试获取文件大小
    fn test_get_file_size_1() {
        let path = "./tests/examples/tree/node/file/test_get_file_size.txt";
        let size = get_file_size(path).unwrap();
        println!("{}", size);
        assert!(size > 0);
    }
}
