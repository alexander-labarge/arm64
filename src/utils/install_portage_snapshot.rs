use std::process::{Command, exit};

pub fn install_portage_snapshot(portage_snapshot_url: &str, mount_dir: &str) {
    println!("Installing Portage snapshot...");

    let portage_dir = format!("{}/var/db/repos/gentoo", mount_dir);

    std::fs::create_dir_all(&portage_dir).expect("Failed to create Portage directory");

    let wget_output = Command::new("wget")
        .arg(portage_snapshot_url)
        .arg("-O")
        .arg("portage.tar.bz2")
        .current_dir(&portage_dir)
        .output()
        .expect("Failed to execute wget");

    if wget_output.status.success() {
        println!("Portage snapshot downloaded successfully.");
    } else {
        eprintln!("Failed to download Portage snapshot: {}", String::from_utf8_lossy(&wget_output.stderr));
        exit(1);
    }

    let tar_output = Command::new("tar")
        .arg("xpf")
        .arg("portage.tar.bz2")
        .arg("--strip-components=1")
        .current_dir(&portage_dir)
        .output()
        .expect("Failed to execute tar");

    if tar_output.status.success() {
        println!("Portage snapshot extracted successfully.");
    } else {
        eprintln!("Failed to extract Portage snapshot: {}", String::from_utf8_lossy(&tar_output.stderr));
        exit(1);
    }
}
