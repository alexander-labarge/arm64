use std::process::{Command, exit};

pub fn install_kernel_firmware(mount_dir: &str, target_drive: &str) {
    println!("Installing kernel and firmware...");

    let partition_suffix = if target_drive.contains("nvme") || target_drive.contains("mmcblk") {
        "p"
    } else {
        ""
    };

    let boot_partition = format!("{}{}1", target_drive, partition_suffix);
    let firmware_repo = "https://github.com/raspberrypi/firmware";
    let boot_dir = format!("{}/boot", mount_dir);

    let git_clone_output = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg(firmware_repo)
        .output()
        .expect("Failed to execute git clone");

    if git_clone_output.status.success() {
        println!("Firmware repository cloned successfully.");
    } else {
        eprintln!("Failed to clone firmware repository: {}", String::from_utf8_lossy(&git_clone_output.stderr));
        exit(1);
    }

    let mount_output = Command::new("mount")
        .arg(&boot_partition)
        .arg(&boot_dir)
        .output()
        .expect("Failed to execute mount");

    if !mount_output.status.success() {
        eprintln!("Failed to mount boot partition: {}", String::from_utf8_lossy(&mount_output.stderr));
        exit(1);
    }

    let cp_output = Command::new("cp")
        .arg("firmware/boot/{bcm2712-rpi-5-b.dtb,fixup_cd.dat,fixup.dat,start_cd.elf,start.elf,bootcode.bin,kernel8.img}")
        .arg(&boot_dir)
        .output()
        .expect("Failed to execute cp");

    if cp_output.status.success() {
        println!("Firmware files copied successfully.");
    } else {
        eprintln!("Failed to copy firmware files: {}", String::from_utf8_lossy(&cp_output.stderr));
        exit(1);
    }

    let cp_overlay_output = Command::new("cp")
        .arg("-r")
        .arg("firmware/boot/overlays")
        .arg(&boot_dir)
        .output()
        .expect("Failed to execute cp for overlays");

    if cp_overlay_output.status.success() {
        println!("Overlay files copied successfully.");
    } else {
        eprintln!("Failed to copy overlay files: {}", String::from_utf8_lossy(&cp_overlay_output.stderr));
        exit(1);
    }

    let cp_modules_output = Command::new("cp")
        .arg("-r")
        .arg("firmware/modules")
        .arg(format!("{}/lib/", mount_dir))
        .output()
        .expect("Failed to execute cp for modules");

    if cp_modules_output.status.success() {
        println!("Kernel modules copied successfully.");
    } else {
        eprintln!("Failed to copy kernel modules: {}", String::from_utf8_lossy(&cp_modules_output.stderr));
        exit(1);
    }
}
