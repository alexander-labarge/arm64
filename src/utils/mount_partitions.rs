use std::process::{Command, exit};

pub fn mount_partitions(mount_dir: &str, target_drive: &str) {
    let partition_suffix = if target_drive.contains("nvme") || target_drive.contains("mmcblk") {
        "p"
    } else {
        ""
    };

    let root_partition = format!("{}{}3", target_drive, partition_suffix);
    let boot_partition = format!("{}{}1", target_drive, partition_suffix);
    let boot_dir_path = format!("{}/boot", mount_dir);

    // Create the /boot directory within the mount directory
    let mkdir_boot_output = Command::new("mkdir")
        .arg("-p")
        .arg(&boot_dir_path)
        .output()
        .expect("Failed to create /boot directory");

    if !mkdir_boot_output.status.success() {
        eprintln!("Failed to create /boot directory: {}", String::from_utf8_lossy(&mkdir_boot_output.stderr));
        exit(1);
    }

    // Mount the root partition
    let mount_output = Command::new("mount")
        .arg(&root_partition)
        .arg(mount_dir)
        .output()
        .expect("Failed to execute mount");

    if !mount_output.status.success() {
        eprintln!("Failed to mount root partition: {}", String::from_utf8_lossy(&mount_output.stderr));
        exit(1);
    }

    // Mount the boot partition
    let mount_boot_output = Command::new("mount")
        .arg(&boot_partition)
        .arg(&boot_dir_path)
        .output()
        .expect("Failed to execute mount");

    if !mount_boot_output.status.success() {
        eprintln!("Failed to mount boot partition: {}", String::from_utf8_lossy(&mount_boot_output.stderr));
        exit(1);
    }
}
