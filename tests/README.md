# Wakatiwai testing suite

## `wakatiwai.sh`
The `wakatiwai.sh` script can be used to quickly generate disk images containing the Wakatiwai Bootloader. Its usage is as follows:
```
usage: wakatiwai.sh [-hl] <profile>
    -h  --help        : Prints this message
    -l  --list        : Lists profiles
```

In order to facilitate this, files with the extension `.wtprof` exist in the `tests` directory of the Wakatiwai repository. These are called profiles and when supplied to the aforementioned program, they will be used to create a specifed disk image. These profiles are essentially [tab-separated-value](https://en.wikipedia.org/wiki/Tab-separated_values) files whose lines adhere to the following format, though they should also contain a [boot partition](#boot-partition) (typically at the start).

```<type/guid>	<size>	<fs>```

### Type/Guid
This is either a GPT Partition Type identifier as used by `fdisk` or a a type GUID for this partition. See [`fdisk_gpt_table.md`](core/fdisk_gpt_table.md) for a complete list.

### Size
The size of the partition in either Megabytes (M) or Gigabytes (G = 1024M).

### FS
The filesystem to be used by the partition. Currently supported filesystems include:

- ext2/3/4
- FAT12/16/32
- swap
- btrfs

### Boot Partition
Since this script is designed to create images for the Wakatiwai Bootloader, the following line may be placed within the profile to automatically populate a partition with the Wakatiwai Bootloader:

```BOOT	<size>```

where the `<size>` parameter acts [as described above](#size).

## `run.sh`

To run an disk image in QEMU, simply run `./run.sh <disk-image>.img`.