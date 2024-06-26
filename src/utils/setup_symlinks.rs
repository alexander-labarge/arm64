use std::os::unix::fs::symlink;

pub fn setup_firmware_symlinks(mount_dir: &str) {
    println!("Setting up firmware symlinks...");

    let firmware_dir = format!("{}/lib/firmware/brcm", mount_dir);

    create_symlink(&firmware_dir, "brcmfmac43455-sdio.bin", "brcmfmac43455-sdio.raspberrypi,5-model-b.bin");
    create_symlink(&firmware_dir, "brcmfmac43455-sdio.clm_blob", "brcmfmac43455-sdio.raspberrypi,5-model-b.clm_blob");
    create_symlink(&firmware_dir, "brcmfmac43455-sdio.txt", "brcmfmac43455-sdio.raspberrypi,5-model-b.txt");
    create_symlink(&firmware_dir, "BCM4345C0.hcd", "BCM4345C0.raspberrypi,5-model-b.hcd");
}

fn create_symlink(firmware_dir: &str, target_file: &str, symlink_name: &str) {
    let target_path = format!("{}/{}", firmware_dir, target_file);
    let symlink_path = format!("{}/{}", firmware_dir, symlink_name);

    if let Err(e) = symlink(&target_path, &symlink_path) {
        eprintln!("Failed to create symlink for {}: {}", symlink_name, e);
    } else {
        println!("Created symlink: {} -> {}", symlink_name, target_path);
    }
}
