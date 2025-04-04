#!/bin/bash

DRIVERS_DIR=$(realpath $(dirname $0))
BOOT_DRIVERS_DIR=$DRIVERS_DIR/boot
FS_DRIVERS_DIR=$DRIVERS_DIR/fs

OUT=$(realpath $DRIVERS_DIR/../out)/drivers
BOOT_OUT=$OUT/boot
FS_OUT=$OUT/fs

# remove unnecessary .pdb files
rm -f $OUT
mkdir -p $BOOT_OUT
mkdir -p $FS_OUT

build_drivers() {
    for DRIVER in $(ls -d $1/*/); do
        cd $DRIVER
        cargo build --profile release --artifact-dir $2 -Z unstable-options
        BUILD_ERR_CODE=$?
        if [ $BUILD_ERR_CODE -ne 0 ]; then
            exit $BUILD_ERR_CODE
        fi
    done
}

build_drivers $BOOT_DRIVERS_DIR $BOOT_OUT
build_drivers $FS_DRIVERS_DIR $FS_OUT

# remove unnecessary .pdb files
rm -f $BOOT_OUT*.pdb
rm -f $FS_OUT*.pdb