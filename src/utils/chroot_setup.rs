use std::process::Command;

pub fn chroot_setup(
    mount_dir: &str,
    hostname: &str,
    username: &str,
    password: &str,
    root_password_hash: &str,
    timezone_choice: &str,
) {
    println!("Setting up chroot environment...");

    let resolv_conf_path = format!("{}/etc/resolv.conf", mount_dir);
    let proc_path = format!("{}/proc", mount_dir);
    let sys_path = format!("{}/sys", mount_dir);
    let dev_path = format!("{}/dev", mount_dir);
    let boot_path = format!("{}/boot", mount_dir);

    // Copy resolv.conf
    Command::new("cp")
        .arg("--dereference")
        .arg("/etc/resolv.conf")
        .arg(&resolv_conf_path)
        .output()
        .expect("Failed to copy resolv.conf");

    // Mount /proc
    Command::new("mount")
        .arg("--types")
        .arg("proc")
        .arg("/proc")
        .arg(&proc_path)
        .output()
        .expect("Failed to mount /proc");

    // Mount /sys
    Command::new("mount")
        .arg("--rbind")
        .arg("/sys")
        .arg(&sys_path)
        .output()
        .expect("Failed to mount /sys");

    Command::new("mount")
        .arg("--make-rslave")
        .arg(&sys_path)
        .output()
        .expect("Failed to make /sys rslave");

    // Mount /dev
    Command::new("mount")
        .arg("--rbind")
        .arg("/dev")
        .arg(&dev_path)
        .output()
        .expect("Failed to mount /dev");

    Command::new("mount")
        .arg("--make-rslave")
        .arg(&dev_path)
        .output()
        .expect("Failed to make /dev rslave");

    // Mount /boot
    Command::new("mount")
        .arg("--rbind")
        .arg("/boot")
        .arg(&boot_path)
        .output()
        .expect("Failed to mount /boot");

    Command::new("mount")
        .arg("--make-rslave")
        .arg(&boot_path)
        .output()
        .expect("Failed to make /boot rslave");

    // Load profile
    let load_profile = "source /etc/profile;";

    // Set timezone
    let set_timezone = format!("ln -sf /usr/share/zoneinfo/{} /etc/localtime;", timezone_choice);

    // Enable locales
    let enable_locales = "
        sed -i '/en_US.UTF-8 UTF-8/s/^#//g' /etc/locale.gen; \
        locale-gen;";

    // Eselect news
    let eselect_news = "eselect news read;";

    // Set hostname
    let set_hostname = format!("echo \"{}\" > /etc/hostname;", hostname);

    // Create user
    let create_user = format!("useradd -m -G users,wheel -s /bin/bash {};", username);

    // Set user password
    let set_password = format!(
        "echo -e \"{}\\n{}\" | passwd {};",
        password, password, username
    );

    // Configure sudoers
    let configure_sudoers = format!(
        "echo \"{} ALL=(ALL) NOPASSWD: ALL\" >> /etc/sudoers;",
        username
    );

    // Generate SSH keys
    let generate_ssh_keys = "ssh-keygen -A;";

    // Backup shadow file
    let backup_shadow = "cp /etc/shadow /etc/shadow.backup;";

    // Set root password
    let set_root_password = format!(
        "sed -i \"s|^root:[^:]*:|root:{}:|g\" /etc/shadow;",
        root_password_hash
    );

    // Concatenate all commands
    let chroot_commands = format!(
        "{} {} {} {} {} {} {} {} {} {} {}",
        load_profile,
        set_timezone,
        enable_locales,
        eselect_news,
        set_hostname,
        create_user,
        set_password,
        configure_sudoers,
        generate_ssh_keys,
        backup_shadow,
        set_root_password
    );

    // Execute chroot setup
    Command::new("chroot")
        .arg(mount_dir)
        .arg("/bin/bash")
        .arg("-c")
        .arg(chroot_commands)
        .output()
        .expect("Failed to execute chroot setup");
}
