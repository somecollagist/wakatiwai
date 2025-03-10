#!/bin/bash

usage() {
	cat << HELP_USAGE
usage: install.sh <ESP>
	ESP           : The mount point of the EFI System Partition

Licensed under GPLv3
Copyright (C) 2024  Johann Scott
HELP_USAGE
	exit 0
}

if [ "$(id -u)" -ne 0 ]; then
	echo "This script must be run as root." 1>&2
	exit 1
fi

if [ "$1" = "" ]; then
	usage
	exit 1
fi

PROJ_DIR=$(realpath $(dirname $0))

cd $PROJ_DIR
cargo build --profile release --artifact-dir ./out -Z unstable-options
BUILD_ERR_CODE=$?
if [ $BUILD_ERR_CODE -ne 0 ]; then
	exit $BUILD_ERR_CODE
fi

echo "Built Wakatiwai bootloader"

ESP_MNT=$1
ESP_DISK_PART=$(findmnt $ESP_MNT -no SOURCE | grep -oE '[0-9]+$')
ESP_DISK_DEV=$(findmnt $ESP_MNT -no SOURCE | sed "s/p$ESP_DISK_PART$//")

sudo mkdir -p "$ESP_MNT/EFI/wakatiwai"
sudo cp ./out/wakatiwai.efi "$ESP_MNT/EFI/wakatiwai/wakatiwai.efi"
echo "Copied bootloader program"

CURRENT_WAKATIWAI=$(efibootmgr | grep -oE "Boot[0-9]{4}\\* Wakatiwai" | grep -oE "[0-9]{4}")
if [ $? -eq 0 ]; then
	# Wakatiwai exists as a boot option - clean it
	efibootmgr -B -b $CURRENT_WAKATIWAI
	echo "Removed duplicate boot entry (Boot$CURRENT_WAKATIWAI)"
fi

efibootmgr -c -L "Wakatiwai" -l "\EFI\wakatiwai\wakatiwai.efi" -d $ESP_DISK_DEV -p $ESP_DISK_PART
echo "Successfully added boot entry"