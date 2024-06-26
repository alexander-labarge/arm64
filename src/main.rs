mod utils;

fn main() {
    // Ensure script is run as root
    utils::check_root::check_root();
    
    // Parse command line arguments
    let params = utils::arguments::parse_arguments();

    // Get configuration with default values
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

    // TODO: Automated hooks for web entry

    println!("Configuration:
        hostname: {}
        target_drive: {}
        mount_dir: {}
        boot_partition: {}
        swap_partition: {}
        root_partition: {}
        stage3_url: {}
        firmware_repo: {}
        nonfree_repo: {}
        bluez_repo: {}
        boot_dir: {}
        cmdline_txt: {}
        config_txt: {}
        root_password_hash: {}
        shadow_file: {}
        sshd_config_file: {}
        nvme_luks_password: {}
        nvme_luks_password_file: {}
        boot_size: {}
        swap_size: {}
        portage_snapshot_url: {}
        cmdline_console: {}
        cmdline_extra: {}
        config_audio: {}
        config_overlay: {}
        config_max_framebuffers: {}
        config_fw_kms_setup: {}
        config_64bit: {}
        config_overscan: {}
        config_arm_boost: {}
        config_otg_mode: {}
        config_pcie: {}
        config_pcie_gen: {}
        config_usb_power: {}
        username: {}
        password: {}
        extra_packages: {}
        ",
        hostname, target_device, mount_dir, boot_partition, swap_partition, root_partition, stage3_url, firmware_repo, nonfree_repo,
        bluez_repo, boot_dir, cmdline_txt, config_txt, root_password_hash, shadow_file, sshd_config_file, nvme_luks_password,
        nvme_luks_password_file, boot_size, swap_size, portage_snapshot_url, cmdline_console, cmdline_extra,
        config_audio, config_overlay, config_max_framebuffers, config_fw_kms_setup, config_64bit, config_overscan,
        config_arm_boost, config_otg_mode, config_pcie, config_pcie_gen, config_usb_power, username, password, extra_packages
    );

    // Unmount any existing partitions on the target drive
    utils::unmount::unmount_partitions_on_drive(&target_device);

    // Install necessary tools
    utils::install_tools::install_necessary_tools();

    // Create partitions
    utils::create_partitions::create_partitions(&target_device, &boot_size, &swap_size);

    // Format partitions
    utils::format_partitions::format_partitions(&target_device);

    // Install Gentoo base system
    utils::install_gentoo::install_gentoo(&stage3_url, mount_dir, &target_device);

    // Install Portage snapshot
    utils::install_portage_snapshot::install_portage_snapshot(&portage_snapshot_url, mount_dir);

    // Install Kernel and Firmware
    utils::install_kernel_firmware::install_kernel_firmware(mount_dir, &target_device);

    // Get UUIDs for partitions
    let uuid_root = utils::get_uuid::get_uuid(&root_partition);
    let uuid_boot = utils::get_uuid::get_uuid(&boot_partition);
    let uuid_swap = utils::get_uuid::get_uuid(&swap_partition);

    // Setup boot configuration
    utils::setup_boot_config::setup_boot_config(
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
    utils::copy_firmware::copy_firmware(mount_dir);

    // Setup firmware symlinks
    utils::setup_symlinks::setup_firmware_symlinks(mount_dir);

    // Create fstab
    utils::create_fstab::create_fstab(mount_dir, &uuid_root, &uuid_boot, &uuid_swap);

    // Chroot setup
    utils::chroot_setup::chroot_setup(mount_dir, &hostname, &username, &password, &root_password_hash);

    println!("Gentoo installation on Raspberry Pi 5 completed successfully.");
}
