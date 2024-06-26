use std::process::{Command, exit};

pub fn copy_firmware(mount_dir: &str) {
    println!("Copying WiFi and Bluetooth firmware...");

    let wifi_firmware_repo = "https://github.com/RPi-Distro/firmware-nonfree";
    let bluetooth_firmware_repo = "https://github.com/RPi-Distro/bluez-firmware";
    let firmware_dir = format!("{}/lib/firmware", mount_dir);
    let brcm_firmware_dir = format!("{}/brcm", firmware_dir);

    let git_clone_wifi_output = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg(wifi_firmware_repo)
        .output()
        .expect("Failed to execute git clone for WiFi firmware");

    if !git_clone_wifi_output.status.success() {
        eprintln!("Failed to clone WiFi firmware repository: {}", String::from_utf8_lossy(&git_clone_wifi_output.stderr));
        exit(1);
    }

    std::fs::create_dir_all(&brcm_firmware_dir).expect("Failed to create brcm firmware directory");

    let cp_wifi_output = Command::new("cp")
        .arg("firmware-nonfree/debian/config/brcm80211/cypress/cyfmac43455-sdio-standard.bin")
        .arg(&brcm_firmware_dir)
        .output()
        .expect("Failed to copy WiFi firmware");

    if cp_wifi_output.status.success() {
        println!("WiFi firmware copied successfully.");
    } else {
        eprintln!("Failed to copy WiFi firmware: {}", String::from_utf8_lossy(&cp_wifi_output.stderr));
        exit(1);
    }

    let git_clone_bt_output = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg(bluetooth_firmware_repo)
        .output()
        .expect("Failed to execute git clone for Bluetooth firmware");

    if !git_clone_bt_output.status.success() {
        eprintln!("Failed to clone Bluetooth firmware repository: {}", String::from_utf8_lossy(&git_clone_bt_output.stderr));
        exit(1);
    }

    let cp_bt_output = Command::new("cp")
        .arg("bluez-firmware/debian/firmware/broadcom/BCM4345C0.hcd")
        .arg(&brcm_firmware_dir)
        .output()
        .expect("Failed to copy Bluetooth firmware");

    if cp_bt_output.status.success() {
        println!("Bluetooth firmware copied successfully.");
    } else {
        eprintln!("Failed to copy Bluetooth firmware: {}", String::from_utf8_lossy(&cp_bt_output.stderr));
        exit(1);
    }
}
