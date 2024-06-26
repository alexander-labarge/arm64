use std::fs::File;
use std::io::Write;

pub fn setup_boot_config(
    mount_dir: &str, 
    uuid_root: &str, 
    cmdline_console: &str, 
    cmdline_extra: &str, 
    config_params: &Vec<&str>
) {
    println!("Setting up boot configuration...");

    let boot_dir = format!("{}/boot", mount_dir);
    let cmdline_txt_path = format!("{}/cmdline.txt", boot_dir);
    let config_txt_path = format!("{}/config.txt", boot_dir);

    // Create cmdline.txt
    let mut cmdline_txt = File::create(&cmdline_txt_path).expect("Failed to create cmdline.txt");
    write!(cmdline_txt, "root=UUID={} {} {}", uuid_root, cmdline_console, cmdline_extra)
        .expect("Failed to write to cmdline.txt");

    // Create config.txt
    let mut config_txt = File::create(&config_txt_path).expect("Failed to create config.txt");
    for param in config_params {
        writeln!(config_txt, "{}", param).expect("Failed to write to config.txt");
    }

    println!("Boot configuration files created:");
    println!("  - {}", cmdline_txt_path);
    println!("  - {}", config_txt_path);
}