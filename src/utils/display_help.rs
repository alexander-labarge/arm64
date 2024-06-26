use colored::*;

pub fn display_help() {
    let ascii_art = r#"
    _______   __          _______          ______                        __                         
    |       \ |  \        |       \        /      \                      |  \                        
    | $$$$$$$\ \$$        | $$$$$$$       |  $$$$$$\  ______   _______  _| $$_     ______    ______  
    | $$__/ $$|  \ ______ | $$____        | $$ __\$$ /      \ |       \|   $$ \   /      \  /      \ 
    | $$    $$| $$|      \| $$    \       | $$|    \|  $$$$$$\| $$$$$$$\\$$$$$$  |  $$$$$$\|  $$$$$$\
    | $$$$$$$ | $$ \$$$$$$ \$$$$$$$\      | $$ \$$$$| $$    $$| $$  | $$ | $$ __ | $$  | $$| $$  | $$
    | $$      | $$        |  \__| $$      | $$__| $$| $$$$$$$$| $$  | $$ | $$|  \| $$__/ $$| $$__/ $$
    | $$      | $$         \$$    $$       \$$    $$ \$$     \| $$  | $$  \$$  $$ \$$    $$ \$$    $$
     \$$       \$$          \$$$$$$         \$$$$$$   \$$$$$$$ \$$   \$$   \$$$$   \$$$$$$   \$$$$$$                                                                                                                                                                                      
    "#;

    let title = "Raspberry Pi 5: Gentoo Installer";

    println!("{}", ascii_art.bold().purple());
    println!("{}", title.bold().green());
    println!();
    println!("{}", "Usage: sudo ./pi_installer [OPTIONS]".bold().yellow());
    println!();
    println!("{}", "Arguments:".bold().yellow());
    println!("  {:<30} {}", "--target_drive".bold().cyan(), "The target drive to install Gentoo (default: /dev/sda)".green());
    println!("  {:<30} {}", "--boot_size".bold().cyan(), "The size of the boot partition (default: 512M)".green());
    println!("  {:<30} {}", "--swap_size".bold().cyan(), "The size of the swap partition (default: 8G)".green());
    println!("  {:<30} {}", "--stage3_url".bold().cyan(), "The URL to download the stage3 tarball (default: URL)".green());
    println!("  {:<30} {}", "--portage_snapshot_url".bold().cyan(), "The URL to download the portage snapshot (default: URL)".green());
    println!("  {:<30} {}", "--hostname".bold().cyan(), "The hostname for the new system (default: gentoo-pi5-router)".green());
    println!("  {:<30} {}", "--root_password_hash".bold().cyan(), "The hashed root password (default: hashed 'skywalker')".green());
    println!("  {:<30} {}", "--cmdline_console".bold().cyan(), "The console parameter for cmdline.txt (default: console=tty1)".green());
    println!("  {:<30} {}", "--cmdline_extra".bold().cyan(), "Additional parameters for cmdline.txt (default: params)".green());
    println!("  {:<30} {}", "--config_audio".bold().cyan(), "The audio parameter for config.txt (default: dtparam=audio=on)".green());
    println!("  {:<30} {}", "--config_overlay".bold().cyan(), "The overlay parameter for config.txt (default: dtoverlay=vc4-kms-v3d)".green());
    println!("  {:<30} {}", "--config_max_framebuffers".bold().cyan(), "The max framebuffers parameter for config.txt (default: max_framebuffers=2)".green());
    println!("  {:<30} {}", "--config_fw_kms_setup".bold().cyan(), "The firmware KMS setup parameter for config.txt (default: disable_fw_kms_setup=1)".green());
    println!("  {:<30} {}", "--config_64bit".bold().cyan(), "The 64-bit mode parameter for config.txt (default: arm_64bit=1)".green());
    println!("  {:<30} {}", "--config_overscan".bold().cyan(), "The overscan parameter for config.txt (default: disable_overscan=1)".green());
    println!("  {:<30} {}", "--config_arm_boost".bold().cyan(), "The ARM boost parameter for config.txt (default: arm_boost=1)".green());
    println!("  {:<30} {}", "--config_otg_mode".bold().cyan(), "The OTG mode parameter for config.txt (default: otg_mode=1)".green());
    println!("  {:<30} {}", "--config_pcie".bold().cyan(), "The PCIe parameter for config.txt (default: dtparam=pciex1)".green());
    println!("  {:<30} {}", "--config_pcie_gen".bold().cyan(), "The PCIe generation parameter for config.txt (default: dtparam=pciex1_gen=3)".green());
    println!("  {:<30} {}", "--config_usb_power".bold().cyan(), "The USB power parameter for config.txt (default: usb_max_current_enable=1)".green());
    println!("  {:<30} {}", "--username".bold().cyan(), "The username to create (default: skywalker)".green());
    println!("  {:<30} {}", "--password".bold().cyan(), "The password for the created user (default: skywalker)".green());
    println!("  {:<30} {}", "--extra_packages".bold().cyan(), "Additional packages to install in the Gentoo system (default: \"\")".green());
}
