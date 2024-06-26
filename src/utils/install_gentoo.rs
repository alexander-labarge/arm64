use std::process::{Command, exit};

pub fn install_gentoo(stage3_url: &str, mount_dir: &str, target_drive: &str) {
    println!("Installing Gentoo base system...");

    let partition_suffix = if target_drive.contains("nvme") || target_drive.contains("mmcblk") {
        "p"
    } else {
        ""
    };

    let root_partition = format!("{}{}3", target_drive, partition_suffix);

    // Create the mount directory if it doesn't exist
    let mkdir_output = Command::new("mkdir")
        .arg("-p")
        .arg(mount_dir)
        .output()
        .expect("Failed to create mount directory");

    if !mkdir_output.status.success() {
        eprintln!("Failed to create mount directory: {}", String::from_utf8_lossy(&mkdir_output.stderr));
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

    // Download the Stage3 tarball
    let wget_output = Command::new("wget")
        .arg(stage3_url)
        .arg("-O")
        .arg("stage3.tar.xz")
        .current_dir(mount_dir)
        .output()
        .expect("Failed to execute wget");

    if wget_output.status.success() {
        println!("Stage3 tarball downloaded successfully.");
    } else {
        eprintln!("Failed to download stage3 tarball: {}", String::from_utf8_lossy(&wget_output.stderr));
        exit(1);
    }

    // Extract the Stage3 tarball
    let tar_output = Command::new("tar")
        .arg("xpf")
        .arg("stage3.tar.xz")
        .arg("--xattrs-include='*.*'")
        .arg("--numeric-owner")
        .current_dir(mount_dir)
        .output()
        .expect("Failed to execute tar");

    if tar_output.status.success() {
        println!("Stage3 tarball extracted successfully.");
    } else {
        eprintln!("Failed to extract stage3 tarball: {}", String::from_utf8_lossy(&tar_output.stderr));
        exit(1);
    }

    println!("Gentoo base system installed successfully.");
}
