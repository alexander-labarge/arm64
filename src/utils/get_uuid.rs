use std::process::Command;

pub fn get_uuid(device: &str) -> String {
    let output = Command::new("blkid")
        .arg("-s")
        .arg("UUID")
        .arg("-o")
        .arg("value")
        .arg(device)
        .output()
        .expect("Failed to execute blkid");

    if output.status.success() {
        let uuid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        uuid
    } else {
        panic!("Failed to get UUID for device: {}", device);
    }
}
