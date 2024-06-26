use colored::*;

pub fn print_logo() {
    let ascii_art = r#"
    =================================================================================================
    _______   __          _______          ______                        __                         
    |       \ |  \        |       \        /      \                      |  \                        
    | $$$$$$$\ \$$        | $$$$$$$       |  $$$$$$\  ______   _______  _| $$_     ______    ______  
    | $$__/ $$|  \ ______ | $$____        | $$ __\$$ /      \ |       \|   $$ \   /      \  /      \ 
    | $$    $$| $$|      \| $$    \       | $$|    \|  $$$$$$\| $$$$$$$\\$$$$$$  |  $$$$$$\|  $$$$$$\
    | $$$$$$$ | $$ \$$$$$$ \$$$$$$$\      | $$ \$$$$| $$    $$| $$  | $$ | $$ __ | $$  | $$| $$  | $$
    | $$      | $$        |  \__| $$      | $$__| $$| $$$$$$$$| $$  | $$ | $$|  \| $$__/ $$| $$__/ $$
    | $$      | $$         \$$    $$       \$$    $$ \$$     \| $$  | $$  \$$  $$ \$$    $$ \$$    $$
     \$$       \$$          \$$$$$$         \$$$$$$   \$$$$$$$ \$$   \$$   \$$$$   \$$$$$$   \$$$$$$ 

    =================================================================================================
    "#;

    let title = "Raspberry Pi 5: Gentoo Installer";

    println!("{}", ascii_art.bold().bright_white());
    println!("{}", title.bold().yellow().underline());
}

