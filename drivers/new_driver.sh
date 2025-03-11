#!/bin/bash

DRIVERS_DIR=$(realpath $(dirname $0))

# Get the type of driver needed
select DRIVER_TYPE in "OS Boot Driver" "File System Driver"; do
    case $DRIVER_TYPE in
        "OS Boot Driver")       DRIVER_TYPE="BOOT"; break;;
        "File System Driver")   DRIVER_TYPE="FS"; break;;
        *) ;;
    esac
done

# Go to ordained directory
if [ $DRIVER_TYPE = "BOOT" ]; then
    cd $DRIVERS_DIR/boot
elif [ $DRIVER_TYPE = "FS" ]; then
    cd $DRIVERS_DIR/fs
fi

# Get name of driver
read -p "Name of new driver: " DRIVER_NAME
cargo new $DRIVER_NAME
if [ $? -ne 0 ]; then
    echo "Invalid driver name!"
    exit $?
fi

# Setup driver
cd $DRIVER_NAME

# Install dependencies
cargo add uefi@^0.34 --features panic_handler
cargo add uefi-raw@^0.10
cargo add --path $DRIVERS_DIR/../springboard

# Set target
mkdir .cargo
cp ../default_config.toml .cargo/

# Populate driver main
cat ../default_main.rs > src/main.rs