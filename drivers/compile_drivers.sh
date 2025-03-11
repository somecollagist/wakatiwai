#!/bin/bash

DRIVERS_DIR=$(realpath $(dirname $0))
BOOT_DRIVERS_DIR=$DRIVERS_DIR/boot
FS_DRIVERS_DIR=$DRIVERS_DIR/fs

OUT=$(realpath $DRIVERS_DIR/../out)
mkdir -p $OUT/drivers/boot
mkdir -p $OUT/drivers/fs

for BOOT_DRIVER in $(ls -d $BOOT_DRIVERS_DIR/*/); do
    cd $BOOT_DRIVER
    cargo build --profile release --artifact-dir $OUT/drivers/boot -Z unstable-options
    BUILD_ERR_CODE=$?
    if [ $BUILD_ERR_CODE -ne 0 ]; then
        exit $BUILD_ERR_CODE
    fi
done

for FS_DRIVER in $(ls -d $FS_DRIVERS_DIR/*/); do
    cd $FS_DRIVER
    cargo build --profile release --artifact-dir $OUT/drivers/fs -Z unstable-options
    BUILD_ERR_CODE=$?
    if [ $BUILD_ERR_CODE -ne 0 ]; then
        exit $BUILD_ERR_CODE
    fi
done

# remove unnecessary .pdb files
rm -f $OUT/drivers/boot/*.pdb
rm -f $OUT/drivers/fs/*.pdb