pub fn display_help() {
    print_logo();
    println!();
    println!("{}", "Usage: sudo ./pi_installer [OPTIONS]".bold().yellow());
    println!();
    println!("{}", "Arguments:".bold().yellow());
    println!("  {:<30} {}", "--target_drive".bold().cyan(), "The target drive to install Gentoo (default: /dev/sda)".bold().white());
    println!("  {:<30} {}", "--boot_size".bold().cyan(), "The size of the boot partition (default: 512M)".bold().white());
    println!("  {:<30} {}", "--swap_size".bold().cyan(), "The size of the swap partition (default: 8G)".bold().white());
    println!("  {:<30} {}", "--stage3_url".bold().cyan(), "The URL to download the stage3 tarball (default: URL)".bold().white());
    println!("  {:<30} {}", "--portage_snapshot_url".bold().cyan(), "The URL to download the portage snapshot (default: URL)".bold().white());
    println!("  {:<30} {}", "--hostname".bold().cyan(), "The hostname for the new system (default: gentoo-pi5-router)".bold().white());
    println!("  {:<30} {}", "--root_password_hash".bold().cyan(), "The hashed root password (default: hashed 'skywalker')".bold().white());
    println!("  {:<30} {}", "--cmdline_console".bold().cyan(), "The console parameter for cmdline.txt (default: console=tty1)".bold().white());
    println!("  {:<30} {}", "--cmdline_extra".bold().cyan(), "Additional parameters for cmdline.txt (default: params)".bold().white());
    println!("  {:<30} {}", "--config_audio".bold().cyan(), "The audio parameter for config.txt (default: dtparam=audio=on)".bold().white());
    println!("  {:<30} {}", "--config_overlay".bold().cyan(), "The overlay parameter for config.txt (default: dtoverlay=vc4-kms-v3d)".bold().white());
    println!("  {:<30} {}", "--config_max_framebuffers".bold().cyan(), "The max framebuffers parameter for config.txt (default: max_framebuffers=2)".bold().white());
    println!("  {:<30} {}", "--config_fw_kms_setup".bold().cyan(), "The firmware KMS setup parameter for config.txt (default: disable_fw_kms_setup=1)".bold().white());
    println!("  {:<30} {}", "--config_64bit".bold().cyan(), "The 64-bit mode parameter for config.txt (default: arm_64bit=1)".bold().white());
    println!("  {:<30} {}", "--config_overscan".bold().cyan(), "The overscan parameter for config.txt (default: disable_overscan=1)".bold().white());
    println!("  {:<30} {}", "--config_arm_boost".bold().cyan(), "The ARM boost parameter for config.txt (default: arm_boost=1)".bold().white());
    println!("  {:<30} {}", "--config_otg_mode".bold().cyan(), "The OTG mode parameter for config.txt (default: otg_mode=1)".bold().white());
    println!("  {:<30} {}", "--config_pcie".bold().cyan(), "The PCIe parameter for config.txt (default: dtparam=pciex1)".bold().white());
    println!("  {:<30} {}", "--config_pcie_gen".bold().cyan(), "The PCIe generation parameter for config.txt (default: dtparam=pciex1_gen=3)".bold().white());
    println!("  {:<30} {}", "--config_usb_power".bold().cyan(), "The USB power parameter for config.txt (default: usb_max_current_enable=1)".bold().white());
    println!("  {:<30} {}", "--username".bold().cyan(), "The username to create (default: skywalker)".bold().white());
    println!("  {:<30} {}", "--password".bold().cyan(), "The password for the created user (default: skywalker)".bold().white());
    println!("  {:<30} {}", "--extra_packages".bold().cyan(), "Additional packages to install in the Gentoo system (default: \"\")".bold().white());
    println!("  {:<30} {}", "--timezone".bold().cyan(), "Timezone to set (default: America/New_York)".bold().white());
    println!("  {:<30} {}", "--automate".bold().cyan(), "Automatically confirm all prompts (default: n)".bold().white());
    println!();
    println!("{}", "Example:".bold().yellow());
    println!("{}", "sudo ./pi_installer \\".bold().cyan());
    println!("    {} {}", "--target_drive".bold().cyan(), "/dev/sdb \\".bold().green());
    println!("    {} {}", "--boot_size".bold().cyan(), "1G \\".bold().green());
    println!("    {} {}", "--swap_size".bold().cyan(), "8G \\".bold().green());
    println!("    {} {}", "--stage3_url".bold().cyan(), "https://distfiles.gentoo.org/releases/arm64/autobuilds/latest-stage3-arm64-desktop-openrc.tar.xz \\".bold().green());
    println!("    {} {}", "--portage_snapshot_url".bold().cyan(), "https://distfiles.gentoo.org/snapshots/portage-latest.tar.bz2 \\".bold().green());
    println!("    {} {}", "--hostname".bold().cyan(), "my-gentoo-pi \\".bold().green());
    println!("    {} {}", "--root_password_hash".bold().cyan(), "'$6$.KYgMi02hVG4MNRk$1y6XS8QuIWEsqZNj6VFL9q9vMbItPkzPRi.Uh4/iiPIihsrx7ky23Rrwt.44IrkA76cx2HOrxrrMOOvz6TK6A/' \\".bold().green());
    println!("    {} {}", "--cmdline_console".bold().cyan(), "console=tty1 \\".bold().green());
    println!("    {} {}", "--cmdline_extra".bold().cyan(), "'dwc_otg.lpm_enable=0 rootfstype=ext4 rootwait cma=256M@256M net.ifnames=0' \\".bold().green());
    println!("    {} {}", "--config_audio".bold().cyan(), "dtparam=audio=on \\".bold().green());
    println!("    {} {}", "--config_overlay".bold().cyan(), "dtoverlay=vc4-kms-v3d \\".bold().green());
    println!("    {} {}", "--config_max_framebuffers".bold().cyan(), "max_framebuffers=2 \\".bold().green());
    println!("    {} {}", "--config_fw_kms_setup".bold().cyan(), "disable_fw_kms_setup=1 \\".bold().green());
    println!("    {} {}", "--config_64bit".bold().cyan(), "arm_64bit=1 \\".bold().green());
    println!("    {} {}", "--config_overscan".bold().cyan(), "disable_overscan=1 \\".bold().green());
    println!("    {} {}", "--config_arm_boost".bold().cyan(), "arm_boost=1 \\".bold().green());
    println!("    {} {}", "--config_otg_mode".bold().cyan(), "otg_mode=1 \\".bold().green());
    println!("    {} {}", "--config_pcie".bold().cyan(), "dtparam=pciex1 \\".bold().green());
    println!("    {} {}", "--config_pcie_gen".bold().cyan(), "dtparam=pciex1_gen=3 \\".bold().green());
    println!("    {} {}", "--config_usb_power".bold().cyan(), "usb_max_current_enable=1 \\".bold().green());
    println!("    {} {}", "--username".bold().cyan(), "myuser \\".bold().green());
    println!("    {} {}", "--password".bold().cyan(), "mypassword \\".bold().green());
    println!("    {} {}", "--extra_packages".bold().cyan(), "'dev-vcs/git app-editors/vim' \\".bold().green());
    println!("    {} {}", "--timezone".bold().cyan(), "America/New_York \\".bold().green());
    println!("    {} {}", "--automate".bold().cyan(), "y".bold().green());
}
