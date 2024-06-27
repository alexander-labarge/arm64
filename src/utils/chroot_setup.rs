use std::process::Command;
use std::ops::Drop;

struct MountGuard {
    proc_path: String,
    sys_path: String,
    dev_path: String,
}

impl MountGuard {
    fn new(mount_dir: &str) -> MountGuard {
        MountGuard {
            proc_path: format!("{}/proc", mount_dir),
            sys_path: format!("{}/sys", mount_dir),
            dev_path: format!("{}/dev", mount_dir),
        }
    }

    fn unmount(&self) {
        let _ = Command::new("umount")
            .arg(&self.proc_path)
            .output();
        let _ = Command::new("umount")
            .arg(&self.sys_path)
            .output();
        let _ = Command::new("umount")
            .arg(&self.dev_path)
            .output();
    }
}

impl Drop for MountGuard {
    fn drop(&mut self) {
        self.unmount();
    }
}

pub fn chroot_setup(
    mount_dir: &str,
    hostname: &str,
    username: &str,
    password: &str,
    root_password_hash: &str,
    timezone_choice: &str,
) -> Result<String, String> {
    println!("Setting up chroot environment...");

    let mount_guard = MountGuard::new(mount_dir);

    let resolv_conf_path = format!("{}/etc/resolv.conf", mount_dir);
    let proc_path = &mount_guard.proc_path;
    let sys_path = &mount_guard.sys_path;
    let dev_path = &mount_guard.dev_path;

    // Copy resolv.conf
    if !copy_resolv_conf(&resolv_conf_path) {
        return Err("Failed to copy resolv.conf.".to_string());
    }

    // Mount /proc
    if !mount_fs("proc", "/proc", proc_path) {
        return Err("Failed to mount /proc.".to_string());
    }

    // Mount /sys with rbind and make-rslave
    if !mount_rbind_and_rslave("/sys", sys_path) {
        return Err("Failed to mount /sys or make it rslave.".to_string());
    }

    // Mount /dev with rbind and make-rslave
    if !mount_rbind_and_rslave("/dev", dev_path) {
        return Err("Failed to mount /dev or make it rslave.".to_string());
    }

    // Update environment
    let profile_command = "source /etc/profile";
    if !execute_chroot_command(mount_dir, profile_command) {
        return Err("Failed to load profile.".to_string());
    }

    // Set timezone
    let timezone_command = format!("ln -sf /usr/share/zoneinfo/{} /etc/localtime", timezone_choice);
    if !execute_chroot_command(mount_dir, &timezone_command) {
        return Err("Failed to set timezone.".to_string());
    }

    // Enable locales
    let locales_command = "sed -i '/en_US.UTF-8 UTF-8/s/^#//g' /etc/locale.gen && locale-gen";
    if !execute_chroot_command(mount_dir, locales_command) {
        return Err("Failed to enable locales.".to_string());
    }

    // Eselect locale
    let eselect_locale_command = "eselect locale set en_US.utf8 && env-update && source /etc/profile";
    if !execute_chroot_command(mount_dir, eselect_locale_command) {
        return Err("Failed to set locale.".to_string());
    }

    // Eselect news
    let news_command = "eselect news read";
    if !execute_chroot_command(mount_dir, news_command) {
        return Err("Failed to read eselect news.".to_string());
    }

    // Set hostname
    let hostname_command = format!("echo \"{}\" > /etc/hostname", hostname);
    if !execute_chroot_command(mount_dir, &hostname_command) {
        return Err("Failed to set hostname.".to_string());
    }

    // Create user
    let user_command = format!("useradd -m -G users,wheel -s /bin/bash {}", username);
    if !execute_chroot_command(mount_dir, &user_command) {
        return Err("Failed to create user.".to_string());
    }

    // Set user password
    let password_command = format!("echo -e \"{}\\n{}\" | passwd {}", password, password, username);
    if !execute_chroot_command(mount_dir, &password_command) {
        return Err("Failed to set user password.".to_string());
    }

    // Configure sudoers
    let sudoers_command = format!("echo \"{} ALL=(ALL) NOPASSWD: ALL\" >> /etc/sudoers", username);
    if !execute_chroot_command(mount_dir, &sudoers_command) {
        return Err("Failed to configure sudoers.".to_string());
    }

    // Backup shadow file
    let shadow_backup_command = "cp /etc/shadow /etc/shadow.backup";
    if !execute_chroot_command(mount_dir, shadow_backup_command) {
        return Err("Failed to backup shadow file.".to_string());
    }

    // Set root password
    let root_password_command = format!("sed -i \"s|^root:[^:]*:|root:{}:|g\" /etc/shadow", root_password_hash);
    if !execute_chroot_command(mount_dir, &root_password_command) {
        return Err("Failed to set root password.".to_string());
    }

    // Configure make.conf
    let make_conf_command = r#"
        cat <<EOF > /etc/portage/make.conf
COMMON_FLAGS="-mcpu=cortex-a76+crc+crypto -mtune=cortex-a76 -O2 -pipe"
CFLAGS="${COMMON_FLAGS}"
CXXFLAGS="${COMMON_FLAGS}"
FCFLAGS="${COMMON_FLAGS}"
FFLAGS="${COMMON_FLAGS}"
CHOST="aarch64-unknown-linux-gnu"
LC_MESSAGES=C.utf8
ACCEPT_LICENSE="*"
MAKEOPTS="-j5"
EOF
    "#;
    if !execute_chroot_command(mount_dir, make_conf_command) {
        return Err("Failed to configure make.conf.".to_string());
    }

    // Update the system
    let update_system_command = "emerge --sync && emerge -uDN @world";
    if !execute_chroot_command(mount_dir, update_system_command) {
        return Err("Failed to update the system.".to_string());
    }

    // Install cpuid2cpuflags
    if !execute_chroot_command(mount_dir, "emerge cpuid2cpuflags") {
        return Err("Failed to install cpuid2cpuflags.".to_string());
    }

    // Configure package.use and package.accept_keywords
    let configure_package_use_command = r#"
        echo "*/* $(cpuid2cpuflags)" > /etc/portage/package.use/00cpu-flags
        mkdir -p /etc/portage/package.use/custom
        echo ">=net-wireless/wpa_supplicant-2.10-r3 dbus" > /etc/portage/package.use/networkmanager
        echo "=sys-block/io-scheduler-udev-rules-2 ~arm64" > /etc/portage/package.accept_keywords/io-scheduler
    "#;
    if !execute_chroot_command(mount_dir, configure_package_use_command) {
        return Err("Failed to configure package.use and package.accept_keywords.".to_string());
    }

    // Packages to install
    let packages = [
        "net-misc/networkmanager",
        "net-misc/openssh",
        "net-misc/chrony",
        "app-admin/sudo",
        "net-misc/wget",
        "dev-vcs/git",
        "sys-apps/parted",
        "net-misc/curl",
        "app-misc/tree",
        "app-editors/vim",
        "app-misc/neofetch"
    ];

    // Install packages
    for package in &packages {
        let install_package_command = format!("emerge --verbose --autounmask-continue=y {}", package);
        if !execute_chroot_command(mount_dir, &install_package_command) {
            return Err(format!("Failed to install package: {}", package));
        }
    }

    // Enable and start services using OpenRC
    let services = [
        ("NetworkManager", "networkmanager"),
        ("sshd", "sshd"),
        ("chronyd", "chrony"),
        ("dhcpcd", "dhcpcd")
    ];

    for (service, description) in &services {
        let enable_service_command = format!("rc-update add {} default && rc-service {} start", service, service);
        if !execute_chroot_command(mount_dir, &enable_service_command) {
            return Err(format!("Failed to enable and start {}.", description));
        }
    }

    // Disable SSH root login
    let disable_ssh_root_command = "sed -i 's/#PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config";
    if !execute_chroot_command(mount_dir, disable_ssh_root_command) {
        return Err("Failed to disable SSH root login.".to_string());
    }

    // Generate SSH host keys
    let generate_ssh_keys_command = "ssh-keygen -A";
    if !execute_chroot_command(mount_dir, generate_ssh_keys_command) {
        return Err("Failed to generate SSH host keys.".to_string());
    }

    // Comment out the specified line in /etc/inittab
    let update_inittab_command = r#"
        sed -i 's|^f0:12345:respawn:/sbin/agetty 9600 ttyAMA0 vt100|#f0:12345:respawn:/sbin/agetty 9600 ttyAMA0 vt100|' /etc/inittab
    "#;
    if !execute_chroot_command(mount_dir, update_inittab_command) {
        return Err("Failed to update /etc/inittab.".to_string());
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
