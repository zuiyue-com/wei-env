pub fn home_dir() -> Result<String, Box<dyn std::error::Error>> {
    let home_dir = if cfg!(target_os = "windows") {
        std::env::var("USERPROFILE")?
    } else {
        std::env::var("HOME")?
    };
    Ok(home_dir)
}

pub fn uuid_dir() -> String {
    let home_dir = home_dir().unwrap();
    let file_path;

    if cfg!(target_os = "windows") {
        file_path = format!("{}/AppData/Local/Ai/uuid.dat", home_dir);
    } else {
        file_path = format!("{}/uuid.dat", home_dir);
    }
    file_path
}

pub fn user_dir() -> String {
    let home_dir = home_dir().unwrap();
    let file_path;

    if cfg!(target_os = "windows") {
        file_path = format!("{}/AppData/Local/Ai/user.dat", home_dir);
    } else {
        file_path = format!("{}/user.dat", home_dir);
    }
    file_path
}