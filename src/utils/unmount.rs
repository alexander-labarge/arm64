use std::process::Command;

pub fn unmount_partitions_on_drive(drive: &str) {
    let partprobe_output = Command::new("partprobe")
        .arg(drive)
        .output()
        .expect("Failed to execute partprobe");

    if !partprobe_output.status.success() {
        eprintln!("Failed to execute partprobe on the drive.");
        return;
    }

    let partitions_output = Command::new("lsblk")
        .arg("-ln")
        .arg("-o")
        .arg("NAME,MOUNTPOINT")
        .output()
        .expect("Failed to execute lsblk");

    let partitions_str = String::from_utf8_lossy(&partitions_output.stdout);

    for line in partitions_str.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 && parts[0].starts_with(drive) {
            let partition = parts[1];

            let umount_output = Command::new("umount")
                .arg("-f")
                .arg(partition)
                .output();
                
            match umount_output {
                Ok(output) if output.status.success() => println!("Unmounted {} successfully.", partition),
                _ => {
                    let lazy_unmount_output = Command::new("umount")
                        .arg("-l")
                        .arg(partition)
                        .output();
                        
                    match lazy_unmount_output {
                        Ok(_) => println!("Lazy unmounted {}.", partition),
                        Err(e) => {
                            eprintln!("Failed to unmount {}: {}", partition, e);
                            return;
                        },
                    }
                },
            }
        }
    }
}
