use std::process::{Command, exit};

pub fn copy_firmware(mount_dir: &str) {
    println!("Copying WiFi and Bluetooth firmware...");

    let wifi_firmware_repo = "https://github.com/RPi-Distro/firmware-nonfree";
    let bluetooth_firmware_repo = "https://github.com/RPi-Distro/bluez-firmware";
    let firmware_dir = format!("{}/lib/firmware", mount_dir);
    let brcm_firmware_dir = format!("{}/brcm", firmware_dir);

    // Clone WiFi firmware repository
    let wifi_firmware_clone_dir = format!("{}/firmware-nonfree", mount_dir);
    let git_clone_wifi_output = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg(wifi_firmware_repo)
        .arg(&wifi_firmware_clone_dir)
        .output()
        .expect("Failed to execute git clone for WiFi firmware");

    if !git_clone_wifi_output.status.success() {
        eprintln!("Failed to clone WiFi firmware repository: {}", String::from_utf8_lossy(&git_clone_wifi_output.stderr));
        exit(1);
    }

    std::fs::create_dir_all(&brcm_firmware_dir).expect("Failed to create brcm firmware directory");

    let cp_wifi_firmware_files = [
        "debian/config/brcm80211/cypress/cyfmac43455-sdio-standard.bin",
        "debian/config/brcm80211/cypress/cyfmac43455-sdio.clm_blob",
        "debian/config/brcm80211/brcm/brcmfmac43455-sdio.txt"
    ];

    for file in &cp_wifi_firmware_files {
        let src = format!("{}/{}", wifi_firmware_clone_dir, file);
        let cp_wifi_output = Command::new("cp")
            .arg(&src)
            .arg(&brcm_firmware_dir)
            .output()
            .expect("Failed to copy WiFi firmware");

        if cp_wifi_output.status.success() {
            println!("WiFi firmware copied successfully: {}", src);
        } else {
            eprintln!("Failed to copy WiFi firmware: {}", String::from_utf8_lossy(&cp_wifi_output.stderr));
            exit(1);
        }
    }

    // Clone Bluetooth firmware repository
    let bluetooth_firmware_clone_dir = format!("{}/bluez-firmware", mount_dir);
    let git_clone_bt_output = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg(bluetooth_firmware_repo)
        .arg(&bluetooth_firmware_clone_dir)
        .output()
        .expect("Failed to execute git clone for Bluetooth firmware");

    if !git_clone_bt_output.status.success() {
        eprintln!("Failed to clone Bluetooth firmware repository: {}", String::from_utf8_lossy(&git_clone_bt_output.stderr));
        exit(1);
    }

    let cp_bt_output = Command::new("cp")
        .arg(format!("{}/debian/firmware/broadcom/BCM4345C0.hcd", bluetooth_firmware_clone_dir))
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
