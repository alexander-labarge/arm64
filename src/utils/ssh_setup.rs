// use std::fs::{self, OpenOptions};
// use std::io::Write;
// use std::path::Path;
// use std::process::Command;

// pub fn add_ssh_key(mount_dir: &str, username: &str, ssh_key: &str) -> Result<(), String> {
//     let ssh_dir = format!("{}/home/{}/.ssh", mount_dir, username);
//     let authorized_keys_path = format!("{}/authorized_keys", ssh_dir);

//     // Ensure the SSH directory exists
//     if !Path::new(&ssh_dir).exists() {
//         fs::create_dir_all(&ssh_dir).map_err(|e| format!("Failed to create SSH directory: {}", e))?;
//     }

//     // Append the SSH key to authorized_keys
//     let mut file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open(&authorized_keys_path)
//         .map_err(|e| format!("Failed to open authorized_keys: {}", e))?;
    
//     writeln!(file, "{}", ssh_key)
//         .map_err(|e| format!("Failed to write SSH key to authorized_keys: {}", e))?;

//     // Set permissions
//     Command::new("chmod")
//         .arg("600")
//         .arg(&authorized_keys_path)
//         .output()
//         .map_err(|e| format!("Failed to set permissions for authorized_keys: {}", e))?;

//     Command::new("chown")
//         .arg("-R")
//         .arg(format!("{}:{}", username, username))
//         .arg(format!("{}/home/{}", mount_dir, username))
//         .output()
//         .map_err(|e| format!("Failed to set ownership for .ssh directory: {}", e))?;

//     Ok(())
// }
