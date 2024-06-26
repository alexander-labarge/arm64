use std::process::Command;

pub fn chroot_setup(mount_dir: &str, hostname: &str, username: &str, password: &str, root_password_hash: &str) {
    println!("Setting up chroot environment...");

    let resolv_conf_path = format!("{}/etc/resolv.conf", mount_dir);
    let proc_path = format!("{}/proc", mount_dir);
    let sys_path = format!("{}/sys", mount_dir);
    let dev_path = format!("{}/dev", mount_dir);

    Command::new("cp")
        .arg("--dereference")
        .arg("/etc/resolv.conf")
        .arg(&resolv_conf_path)
        .output()
        .expect("Failed to copy resolv.conf");

    Command::new("mount")
        .arg("--types")
        .arg("proc")
        .arg("/proc")
        .arg(&proc_path)
        .output()
        .expect("Failed to mount /proc");

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

    Command::new("chroot")
        .arg(mount_dir)
        .arg("/bin/bash")
        .arg("-c")
        .arg(format!(
            "source /etc/profile; \
            sed -i '/en_US ISO-8859-1/s/^#//g' /etc/locale.gen; \
            sed -i '/en_US.UTF-8 UTF-8/s/^#//g' /etc/locale.gen; \
            locale-gen; \
            eselect news read; \
            echo \"{}\" > /etc/hostname; \
            useradd -m -G users,wheel -s /bin/bash {}; \
            echo -e \"{}\\n{}\" | passwd {}; \
            echo \"{} ALL=(ALL) NOPASSWD: ALL\" >> /etc/sudoers; \
            ssh-keygen -A; \
            cp /etc/shadow /etc/shadow.backup; \
            sed -i \"s|^root:[^:]*:|root:{}:|g\" /etc/shadow;",
            hostname, username, password, password, username, username, root_password_hash
        ))
        .output()
        .expect("Failed to execute chroot setup");
}
