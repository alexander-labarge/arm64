use std::process::Command;

pub fn chroot_setup(
    mount_dir: &str,
    hostname: &str,
    username: &str,
    password: &str,
    root_password_hash: &str,
    timezone_choice: &str,
) -> Result<String, String> {
    println!("Setting up chroot environment...");

    let resolv_conf_path = format!("{}/etc/resolv.conf", mount_dir);
    let proc_path = format!("{}/proc", mount_dir);
    let sys_path = format!("{}/sys", mount_dir);
    let dev_path = format!("{}/dev", mount_dir);

    // Copy resolv.conf
    if !copy_resolv_conf(&resolv_conf_path) {
        return Err("Failed to copy resolv.conf.".to_string());
    }

    // Mount /proc
    if !mount_fs("proc", "/proc", &proc_path) {
        return Err("Failed to mount /proc.".to_string());
    }

    // Mount /sys with rbind and make-rslave
    if !mount_rbind_and_rslave("/sys", &sys_path) {
        return Err("Failed to mount /sys or make it rslave.".to_string());
    }

    // Mount /dev with rbind and make-rslave
    if !mount_rbind_and_rslave("/dev", &dev_path) {
        return Err("Failed to mount /dev or make it rslave.".to_string());
    }

    // Execute each command separately and report success or failure
    let profile_command = "source /etc/profile;";
    let timezone_command = format!("ln -sf /usr/share/zoneinfo/{} /etc/localtime;", timezone_choice);
    let locales_command = "sed -i '/en_US.UTF-8 UTF-8/s/^#//g' /etc/locale.gen; locale-gen;";
    let news_command = "eselect news read;";
    let hostname_command = format!("echo \"{}\" > /etc/hostname;", hostname);
    let user_command = format!("useradd -m -G users,wheel -s /bin/bash {};", username);
    let password_command = format!("echo -e \"{}\\n{}\" | passwd {};", password, password, username);
    let sudoers_command = format!("echo \"{} ALL=(ALL) NOPASSWD: ALL\" >> /etc/sudoers;", username);
    let shadow_backup_command = "cp /etc/shadow /etc/shadow.backup;";
    let root_password_command = format!("sed -i \"s|^root:[^:]*:|root:{}:|g\" /etc/shadow;", root_password_hash);

    let commands = vec![
        (profile_command, "Loading profile"),
        (&timezone_command, "Setting timezone"),
        (locales_command, "Enabling locales"),
        (news_command, "Eselect news"),
        (&hostname_command, "Setting hostname"),
        (&user_command, "Creating user"),
        (&password_command, "Setting user password"),
        (&sudoers_command, "Configuring sudoers"),
        (shadow_backup_command, "Backing up shadow file"),
        (&root_password_command, "Setting root password"),
    ];

    for (command, description) in commands {
        if !execute_chroot_command(mount_dir, command) {
            return Err(format!("Failed to complete {}.", description));
        }
    }

    Ok(String::new())
}

fn copy_resolv_conf(resolv_conf_path: &str) -> bool {
    match Command::new("cp")
        .arg("--dereference")
        .arg("/etc/resolv.conf")
        .arg(resolv_conf_path)
        .output()
    {
        Ok(output) => output.status.success(),
        Err(e) => {
            eprintln!("Error copying resolv.conf: {}", e);
            false
        }
    }
}

fn mount_fs(fs_type: &str, source: &str, target: &str) -> bool {
    match Command::new("mount")
        .arg("--types")
        .arg(fs_type)
        .arg(source)
        .arg(target)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("Mounted {} to {} successfully.", source, target);
                true
            } else {
                eprintln!("Failed to mount {} to {}: {}", source, target, String::from_utf8_lossy(&output.stderr));
                false
            }
        },
        Err(e) => {
            eprintln!("Error mounting {}: {}", fs_type, e);
            false
        }
    }
}

fn mount_rbind_and_rslave(source: &str, target: &str) -> bool {
    let rbind_status = Command::new("mount")
        .arg("--rbind")
        .arg(source)
        .arg(target)
        .output();

    match rbind_status {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Failed to rbind mount {} to {}: {}", source, target, String::from_utf8_lossy(&output.stderr));
                return false;
            }
        },
        Err(e) => {
            eprintln!("Error rbind mounting {}: {}", source, e);
            return false;
        }
    }

    let rslave_status = Command::new("mount")
        .arg("--make-rslave")
        .arg(target)
        .output();

    match rslave_status {
        Ok(output) => {
            if output.status.success() {
                println!("Mounted {} and made rslave successfully.", target);
                true
            } else {
                eprintln!("Failed to make {} rslave: {}", target, String::from_utf8_lossy(&output.stderr));
                false
            }
        },
        Err(e) => {
            eprintln!("Error making {} rslave: {}", target, e);
            false
        }
    }
}

fn execute_chroot_command(mount_dir: &str, command: &str) -> bool {
    match Command::new("chroot")
        .arg(mount_dir)
        .arg("/bin/bash")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("Command '{}' executed successfully in chroot.", command);
                true
            } else {
                eprintln!("Failed to execute chroot command '{}': {}", command, String::from_utf8_lossy(&output.stderr));
                false
            }
        },
        Err(e) => {
            eprintln!("Error executing chroot command '{}': {}", command, e);
            false
        }
    }
}
