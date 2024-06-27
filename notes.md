#!/bin/bash

# Example command to start the Pi installer with all possible arguments
sudo ./pi_installer \
    --hostname gentoo-pi5-router \
    --target_drive /dev/sda \
    --boot_size 1G \
    --swap_size 8G \
    --stage3_url https://distfiles.gentoo.org/releases/arm64/autobuilds/20240623T231913Z/stage3-arm64-desktop-openrc-20240623T231913Z.tar.xz \
    --portage_snapshot_url https://distfiles.gentoo.org/snapshots/portage-latest.tar.bz2 \
    --root_password_hash '$6$.KYgMi02hVG4MNRk$1y6XS8QuIWEsqZNj6VFL9q9vMbItPkzPRi.Uh4/iiPIihsrx7ky23Rrwt.44IrkA76cx2HOrxrrMOOvz6TK6A/' \
    --cmdline_console 'console=serial0,115200 console=tty1' \
    --cmdline_extra 'rootfstype=ext4 fsck.repair=yes rootwait splash plymouth.ignore-serial-consoles' \
    --config_audio 'dtparam=audio=on' \
    --config_overlay 'dtoverlay=vc4-kms-v3d' \
    --config_max_framebuffers 'max_framebuffers=2' \
    --config_fw_kms_setup 'disable_fw_kms_setup=1' \
    --config_64bit 'arm_64bit=1' \
    --config_overscan 'disable_overscan=1' \
    --config_arm_boost 'arm_boost=1' \
    --config_otg_mode 'otg_mode=1' \
    --config_pcie 'dtparam=pciex1' \
    --config_pcie_gen 'dtparam=pciex1_gen=3' \
    --config_usb_power 'usb_max_current_enable=1' \
    --username skywalker \
    --password skywalker \
    --extra_packages 'dev-vcs/git app-editors/vim' \
    --timezone America/New_York \
    --ssh_key 'your-ssh-public-key-here'
