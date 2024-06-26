use std::process::{Command, exit};
use std::io::Write;

pub fn create_partitions(target_drive: &str, boot_size: &str, swap_size: &str) {
    println!("Creating partitions on {}", target_drive);

    let commands = format!(
        "g\nn\n\n\n+{}\nn\n\n\n+{}\nn\n\n\n\nt\n1\n11\nt\n2\n19\nw\n",
        boot_size, swap_size
    );

    let output = Command::new("fdisk")
        .arg(target_drive)
        .arg("-c")
        .arg("-w")
        .output()
        .expect("Failed to execute fdisk");

    let mut child = Command::new("fdisk")
        .arg(target_drive)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute fdisk");

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(commands.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to read stdout");

    if output.status.success() {
        println!("Partitions created successfully.");
    } else {
        eprintln!("Failed to create partitions: {}", String::from_utf8_lossy(&output.stderr));
    }
}