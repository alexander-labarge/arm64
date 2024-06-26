use std::fs::File;
use std::io::Write;

pub fn create_fstab(mount_dir: &str, uuid_root: &str, uuid_boot: &str, uuid_swap: &str) {
    let fstab_content = format!(
        "# <fs>      <mountpoint> <type>  <opts>          <dump/pass>
UUID={} /            ext4    noatime         0 1
UUID={} /boot        vfat    noatime,noauto,nodev,nosuid,noexec  1 2
UUID={} none         swap    defaults        0 0
",
        uuid_root, uuid_boot, uuid_swap
    );

    let fstab_path = format!("{}/etc/fstab", mount_dir);
    let mut fstab_file = File::create(&fstab_path).expect("Failed to create fstab file");
    fstab_file.write_all(fstab_content.as_bytes()).expect("Failed to write to fstab file");

    println!("fstab created successfully at {}", fstab_path);
}
