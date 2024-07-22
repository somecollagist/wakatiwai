# Wakatiwai testing suite

<h2>Table of Contents</h2>

- [Wakatiwai testing suite](#wakatiwai-testing-suite)
	- [`wakatiwai.sh`](#wakatiwaish)
		- [Supported Filesystems](#supported-filesystems)
	- [`run.sh`](#runsh)

## `wakatiwai.sh`
The `wakatiwai.sh` script can be used to quickly generate disk images containing the Wakatiwai Bootloader. Its usage is as follows:
```
usage: wakatiwai.sh [-hl] <profile>
	-h  --help        : Prints this message
	-l  --list        : Lists profiles
```

In order to facilitate this, `.json` files in the `tests` directory exist which describe how a disk image should be formatted and populated, as well as providing a config file for Wakatiwai itself. The profile itself is a simple JSON object containing two fields; `config`, which contains a config object as would be found in a `wtconfig.json` file, and an array called `partitions`, itself containing objects with the following fields:

|Property|Example|Notes|
|---|---|---|
|`type`|`20`, `F90358A9-1AA5-4264-8C74-C298A4B801B1`, `BOOT`|This is either a GPT Partition Type identifier as used by `fdisk` or a a type GUID for this partition. See [`fdisk_gpt_table.md`](core/fdisk_gpt_table.md) for a complete list.<br>**N.B. This field may also be "BOOT". If so, all other fields bar `size` will be ignored, and this partition will be automatically configured with the Wakatiwai bootloader.**|
|`size`|`64M`, `300M`, `8G`|The size of the partition in either Megabytes (M) or Gigabytes (G = 1024M).|
|`fs`|`ext2`, `fat32`, `swap`|The filesystem to be used by the partition. Click [here](#supported-filesystems) for a list of currently supported filesystems.|


### Supported Filesystems
 - ext2/3/4
 - fat12/16/32
 - swap
 - btrfs

## `run.sh`

To run an disk image in QEMU, simply run `./run.sh <disk-image>.img`.