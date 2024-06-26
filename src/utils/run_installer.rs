use std::collections::HashMap;
use colored::*;

use crate::utils::{
    unmount,
    install_tools,
    create_partitions,
    format_partitions,
    install_gentoo,
    install_portage_snapshot,
    install_kernel_firmware,
    get_uuid,
    setup_boot_config,
    copy_firmware,
    setup_symlinks,
    create_fstab,
    chroot_setup::chroot_setup, // function only available in this module
};

pub fn run_installer(params: HashMap<String, String>) {
    // Extracting parameters with default values
    let hostname = params.get("--hostname").unwrap_or(&"gentoo-pi5-router".to_string()).to_string();
    let target_device = params.get("--target_drive").unwrap_or(&"/dev/sda".to_string()).to_string();
    let boot_size = params.get("--boot_size").unwrap_or(&"1G".to_string()).to_string();
    let swap_size = params.get("--swap_size").unwrap_or(&"8G".to_string()).to_string();
    let stage3_url = params.get("--stage3_url").unwrap_or(&"https://distfiles.gentoo.org/releases/arm64/autobuilds/latest-stage3-arm64-desktop-openrc.tar.xz".to_string()).to_string();
    let portage_snapshot_url = params.get("--portage_snapshot_url").unwrap_or(&"https://distfiles.gentoo.org/snapshots/portage-latest.tar.bz2".to_string()).to_string();
    let root_password_hash = params.get("--root_password_hash").unwrap_or(&"$6$.KYgMi02hVG4MNRk$1y6XS8QuIWEsqZNj6VFL9q9vMbItPkzPRi.Uh4/iiPIihsrx7ky23Rrwt.44IrkA76cx2HOrxrrMOOvz6TK6A/".to_string()).to_string(); // pw == skywalker
    let cmdline_console = params.get("--cmdline_console").unwrap_or(&"console=ttyAMA0,115200".to_string()).to_string();
    let cmdline_extra = params.get("--cmdline_extra").unwrap_or(&"dwc_otg.lpm_enable=0 rootfstype=ext4 rootwait cma=256M@256M net.ifnames=0".to_string()).to_string();
    let config_audio = params.get("--config_audio").unwrap_or(&"dtparam=audio=on".to_string()).to_string();
    let config_overlay = params.get("--config_overlay").unwrap_or(&"dtoverlay=vc4-kms-v3d".to_string()).to_string();
    let config_max_framebuffers = params.get("--config_max_framebuffers").unwrap_or(&"max_framebuffers=2".to_string()).to_string();
    let config_fw_kms_setup = params.get("--config_fw_kms_setup").unwrap_or(&"disable_fw_kms_setup=1".to_string()).to_string();
    let config_64bit = params.get("--config_64bit").unwrap_or(&"arm_64bit=1".to_string()).to_string();
    let config_overscan = params.get("--config_overscan").unwrap_or(&"disable_overscan=1".to_string()).to_string();
    let config_arm_boost = params.get("--config_arm_boost").unwrap_or(&"arm_boost=1".to_string()).to_string();
    let config_otg_mode = params.get("--config_otg_mode").unwrap_or(&"otg_mode=1".to_string()).to_string();
    let config_pcie = params.get("--config_pcie").unwrap_or(&"dtparam=pciex1".to_string()).to_string();
    let config_pcie_gen = params.get("--config_pcie_gen").unwrap_or(&"dtparam=pciex1_gen=3".to_string()).to_string();
    let config_usb_power = params.get("--config_usb_power").unwrap_or(&"usb_max_current_enable=1".to_string()).to_string();
    let username = params.get("--username").unwrap_or(&"skywalker".to_string()).to_string();
    let password = params.get("--password").unwrap_or(&"skywalker".to_string()).to_string();
    let extra_packages = params.get("--extra_packages").unwrap_or(&"".to_string()).to_string();
    let timezone_choice = params.get("--timezone").unwrap_or(&"America/New_York".to_string()).to_string();

    // Determine partition suffix
    let partition_suffix = if target_device.contains("nvme") || target_device.contains("mmcblk") {
        "p"
    } else {
        ""
    };

    // Constants for paths
    let mount_dir = "/mnt/gentoo";
    let boot_partition = format!("{}{}1", target_device, partition_suffix);
    let swap_partition = format!("{}{}2", target_device, partition_suffix);
    let root_partition = format!("{}{}3", target_device, partition_suffix);
    let firmware_repo = "https://github.com/raspberrypi/firmware";
    let nonfree_repo = "https://github.com/RPi-Distro/firmware-nonfree.git";
    let bluez_repo = "https://github.com/RPi-Distro/bluez-firmware.git";
    let boot_dir = format!("{}/boot", mount_dir);
    let cmdline_txt = format!("{}/cmdline.txt", boot_dir);
    let config_txt = format!("{}/config.txt", boot_dir);
    let shadow_file = "/etc/shadow";
    let sshd_config_file = "/etc/ssh/sshd_config";
    let nvme_luks_password = "WouldntYouLikeToKnow";
    let nvme_luks_password_file = "/opt/nvme_luks_password.txt";

    // Display the configuration
    println!("{}", format!("\nConfiguration:\n").bold().green());
    println!("  {:<30} {}", "hostname".bold().green(), hostname);
    println!("  {:<30} {}", "target_drive".bold().green(), target_device);
    println!("  {:<30} {}", "mount_dir".bold().green(), mount_dir);
    println!("  {:<30} {}", "boot_partition".bold().green(), boot_partition);
    println!("  {:<30} {}", "swap_partition".bold().green(), swap_partition);
    println!("  {:<30} {}", "root_partition".bold().green(), root_partition);
    println!("  {:<30} {}", "stage3_url".bold().green(), stage3_url);
    println!("  {:<30} {}", "firmware_repo".bold().green(), firmware_repo);
    println!("  {:<30} {}", "nonfree_repo".bold().green(), nonfree_repo);
    println!("  {:<30} {}", "bluez_repo".bold().green(), bluez_repo);
    println!("  {:<30} {}", "boot_dir".bold().green(), boot_dir);
    println!("  {:<30} {}", "cmdline_txt".bold().green(), cmdline_txt);
    println!("  {:<30} {}", "config_txt".bold().green(), config_txt);
    println!("  {:<30} {}", "root_password_hash".bold().green(), root_password_hash);
    println!("  {:<30} {}", "shadow_file".bold().green(), shadow_file);
    println!("  {:<30} {}", "sshd_config_file".bold().green(), sshd_config_file);
    println!("  {:<30} {}", "nvme_luks_password".bold().green(), nvme_luks_password);
    println!("  {:<30} {}", "nvme_luks_password_file".bold().green(), nvme_luks_password_file);
    println!("  {:<30} {}", "boot_size".bold().green(), boot_size);
    println!("  {:<30} {}", "swap_size".bold().green(), swap_size);
    println!("  {:<30} {}", "portage_snapshot_url".bold().green(), portage_snapshot_url);
    println!("  {:<30} {}", "cmdline_console".bold().green(), cmdline_console);
    println!("  {:<30} {}", "cmdline_extra".bold().green(), cmdline_extra);
    println!("  {:<30} {}", "config_audio".bold().green(), config_audio);
    println!("  {:<30} {}", "config_overlay".bold().green(), config_overlay);
    println!("  {:<30} {}", "config_max_framebuffers".bold().green(), config_max_framebuffers);
    println!("  {:<30} {}", "config_fw_kms_setup".bold().green(), config_fw_kms_setup);
    println!("  {:<30} {}", "config_64bit".bold().green(), config_64bit);
    println!("  {:<30} {}", "config_overscan".bold().green(), config_overscan);
    println!("  {:<30} {}", "config_arm_boost".bold().green(), config_arm_boost);
    println!("  {:<30} {}", "config_otg_mode".bold().green(), config_otg_mode);
    println!("  {:<30} {}", "config_pcie".bold().green(), config_pcie);
    println!("  {:<30} {}", "config_pcie_gen".bold().green(), config_pcie_gen);
    println!("  {:<30} {}", "config_usb_power".bold().green(), config_usb_power);
    println!("  {:<30} {}", "username".bold().green(), username);
    println!("  {:<30} {}", "password".bold().green(), password);
    println!("  {:<30} {}", "extra_packages".bold().green(), extra_packages);
    println!("  {:<30} {}", "timezone".bold().green(), timezone_choice);

    let confirm = if params.get("--automate").map_or(false, |v| v == "y") {
        "y".to_string()
    } else {
        let mut input = String::new();
        println!("{}", format!("\nWARNING: This will destroy all data on the target drive.\nAre you sure you want to proceed? (y/N): ").bold().red());
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    };

    if !matches!(confirm.as_str(), "y" | "Y" | "yes" | "Yes" | "YES") {
        println!("Operation aborted.");
        return;
    }

    // Unmount any existing partitions on the target drive
    unmount::unmount_partitions_on_drive(&target_device);

    // Install necessary tools
    install_tools::install_necessary_tools();

    // Create partitions
    create_partitions::create_partitions(&target_device, &boot_size, &swap_size);

    // Format partitions
    format_partitions::format_partitions(&target_device);

    // Install Gentoo base system
    install_gentoo::install_gentoo(&stage3_url, mount_dir, &target_device);

    // Install Portage snapshot
    install_portage_snapshot::install_portage_snapshot(&portage_snapshot_url, mount_dir);

    // Install Kernel and Firmware
    install_kernel_firmware::install_kernel_firmware(mount_dir, &target_device);

    // Get UUIDs for partitions
    let uuid_root = get_uuid::get_uuid(&root_partition);
    let uuid_boot = get_uuid::get_uuid(&boot_partition);
    let uuid_swap = get_uuid::get_uuid(&swap_partition);

    // Setup boot configuration
    setup_boot_config::setup_boot_config(
        mount_dir,
        &uuid_root,
        &cmdline_console,
        &cmdline_extra,
        &vec![
            &config_audio,
            &config_overlay,
            &config_max_framebuffers,
            &config_fw_kms_setup,
            &config_64bit,
            &config_overscan,
            &config_arm_boost,
            &config_otg_mode,
            &config_pcie,
            &config_pcie_gen,
            &config_usb_power,
        ]
    );

    // Copy firmware
    copy_firmware::copy_firmware(mount_dir);

    // Setup firmware symlinks
    setup_symlinks::setup_firmware_symlinks(mount_dir);

    // Create fstab
    create_fstab::create_fstab(mount_dir, &uuid_root, &uuid_boot, &uuid_swap);

    // Chroot setup
    chroot_setup(
        &mount_dir,
        &hostname,
        &username,
        &password,
        &root_password_hash,
        &timezone_choice,
    );

    println!("{}", "Gentoo installation on Raspberry Pi 5 completed successfully.".bold().green());
}
