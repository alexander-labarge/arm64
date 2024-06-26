use std::process::{Command, exit};

pub fn install_necessary_tools() {
    println!("Updating package list and installing necessary tools...");
    
    let update_output = Command::new("apt-get")
        .arg("update")
        .output()
        .expect("Failed to execute apt-get update");

    if !update_output.status.success() {
        eprintln!("Failed to update package list.");
        exit(1);
    }

    let install_output = Command::new("apt-get")
        .arg("install")
        .arg("-y")
        .arg("qemu-user-static")
        .arg("debootstrap")
        .arg("wget")
        .arg("git")
        .arg("parted")
        .arg("curl")
        .arg("tree")
        .arg("vim")
        .arg("neofetch")
        .output()
        .expect("Failed to execute apt-get install");

    if install_output.status.success() {
        println!("Necessary tools installed successfully.");
    } else {
        eprintln!("Failed to install necessary tools.");
        exit(1);
    }
}
