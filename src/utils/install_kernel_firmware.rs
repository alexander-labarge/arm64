use std::process::{Command, exit};
use std::fs;

pub fn install_kernel_firmware(mount_dir: &str, target_drive: &str) {
    println!("Installing kernel and firmware...");

    let partition_suffix = if target_drive.contains("nvme") || target_drive.contains("mmcblk") {
        "p"
    } else {
        ""
    };

    let boot_partition = format!("{}{}1", target_drive, partition_suffix);
    let firmware_repo = "https://github.com/raspberrypi/firmware";
    let firmware_dir = format!("{}/firmware", mount_dir);
    let boot_dir = format!("{}/boot", mount_dir);

    // Create the /boot directory within the mount directory if it doesn't exist
    let mkdir_boot_output = Command::new("mkdir")
        .arg("-p")
        .arg(&boot_dir)
        .output()
        .expect("Failed to create /boot directory");

    if !mkdir_boot_output.status.success() {
        eprintln!("Failed to create /boot directory: {}", String::from_utf8_lossy(&mkdir_boot_output.stderr));
        exit(1);
    }

    // Check if the boot partition is already mounted
    let verify_mount = Command::new("findmnt")
        .arg("-n")
        .arg(&boot_dir)
        .output()
        .expect("Failed to execute findmnt command");

    if verify_mount.status.success() && !verify_mount.stdout.is_empty() {
        println!("Boot partition is already mounted.");
    } else {
        // Attempt to mount the boot partition if it is not mounted
        let mount_output = Command::new("mount")
            .arg(&boot_partition)
            .arg(&boot_dir)
            .output()
            .expect("Failed to execute mount command");

        if !mount_output.status.success() {
            eprintln!("Failed to mount the boot partition: {}", String::from_utf8_lossy(&mount_output.stderr));
            exit(1);
        } else {
            println!("Boot partition mounted successfully.");
        }
    }

    // Clone the firmware repository to the root of the mount directory
    let git_clone_output = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg(firmware_repo)
        .arg(&firmware_dir)
        .output()
        .expect("Failed to execute git clone");

    if git_clone_output.status.success() {
        println!("Firmware repository cloned successfully.");
    } else {
        eprintln!("Failed to clone firmware repository: {}", String::from_utf8_lossy(&git_clone_output.stderr));
        exit(1);
    }

    // Verify the cloned directory exists and has the expected structure
    let cloned_boot_dir = format!("{}/boot", firmware_dir);
    if !fs::metadata(&cloned_boot_dir).is_ok() {
        eprintln!("Cloned firmware directory does not contain the expected 'boot' subdirectory.");
        exit(1);
    }

    // Copy the necessary firmware files to /boot
    let firmware_files = [
        "bcm2712-rpi-5-b.dtb",
        "fixup_cd.dat",
        "fixup.dat",
        "start_cd.elf",
        "start.elf",
        "bootcode.bin",
        "kernel8.img"
    ];

    for file in &firmware_files {
        let src = format!("{}/{}", cloned_boot_dir, file);
        let dst = format!("{}/{}", boot_dir, file);
        let cp_output = Command::new("cp")
            .arg(&src)
            .arg(&dst)
            .output()
            .expect("Failed to execute cp");

        if cp_output.status.success() {
            println!("Copied {} successfully.", file);
        } else {
            eprintln!("Failed to copy {}: {}", file, String::from_utf8_lossy(&cp_output.stderr));
            exit(1);
        }
    }

    // Copy overlays directory
    let cp_overlays_output = Command::new("cp")
        .arg("-r")
        .arg(format!("{}/overlays", cloned_boot_dir))
        .arg(boot_dir)
        .output()
        .expect("Failed to execute cp for overlays");

    if cp_overlays_output.status.success() {
        println!("Overlay files copied successfully.");
    } else {
        eprintln!("Failed to copy overlay files: {}", String::from_utf8_lossy(&cp_overlays_output.stderr));
        exit(1);
    }

    // Copy kernel modules
    let cp_modules_output = Command::new("cp")
        .arg("-r")
        .arg(format!("{}/modules", firmware_dir))
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
