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

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use serde_yaml::Value;

pub fn read_from_yaml(key: &str) -> Result<Option<Value>, io::Error> {
    let file_path = dir_bin();
    let expanded_path_string = Path::new(&file_path).expand_tilde().ok_or(io::Error::new(io::ErrorKind::NotFound, "Cannot expand tilde"))?;
    let expanded_path = Path::new(&expanded_path_string);
    
    if !expanded_path.exists() {
        File::create(&expanded_path)?.write_all(b"---\n")?;
    }
    
    let content = fs::read_to_string(&expanded_path)?;
    let yaml: Value = serde_yaml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(yaml.get(key).cloned())
}

pub fn write_to_yaml(key: &str, value: &Value) -> Result<(), io::Error> {
    let file_path = dir_bin();
    let expanded_path_string = Path::new(&file_path).expand_tilde().ok_or(io::Error::new(io::ErrorKind::NotFound, "Cannot expand tilde"))?;
    let expanded_path = Path::new(&expanded_path_string);

    if !expanded_path.exists() {
        File::create(&expanded_path)?.write_all(b"---\n")?;
    }
    
    let content = fs::read_to_string(&expanded_path)?;
    let mut yaml: Value = serde_yaml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    yaml[key] = value.clone();
    fs::write(&expanded_path, serde_yaml::to_string(&yaml).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)?;
    
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