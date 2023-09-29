use std::env;

pub fn home_dir() -> Result<String, Box<dyn std::error::Error>> {
    let home_dir = if cfg!(target_os = "windows") {
        format!("{}/AppData/Local/wei/", std::env::var("USERPROFILE")?)
    } else {
        format!("{}/.wei/", std::env::var("HOME")?)
    };
    fs::create_dir_all(home_dir.clone())?;
    Ok(home_dir)
}

pub fn dir_daemon() -> String {
    let dir = format!("{}daemon.dat", home_dir().unwrap());
    let path = Path::new(&dir);
    if path.exists() {
        return path.display().to_string();
    } 

    "./daemon.dat".to_string()
}

/// 读取当前状态
pub fn status() -> String {
    // 如果文件不存在，则创建文件，写入1
    let path_status = dir_status();
    let path = Path::new(&path_status);
    if !path.exists() {
        write(&path_status, "status", "1").unwrap();
    }
    read(&path_status, "status").unwrap()
}

pub fn task_start() {
    write(&task_path(), "task", "1").unwrap();
}

pub fn task_stop() {
    write(&task_path(), "task", "0").unwrap();
}

pub fn task_status() -> String {
    // 如果文件不存在，则创建文件，写入1
    let path_status = task_path();
    let path = Path::new(&path_status);
    if !path.exists() {
        write(&path_status, "status", "1").unwrap();
    }
    read(&path_status, "status").unwrap()
}

pub fn task_path() -> String {
    format!("{}task.dat", home_dir().unwrap())
}

/// 开启所有进程
pub fn start() {
    write(&dir_status(), "status", "1").unwrap();
}

/// 关闭所有进程
pub fn stop() {
    write(&dir_status(), "status", "0").unwrap();
}

pub fn dir_status() -> String {
    format!("{}status.dat", home_dir().unwrap())
}

pub fn dir_uuid() -> String {
    format!("{}uuid.dat", home_dir().unwrap())
}

pub fn dir_user() -> String {
    format!("{}user.dat", home_dir().unwrap())
}

pub fn dir_bin() -> String {
    format!("{}bin.dat", home_dir().unwrap())
}

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use serde_yaml::Value;

// 读取 yaml 文件里面的key值 转成string
pub fn read(dir: &str, key: &str) -> Result<String, io::Error> {
    let expanded_path = Path::new(dir);
    
    // Ensure the parent directory exists
    if let Some(parent) = expanded_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(&parent)?;
        }
    }

    if !expanded_path.exists() {
        File::create(&expanded_path)?.write_all(b"---\n")?;
    }
    
    let content = fs::read_to_string(&expanded_path)?;
    let yaml: Value = serde_yaml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    Ok(yaml.get(key).and_then(Value::as_str).unwrap_or("").to_string())
}

// 读取 yaml 文件里面的key值 
pub fn read_value(dir: &str, key: &str) -> Result<Option<Value>, io::Error> {
    let expanded_path = Path::new(dir);
    
    // Ensure the parent directory exists
    if let Some(parent) = expanded_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(&parent)?;
        }
    }

    if !expanded_path.exists() {
        File::create(&expanded_path)?.write_all(b"---\n")?;
    }
    
    let content = fs::read_to_string(&expanded_path)?;
    let yaml: Value = serde_yaml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    Ok(yaml.get(key).cloned())
}

// 使用 yaml 写入文件
pub fn write(file_path: &str, key: &str, value: &str) -> Result<(), io::Error> {
    let expanded_path = Path::new(file_path);

    // Ensure the parent directory exists
    if let Some(parent) = expanded_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(&parent)?;
        }
    }

    if !expanded_path.exists() {
        File::create(&expanded_path)?.write_all(b"---\n")?;
    }
    
    let content = fs::read_to_string(&expanded_path)?;
    let mut yaml: Value = serde_yaml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Set the value for the given key
    yaml[key] = Value::String(value.to_string());

    fs::write(&expanded_path, serde_yaml::to_string(&yaml).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)?;

    Ok(())
}

// 初始化执行文件的路径，方便其它执行文件调用
pub fn bin_init(name: &str) {
    write(&dir_bin(), name, &env::current_exe().unwrap().display().to_string()).unwrap();
}
