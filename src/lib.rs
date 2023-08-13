pub fn home_dir() -> Result<String, Box<dyn std::error::Error>> {
    let home_dir = if cfg!(target_os = "windows") {
        format!("{}/AppData/Local/wei/", std::env::var("USERPROFILE")?)
    } else {
        format!("{}/.wei/", std::env::var("HOME")?)
    };
    Ok(home_dir)
}

pub fn uuid_dir() -> String {
    format!("{}uuid.dat", home_dir().unwrap())
}

pub fn user_dir() -> String {
    format!("{}user.dat", home_dir().unwrap())
}

pub fn dir_bin() -> String {
    format!("{}bin.dat", home_dir().unwrap())
}

use serde_yaml::Value;
use std::fs;
use std::io::Error;
use std::path::Path;

fn read_from_yaml(key: &str) -> Result<Option<Value>, Error> {
    // 读取文件内容
    let content = fs::read_to_string(Path::new(&FILE_PATH).expand_tilde()?)?;
    let yaml: Value = serde_yaml::from_str(&content)?;

    // 获取指定键的值
    Ok(yaml.get(key).cloned())
}

fn write_to_yaml(key: &str, value: &Value) -> Result<(), Error> {
    // 读取现有的YAML文件内容
    let content = fs::read_to_string(Path::new(&FILE_PATH).expand_tilde()?)?;
    let mut yaml: Value = serde_yaml::from_str(&content)?;

    // 设置或更新指定的键
    yaml[key] = value.clone();

    // 将更新后的内容写回文件
    fs::write(Path::new(&FILE_PATH).expand_tilde()?, serde_yaml::to_string(&yaml)?)?;
    Ok(())
}

// 将 ~ 符号扩展到完整的用户家目录路径
trait ExpandTilde {
    fn expand_tilde(&self) -> Option<String>;
}

impl ExpandTilde for Path {
    fn expand_tilde(&self) -> Option<String> {
        if let Some(home_dir) = dirs::home_dir() {
            self.to_str().map(|p| p.replace("~", &home_dir.to_string_lossy()))
        } else {
            None
        }
    }
}

// fn main() {
//     // 示例使用
//     if let Ok(value) = read_from_yaml("some_key") {
//         println!("Read value: {:?}", value);
//     }

//     let new_value = Value::String("New Content".to_string());
//     if write_to_yaml("some_key", &new_value).is_ok() {
//         println!("Value written successfully!");
//     }
// }