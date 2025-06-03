#!/bin/bash

PROJ_DIR=$(realpath $(dirname $0)/..)

if [ "$(id -u)" -ne 0 ]; then
	echo "This script must be run as root." 1>&2
	exit 1
fi

pacman -Syy
pacman -S base-devel dosfstools efibootmgr jq lsb-release ovmf qemu-desktop rustup --needed --noconfirm

rustup default nightly
rustup target install x86_64-unknown-uefi
rustup target add x86_64-unknown-uefi
rustup default nightly
rustup update

cp -r /usr/share/OVMF/x64 $PROJ_DIR/OVMF/
chown -R --reference=$PROJ_DIR/.git/ $PROJ_DIR/OVMF/

for script in $(ls $PROJ_DIR/scripts); do
	ln -s $PROJ_DIR/scripts/$script /usr/local/bin/wakatiwai-$script
done

mkdir /etc/wakatiwai
cp $PROJ_DIR/drivers/driverlist /etc/wakatiwai/driverlist