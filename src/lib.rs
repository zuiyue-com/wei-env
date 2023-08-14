pub fn home_dir() -> Result<String, Box<dyn std::error::Error>> {
    let home_dir = if cfg!(target_os = "windows") {
        format!("{}/AppData/Local/wei/", std::env::var("USERPROFILE")?)
    } else {
        format!("{}/.wei/", std::env::var("HOME")?)
    };
    Ok(home_dir)
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