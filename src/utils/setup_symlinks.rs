use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

pub fn setup_firmware_symlinks(mount_dir: &str) {
    println!("Setting up firmware symlinks...");

    let firmware_dir = format!("{}/lib/firmware/brcm", mount_dir);

    // Create WiFi symlinks
    create_symlink(&firmware_dir, "brcmfmac43455-sdio.bin", "brcmfmac43455-sdio.raspberrypi,5-model-b.bin");
    create_symlink(&firmware_dir, "brcmfmac43455-sdio.clm_blob", "brcmfmac43455-sdio.raspberrypi,5-model-b.clm_blob");
    create_symlink(&firmware_dir, "brcmfmac43455-sdio.txt", "brcmfmac43455-sdio.raspberrypi,5-model-b.txt");

    // Create Bluetooth symlink
    create_symlink(&firmware_dir, "BCM4345C0.hcd", "BCM4345C0.raspberrypi,5-model-b.hcd");
}

fn create_symlink(firmware_dir: &str, target_file: &str, symlink_name: &str) {
    let target_path = format!("{}/{}", firmware_dir, target_file);
    let symlink_path = format!("{}/{}", firmware_dir, symlink_name);

    if Path::new(&symlink_path).exists() {
        if let Err(e) = fs::remove_file(&symlink_path) {
            eprintln!("Failed to remove existing symlink for {}: {}", symlink_name, e);
            return;
        }
    }

    if let Err(e) = symlink(&target_path, &symlink_path) {
        eprintln!("Failed to create symlink for {}: {}", symlink_name, e);
    } else {
        println!("Created symlink: {} -> {}", symlink_name, target_path);
    }
}
