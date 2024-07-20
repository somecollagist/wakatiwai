#!/bin/bash

if [ "$(id -u)" -ne 0 ]; then
	echo "This script must be run as root." 1>&2
	exit 1
fi

pacman -Syy
pacman -S dosfstools efibootmgr jq ovmf qemu-desktop rustup --needed --noconfirm

rustup default nightly
rustup target install x86_64-unknown-uefi
rustup target add x86_64-unknown-uefi
rustup default nightly
rustup update

cp -r /usr/share/OVMF/x64 OVMF/
chown -R --reference=.git/ OVMF/