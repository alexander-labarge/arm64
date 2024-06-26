
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
cargo --version
cargo new pi_installer
cd pi_installer
mkdir -p src/utils
touch src/utils/arguments.rs
touch src/utils/check_root.rs
touch src/utils/chroot_setup.rs
touch src/utils/copy_firmware.rs
touch src/utils/create_fstab.rs
touch src/utils/create_partitions.rs
touch src/utils/display_help.rs
touch src/utils/format_partitions.rs
touch src/utils/get_uuid.rs
touch src/utils/install_gentoo.rs
touch src/utils/install_kernel_firmware.rs
touch src/utils/install_portage_snapshot.rs
touch src/utils/install_tools.rs
touch src/utils/mod.rs
touch src/utils/setup_boot_config.rs
touch src/utils/setup_symlinks.rs
touch src/utils/unmount.rs

cargo run -- \
  --hostname gentoo-pi5-router \
  --target_drive /dev/sda \
  --boot_size 1G \
  --swap_size 8G \
  --stage3_url https://distfiles.gentoo.org/releases/arm64/autobuilds/latest-stage3-arm64-desktop-openrc.tar.xz \
  --portage_snapshot_url https://distfiles.gentoo.org/snapshots/portage-latest.tar.bz2 \
  --root_password_hash $6$.KYgMi02hVG4MNRk$1y6XS8QuIWEsqZNj6VFL9q9vMbItPkzPRi.Uh4/iiPIihsrx7ky23Rrwt.44IrkA76cx2HOrxrrMOOvz6TK6A/ \
  --cmdline_console console=ttyAMA0,115200 \
  --cmdline_extra dwc_otg.lpm_enable=0 rootfstype=ext4 rootwait cma=256M@256M net.ifnames=0 \
  --config_audio dtparam=audio=on \
  --config_overlay dtoverlay=vc4-kms-v3d \
  --config_max_framebuffers max_framebuffers=2 \
  --config_fw_kms_setup disable_fw_kms_setup=1 \
  --config_64bit arm_64bit=1 \
  --config_overscan disable_overscan=1 \
  --config_arm_boost arm_boost=1 \
  --config_otg_mode otg_mode=1 \
  --config_pcie dtparam=pciex1 \
  --config_pcie_gen dtparam=pciex1_gen=3 \
  --config_usb_power usb_max_current_enable=1 \
  --username skywalker \
  --password skywalker \
  --extra_packages ""


cd ~/programming/gentoo/arm64/raspberry_pi_5/pi_installer
cargo build --release
mv target/release/pi_installer .


sudo ./pi_installer --target_drive /dev/sdb --boot_size 1G --swap_size 8G --stage3_url https://distfiles.gentoo.org/releases/arm64/autobuilds/latest-stage3-arm64-desktop-openrc.tar.xz --portage_snapshot_url https://distfiles.gentoo.org/snapshots/portage-latest.tar.bz2 --hostname my-gentoo-pi --root_password_hash '$6$.KYgMi02hVG4MNRk$1y6XS8QuIWEsqZNj6VFL9q9vMbItPkzPRi.Uh4/iiPIihsrx7ky23Rrwt.44IrkA76cx2HOrxrrMOOvz6TK6A/' --cmdline_console console=tty1 --cmdline_extra 'dwc_otg.lpm_enable=0 rootfstype=ext4 rootwait cma=256M@256M net.ifnames=0' --config_audio dtparam=audio=on --config_overlay dtoverlay=vc4-kms-v3d --config_max_framebuffers max_framebuffers=2 --config_fw_kms_setup disable_fw_kms_setup=1 --config_64bit arm_64bit=1 --config_overscan disable_overscan=1 --config_arm_boost arm_boost=1 --config_otg_mode otg_mode=1 --config_pcie dtparam=pciex1 --config_pcie_gen dtparam=pciex1_gen=3 --config_usb_power usb_max_current_enable=1 --username myuser --password mypassword --extra_packages 'vim git'