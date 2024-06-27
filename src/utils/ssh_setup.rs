use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub struct SshKeyPair {
    pub public_key: String,
    pub private_key: String,
}

pub fn generate_ssh_key_pair() -> Result<SshKeyPair, String> {
    let ssh_dir = "/tmp/pi_installer_ssh";
    let private_key_path = format!("{}/id_rsa", ssh_dir);
    let public_key_path = format!("{}/id_rsa.pub", ssh_dir);

    // Ensure the SSH directory exists
    if !Path::new(ssh_dir).exists() {
        fs::create_dir_all(ssh_dir).map_err(|e| format!("Failed to create SSH directory: {}", e))?;
    }

    // Generate the SSH key pair
    let output = Command::new("ssh-keygen")
        .arg("-t")
        .arg("rsa")
        .arg("-b")
        .arg("2048")
        .arg("-f")
        .arg(&private_key_path)
        .arg("-N")
        .arg("")
        .output()
        .map_err(|e| format!("Failed to generate SSH key pair: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to generate SSH key pair: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Read the public key
    let public_key = fs::read_to_string(&public_key_path)
        .map_err(|e| format!("Failed to read public key: {}", e))?;

    // Read the private key
    let private_key = fs::read_to_string(&private_key_path)
        .map_err(|e| format!("Failed to read private key: {}", e))?;

    Ok(SshKeyPair { public_key, private_key })
}

pub fn add_ssh_key(mount_dir: &str, username: &str, ssh_key: &str) -> Result<(), String> {
    let ssh_dir = format!("{}/home/{}/.ssh", mount_dir, username);
    let authorized_keys_path = format!("{}/authorized_keys", ssh_dir);

    // Ensure the SSH directory exists
    if !Path::new(&ssh_dir).exists() {
        fs::create_dir_all(&ssh_dir).map_err(|e| format!("Failed to create SSH directory: {}", e))?;
    }

    // Append the SSH key to authorized_keys
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&authorized_keys_path)
        .map_err(|e| format!("Failed to open authorized_keys: {}", e))?;
    
    writeln!(file, "{}", ssh_key)
        .map_err(|e| format!("Failed to write SSH key to authorized_keys: {}", e))?;

    // Set permissions
    Command::new("chmod")
        .arg("600")
        .arg(&authorized_keys_path)
        .output()
        .map_err(|e| format!("Failed to set permissions for authorized_keys: {}", e))?;

    Command::new("chown")
        .arg("-R")
        .arg(format!("{}:{}", username, username))
        .arg(format!("{}/home/{}", mount_dir, username))
        .output()
        .map_err(|e| format!("Failed to set ownership for .ssh directory: {}", e))?;

    Ok(())
}
