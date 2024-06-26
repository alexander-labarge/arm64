use std::process::{Command, exit};

pub fn install_gentoo(stage3_url: &str, mount_dir: &str, target_drive: &str) {
    println!("Installing Gentoo base system...");

    let partition_suffix = if target_drive.contains("nvme") || target_drive.contains("mmcblk") {
        "p"
    } else {
        ""
    };

    let root_partition = format!("{}{}3", target_drive, partition_suffix);

    let mount_output = Command::new("mount")
        .arg(&root_partition)
        .arg(mount_dir)
        .output()
        .expect("Failed to execute mount");

    if !mount_output.status.success() {
        eprintln!("Failed to mount root partition: {}", String::from_utf8_lossy(&mount_output.stderr));
        exit(1);
    }

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
}
