use std::process;

pub fn display_help() {
    println!("Usage: sudo ./pi_installer [--target_drive TARGET_DRIVE] [--boot_size BOOT_SIZE] [--swap_size SWAP_SIZE] [--stage3_url STAGE3_URL] [--portage_snapshot_url PORTAGE_SNAPSHOT_URL] [--hostname HOSTNAME] [--root_password_hash ROOT_PASSWORD_HASH] [--cmdline_console CMDLINE_CONSOLE] [--cmdline_extra CMDLINE_EXTRA] [--config_audio CONFIG_AUDIO] [--config_overlay CONFIG_OVERLAY] [--config_max_framebuffers CONFIG_MAX_FRAMEBUFFERS] [--config_fw_kms_setup CONFIG_FW_KMS_SETUP] [--config_64bit CONFIG_64BIT] [--config_overscan CONFIG_OVERSCAN] [--config_arm_boost CONFIG_ARM_BOOST] [--config_otg_mode CONFIG_OTG_MODE] [--config_pcie CONFIG_PCIE] [--config_pcie_gen CONFIG_PCIE_GEN] [--config_usb_power CONFIG_USB_POWER] [--username USERNAME] [--password PASSWORD] [--extra_packages EXTRA_PACKAGES]");
    println!();
    println!("Arguments:");
    println!("  --target_drive          The target drive to install Gentoo (default: /dev/sda)");
    println!("  --boot_size             The size of the boot partition (default: 512M)");
    println!("  --swap_size             The size of the swap partition (default: 8G)");
    println!("  --stage3_url            The URL to download the stage3 tarball (default: https://distfiles.gentoo.org/releases/arm64/autobuilds/latest-stage3-arm64-desktop-openrc.tar.xz)");
    println!("  --portage_snapshot_url  The URL to download the portage snapshot (default: https://distfiles.gentoo.org/snapshots/portage-latest.tar.bz2)");
    println!("  --hostname              The hostname for the new system (default: gentoo-pi5-router)");
    println!("  --root_password_hash    The hashed root password (default: hashed 'skywalker')");
    println!("  --cmdline_console       The console parameter for cmdline.txt (default: console=tty1)");
    println!("  --cmdline_extra         Additional parameters for cmdline.txt (default: dwc_otg.lpm_enable=0 rootfstype=ext4 rootwait cma=256M@256M net.ifnames=0)");
    println!("  --config_audio          The audio parameter for config.txt (default: dtparam=audio=on)");
    println!("  --config_overlay        The overlay parameter for config.txt (default: dtoverlay=vc4-kms-v3d)");
    println!("  --config_max_framebuffers The max framebuffers parameter for config.txt (default: max_framebuffers=2)");
    println!("  --config_fw_kms_setup   The firmware KMS setup parameter for config.txt (default: disable_fw_kms_setup=1)");
    println!("  --config_64bit          The 64-bit mode parameter for config.txt (default: arm_64bit=1)");
    println!("  --config_overscan       The overscan parameter for config.txt (default: disable_overscan=1)");
    println!("  --config_arm_boost      The ARM boost parameter for config.txt (default: arm_boost=1)");
    println!("  --config_otg_mode       The OTG mode parameter for config.txt (default: otg_mode=1)");
    println!("  --config_pcie           The PCIe parameter for config.txt (default: dtparam=pciex1)");
    println!("  --config_pcie_gen       The PCIe generation parameter for config.txt (default: dtparam=pciex1_gen=3)");
    println!("  --config_usb_power      The USB power parameter for config.txt (default: usb_max_current_enable=1)");
    println!("  --username              The username to create (default: skywalker)");
    println!("  --password              The password for the created user (default: skywalker)");
    println!("  --extra_packages        Additional packages to install in the Gentoo system (default: \"\")");
    process::exit(0);
}
