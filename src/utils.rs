use std::path::Path;
use uuid::Uuid;
use crate::errors::IOError;

/// 生成随机 id
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// 检查路径是否合法
/// 接受一个路径参数，可以是绝对路径也可以是一个相对路径
/// 若路径不存在则抛出错误 InvalidPath，存在则返回当前路径的绝对路径
pub fn check_path<S>(target: S) -> Result<String, IOError>
where
    S: Into<String>,
{
    let s = target.into();
    let path = Path::new(&s);
    if !path.exists() {
        Err(IOError::InvalidPath(s))
    } else {
        Ok(path.canonicalize()?.to_string_lossy().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id = generate_id();
        assert_eq!(id.len(), 36);
    }

    #[test]
    fn test_check_path() {
        let valid_path = check_path("./src".to_string());
        assert!(valid_path.is_ok());
        // 获取当前启动命令时的绝对路径
        let current_path = std::env::current_dir().unwrap().to_string_lossy().to_string();
        // current_path 拼接上 src 应该和 valid_path 相同
        assert_eq!(current_path + "/src", valid_path.unwrap());
        let invalid_path = check_path("/not_exist".to_string());
        assert!(invalid_path.is_err());
    }
}