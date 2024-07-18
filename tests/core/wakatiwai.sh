#!/bin/bash

PROJ_DIR=$(realpath $(dirname $0)/../..)
TEST_DIR=$PROJ_DIR/tests

usage() {
	cat << HELP_USAGE
usage: wakatiwai.sh [-hlr] <profile>
	-h  --help      : Prints this message
	-l  --list      : Lists profiles
	-r              : Build in release mode

Licensed under GPLv3
Copyright (C) 2024  Johann Scott
HELP_USAGE
	exit 0
}

list_profiles() {
	cd $TEST_DIR
	echo "Available profiles:"
	ls -1 *.json | sed 's/\.json//' | sed 's/^/\t/'
	exit 0
}

PROFILE=""
RELEASE="dev"

for param in "$@"; do
	if [[ $param =~ "-" ]]; then
		case $param in
			"-h" | "--help")
				usage
				;;
			"-l" | "--list")
				list_profiles
				;;
			"-r" | "--release")
				RELEASE="release"
				;;
			*)
				echo "Unrecognised option \"$param\""
				usage
				;;
		esac
	else
		PROFILE=$param
		if [ ! -f "$PROJ_DIR/tests/$PROFILE.json" ]; then
			echo "No such profile or option \"$PROFILE\" found"
			exit 1
		else
			WTPROF="$PROJ_DIR/tests/$PROFILE.json"
			echo "Building image for profile \"$PROFILE\"..."
		fi
	fi
done

# Build bootloader
cd $PROJ_DIR
cargo build --profile $RELEASE --out-dir ./out -Z unstable-options
BUILD_ERR_CODE=$?
if [ $BUILD_ERR_CODE -ne 0 ]; then
	exit $BUILD_ERR_CODE
fi

#Build image
IMG="wakatiwai-$PROFILE.img"
echo "Outputting to $IMG"

# FS Variables
LODEV=$(losetup -f)
MNTPT="$PROJ_DIR/.tmpmnt"

# Image variables
IMG_SIZE_M=$($TEST_DIR/core/image_size.sh $WTPROF)
PARTITIONS=$(wc -l $WTPROF | grep -oe "^[0-9]*")

# Create and prepare image file
rm -f $IMG
dd if=/dev/zero of=$IMG bs=1M count=$IMG_SIZE_M >& /dev/null
$TEST_DIR/core/image_fdisk.sh $WTPROF | fdisk $IMG > /dev/null
echo "Image successfully formatted:"
fdisk -x $IMG

# Mount and populate file systems
sudo losetup -P $LODEV $IMG
mkdir $MNTPT
IFS=$'\n'
LOPC=1

function remove_mount_with_error() {
	echo "$1"
	sudo losetup -d $LODEV
	sudo rm -rf $MNTPT
	exit 1
}

function check_mkfs_installed() {
	command -v $1 > /dev/null
	if [ $? -ne 0 ]; then
		remove_mount_with_error "$1 is not installed"
	fi
}

while read -r part; do
	LOPT=$LODEV"p"$LOPC

	if [ $(echo $part | jq ".type") = "\"BOOT\"" ]; then
		sudo mkfs.fat -F 32 $LOPT > /dev/null
		sudo mount $LOPT $MNTPT
		
		sudo mkdir -p $MNTPT/EFI/BOOT
		sudo cp $PROJ_DIR/out/wakatiwai.efi $MNTPT/EFI/BOOT/BOOTX64.EFI
		jq ".config" $WTPROF > /tmp/wtconfig.json
		sudo cp /tmp/wtconfig.json $MNTPT/wtconfig.json

		echo "Created Wakatiwai boot partition on partition $LOPC"
		FS_TYPE="BOOT" # makes unmounting easier
	else
		FS_TYPE=$(echo $part | jq ".fs" | tr -d '"')
		case $FS_TYPE in
			ext2 | ext3 | ext4)
				sudo mkfs.$FS_TYPE $LOPT > /dev/null
				;;

			fat12 | fat16 | fat32)
				check_mkfs_installed mkfs.fat
				sudo mkfs.fat -F $(echo $FS_TYPE | cut -c4,5) $LOPT > /dev/null
				;;

			btrfs)
				check_mkfs_installed mkfs.btrfs
				sudo mkfs.btrfs -L "BTRFS Partition" $LOPT > /dev/null
				;;

			swap)
				sudo mkswap $LOPT > /dev/null
				;;

			*)
				remove_mount_with_error "Unrecognised file system \"$FS_TYPE\", exiting..."
				;;
		esac

		echo "Created $FS_TYPE filesystem on partition $LOPC"

		if [ $FS_TYPE != "swap" ]; then # swaps need swapon but are volatile so no point populating
			sudo mount $LOPT $MNTPT
		fi

		content=$(echo $part | jq -c ".content[]") >> /dev/null
		if [ $? -eq 0 ]; then
			for content_pair in $content; do
				from=$(echo $content_pair | jq ".from" | tr -d '"') >> /dev/null
				to=$(echo $content_pair | jq ".to" | tr -d '"') >> /dev/null
				to=$(realpath $(dirname "$MNTPT""$to"))
				echo "Copying $from to $to"
				sudo mkdir -p $to
				sudo cp -r $from $to
			done
		fi
	fi

	if [ $FS_TYPE != "swap" ]; then # as above, would be swapoff if needed
		sudo umount $LOPT
	fi

	LOPC=$((LOPC + 1))
done < <(jq -c ".partitions[]" $WTPROF)

sudo losetup -d $LODEV
sudo rm -rf $MNTPT