use std::process::{Command, exit};

pub fn create_partitions(target_drive: &str, boot_size: &str, swap_size: &str) {
    println!("Creating partitions on {}", target_drive);
    println!("WARNING: This will destroy all data on {}.", target_drive);

    let confirm = {
        let mut input = String::new();
        println!("Are you sure you want to proceed? (y/N): ");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    };

    if matches!(confirm.as_str(), "y" | "Y" | "yes" | "Yes" | "YES") {
        let output = Command::new("fdisk")
            .arg(target_drive)
            .arg("<<EOF")
            .arg("g")
            .arg("n")
            .arg("")
            .arg("")
            .arg(boot_size)
            .arg("n")
            .arg("")
            .arg("")
            .arg(swap_size)
            .arg("n")
            .arg("")
            .arg("")
            .arg("")
            .arg("t")
            .arg("1")
            .arg("11")
            .arg("t")
            .arg("2")
            .arg("19")
            .arg("w")
            .output()
            .expect("Failed to execute fdisk");

        if output.status.success() {
            println!("Partitions created successfully.");
        } else {
            eprintln!("Failed to create partitions: {}", String::from_utf8_lossy(&output.stderr));
            exit(1);
        }
    } else {
        println!("Operation aborted.");
        exit(1);
    }
}
