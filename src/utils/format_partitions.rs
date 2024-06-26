use std::process::{Command, exit};

pub fn format_partitions(target_drive: &str) {
    println!("Formatting partitions on {}", target_drive);

    let partition_suffix = if target_drive.contains("nvme") || target_drive.contains("mmcblk") {
        "p"
    } else {
        ""
    };

    let boot_partition = format!("{}{}1", target_drive, partition_suffix);
    let swap_partition = format!("{}{}2", target_drive, partition_suffix);
    let root_partition = format!("{}{}3", target_drive, partition_suffix);

    let mkfs_vfat_output = Command::new("mkfs.vfat")
        .arg(&boot_partition)
        .output()
        .expect("Failed to execute mkfs.vfat");

    if mkfs_vfat_output.status.success() {
        println!("Boot partition formatted successfully.");
    } else {
        eprintln!("Failed to format boot partition: {}", String::from_utf8_lossy(&mkfs_vfat_output.stderr));
        exit(1);
    }

    let mkswap_output = Command::new("mkswap")
        .arg("--pagesize")
        .arg("16384")
        .arg(&swap_partition)
        .output()
        .expect("Failed to execute mkswap");

    if mkswap_output.status.success() {
        println!("Swap partition formatted successfully.");
    } else {
        eprintln!("Failed to format swap partition: {}", String::from_utf8_lossy(&mkswap_output.stderr));
        exit(1);
    }

    let mkfs_ext4_output = Command::new("mkfs.ext4")
        .arg("-F")
        .arg(&root_partition)
        .output()
        .expect("Failed to execute mkfs.ext4");

    if mkfs_ext4_output.status.success() {
        println!("Root partition formatted successfully.");
    } else {
        eprintln!("Failed to format root partition: {}", String::from_utf8_lossy(&mkfs_ext4_output.stderr));
        exit(1);
    }
}
