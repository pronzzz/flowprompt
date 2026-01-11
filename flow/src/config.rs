use std::path::PathBuf;
use std::fs;

pub fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().expect("Could not determine config directory");
    path.push("flowprompt");
    
    if !path.exists() {
        fs::create_dir_all(&path).expect("Could not create config directory");
    }
    
    path
}
