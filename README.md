# Wakatiwai
<p align="right">A simple and configurable UEFI boot manager, written in Rust.</p>

![GitHub License](https://img.shields.io/github/license/somecollagist/wakatiwai) ![GitHub last commit](https://img.shields.io/github/last-commit/somecollagist/wakatiwai)

<h2>Table of Contents</h2>

- [Wakatiwai](#wakatiwai)
  - [About](#about)
  - [Features](#features)
  - [Download](#download)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Configuration](#configuration)
    - [Supported File Systems](#supported-file-systems)
    - [Supported Operating Systems](#supported-operating-systems)
  - [Other Tools](#other-tools)

## About
Wakatiwai is a boot manager for x86_64 UEFI, written in Rust. It supports any file system and any operating system by using runtime drivers.

## Features
 - **⚙️ Easy JSON Configuration**
 - **📁 [Multiple File Systems](#supported-filesystems)**
 - **💿 [Multiple Operating Systems](#supported-program-types)**
 - **🤝 Dynamic driver interop**

## Download
Wakatiwai is currently available for the following Linux distros:
 - Arch Linux (AUR)

Alternatively, you can manually build Wakatiwai from source, by cloning and running `./setup/setup-<OS>.sh`, where `<OS>` is the operating system/Linux distribution you use (or a derivative, i.e. use `arch` for Manjaro).

(Missing a setup script for your OS? Make one and submit a pull request!)

## Installation
Wakatiwai can be installed on your machine by running:

```$ sudo wakatiwai-install <ESP> [driverlist]```

Where `<ESP>` is the mountpoint of the EFI system partition you wish to install Wakatiwai to. Note that this partition must be at least 300MB in size and be formatted to FAT32. `[driverlist]` is an optional argument that defaults to `/etc/wakatiwai/driverlist` - the given file will list the official and custom drivers to be built and used by the bootloader.

The `/EFI/wakatiwai` directory will be created within `<ESP>` and will contain the following:
 - `wakatiwai.efi` - the boot manager itself
 - `wtconfig.json` - the [configuration file](#configuration) for the boot manager
 - `drivers/boot/` - a directory containing OS loader drivers
 - `drivers/fs/` - a directory containing file system drivers

Note that custom drivers can be created and placed in these directories to be used by the boot manager (submit a PR for official support!).

## Usage
Upon starting the boot manager, you will be greeted with a list of menu options. Use the up and down keys to focus one of these options, and press the space or enter keys to boot the focused option.

The following keys also have functions:

| Key | Function                 |
| --- | ------------------------ |
| F5  | Restarts the boot manager. |
| F12 | Powers off the system.   |

## Configuration
The aforementioned `wtconfig.json` file accepts the following **case-sensitive** properties and values:

| Property      | Type        | Default    | Required | Notes                                                                                                                                                                                                                                                                                                                 |
| ------------- | ----------- | ---------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `loglevel`    | String      | `"NORMAL"` | ✘        | Describes how much logging information will be outputted by the boot manager. Options are: <ul><li>`"SILENT"` (Outputs errors only)</li><li>`"QUIET"` (Outputs errors and warnings only)</li><li>`"NORMAL"` (Outputs regular messages)</li><li> `"DEBUG"` (Outputs debug messages)</li></ul>                            |
| `timeout`     | Integer     | 5          | ✘        | Amount of time in seconds to wait until booting the default boot entry. May also be set to 0 to immediately boot or to a negative number to wait for user input.                                                                                                                                                      |
| `exit`        | Boolean     | `true`     | ✘        | If `true`, the boot manager will present the option to exit the boot manager in the boot menu.<br><br>                                                                                                                                                                                                                    |
| `firmware`    | Boolean     | `true`     | ✘        | If `true`, the boot manager will present the option to exit to the system's firmware UI on a reboot.<br><br>**N.B. This option will not be presented if the firmware does not support this action.**                                                                                                                    |
| `editconfig`  | Boolean     | `true`     | ✘        | If `true`, the boot manager will present the option to edit the local `wtconfig.json` for future boots in the boot menu.<br><br>**WARNING: If set to `false`, mistakes in the boot manager's configuration might only be fixable from another operating system - your system may become unbootable.**                     |
| `menuclear`   | Boolean     | `true`     | ✘        | If `true`, the screen will be cleared when the boot menu is displayed.                                                                                                                                                                                                                                                |
| `bootentries` | [BootEntry] | N/A        | ✘        | An array of boot entries to be used by the boot manager. They will be booted preferentially from the start of the array.<br><br>**N.B. If left blank, the boot manager will emit an appropriate warning and automatically offer the user the option to access the UEFI shell or edit the boot manager configuration file.** |

Boot entires are themselves represented as JSON objects and accept the following **case-sensitive** properties and values:

| Property    | Type    | Default                     | Required | Notes                                                                                                                                                       |
| ----------- | ------- | --------------------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`      | String  | N/A                         | ✔        | The name of the boot entry.<br><br>**N.B. This name should be no longer than 64 characters.**                                                               |
| `diskguid`  | String  | The Boot manager's disk GUID. | ✘        | The GUID of the GPT of the disk upon which this boot option resides.                                                                                        |
| `partition` | Integer | N/A                         | ✔        | The partition in which this boot option resides.                                                                                                            |
| `fstype`    | String  | N/A                         | ✔        | The file system of the given partition. A list of supported filesystems (case sensitive, in quotes) can be found [here](#supported-filesystems).            |
| `ostype`    | String  | N/A                         | ✔        | The type of program this boot entry points to. A list of supported program types (case sensitive, in quotes) can be found [here](#supported-program-types). |
| `path`      | String  | N/A                         | ✔        | The path of the program this boot entry points to.                                                                                                          |
| `args`      | String  | N/A                         | ✘        | Stringified arguments to be passed to the OS driver.                                                                                                        |

### Supported File Systems
- `FAT` - supports `FAT12`, `FAT16`, and `FAT32`

### Supported Operating Systems
- `UEFI` - any `.EFI` program

## Other Tools
- `wakatiwai-mkdriver` - creates the boilerplate for a new driver.