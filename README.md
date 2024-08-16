# Wakatiwai
<p align="right">A simple and configurable UEFI bootloader, written in Rust.</p>

![GitHub License](https://img.shields.io/github/license/somecollagist/wakatiwai) ![GitHub last commit](https://img.shields.io/github/last-commit/somecollagist/wakatiwai)



<h2>Table of Contents</h2>

- [Wakatiwai](#wakatiwai)
	- [About](#about)
	- [Features](#features)
	- [Installation](#installation)
	- [Using Wakatiwai](#using-wakatiwai)
	- [`wtconfig.json`](#wtconfigjson)
		- [Boot Entries](#boot-entries)
		- [Supported Filesystems](#supported-filesystems)
	- [Contribution](#contribution)
		- [Languages](#languages)
	- [Testing](#testing)

## About
Wakatiwai (named after the Māori watercraft "waka tīwai", a simple canoe) is a bootloader for x86_64 UEFI written in Rust. It is engineered to support booting the Eisen operating system.

## Features
<b><h3><p>
	⚙️ Easy JSON Configuration<br>
	📠 Customisable Logging<br>
	🤝 UEFI and Flat Binary Booting<br>
	📁 [Multiple File Systems](#supported-filesystems)<br>
	🪟 Windows Support<br>
	🐧 Linux Support (for booting via systemd)<br>
</p></h3></b>

## Installation
Wakatiwai needs to occupy an EFI System partition on your disk. This partition:
 - must be formatted to a FAT32 filesystem
 - must be large enough to function as an EFI System partition (At least 64MB but 300MB-1GB is usually recommended)
 - should be the first partition on disk - this is not necessarily required but is the safest and most conventional position

## Using Wakatiwai
Upon starting the bootloader, you will be greeted with a list of menu options. Use the up and down keys to focus one of these options, and press the space or enter keys to boot the focused option.

The following keys also have functions:

|Key|Function|
|---|---|
|F5|Restarts the bootloader.|
|F12|Powers off the system.|

## `wtconfig.json`
The Wakatiwai Bootloader is configured via a file called `wtconfig.json`, located in the root of the EFI partition in which the bootloader resides. It takes the following **case-sensitive** properties and values:

|Property|Type|Default|Required|Notes|
|---|---|---|---|---|
|`loglevel`|String|`"NORMAL"`|✘|Describes how much logging information will be outputted by the bootloader. Options are: <ul><li>`"SILENT"` (Outputs errors only)</li><li>`"QUIET"` (Outputs errors and warnings only)</li><li>`"NORMAL"` (Outputs regular messages)</li><li> `"DEBUG"` (Outputs debug messages)</li></ul>|
|`timeout`|Integer|5|✘|Amount of time in seconds to wait until booting the default boot entry. May also be set to 0 to immediately boot or to a negative number to wait for user input.<br><br>**N.B. This must be a signed long integer (-2,147,483,648 to 2,147,483,647).**|
|`offerexit`|Boolean|`true`|✘|If `true`, the bootloader will present the option to exit the bootloader in the boot menu.<br><br>**N.B. This can be useful to enter the UEFI (BIOS) Menu (or Shell, if supported).**|
|`editconfig`|Boolean|`true`|✘|If `true`, the bootloader will present the option to edit the local `wtconfig.json` for future boots in the boot menu.<br><br>**WARNING: If set to `false`, mistakes in the bootloader's configuration might only be fixable from another operating system - your system may become unbootable.**|
|`menuclear`|Boolean|`true`|✘|If `true`, the screen will be cleared when the boot menu is displayed.|
|`bootentries`|[BootEntry]|N/A|✘|An array of boot entries to be used by the bootloader. They will be booted preferentially from the start of the array.<br><br>**N.B. If left blank, the bootloader will emit an appropriate warning and automatically offer the user the option to access the UEFI shell or edit the bootloader configuration file.**|

### Boot Entries
Boot entries are themselves respresented in JSON within the `bootentries` array of the `wtconfig.json`. They take the following **case-sensitive** properties and values:

|Property|Type|Default|Required|Notes|
|---|---|---|---|---|
|`name`|String|N/A|✔|The name of the boot entry.<br><br>**N.B. This name should be no longer than 64 characters.**|
|`diskguid`|String|The Bootloader's disk GUID.|✘|The GUID of the GPT of the disk upon which this boot option resides.|
|`partition`|Integer|N/A|✔|The partition in which this boot option resides.|
|`fs`|String|N/A|✔|The file system of the given partition. A list of supported filesystems (case sensitive, in quotes) can be found [here](#supported-filesystems).|
|`progtype`|String|N/A|✔|The type of program this boot entry points to. Options are: <ul><li>`"UEFI"` (Boot entry runs a UEFI program)</li><li>`"ELF"` (Boot entry runs an ELF executable)</li></ul>|
|`path`|String|N/A|✔|The path of the program this boot entry points to.|
|`initrd`|String|N/A|✘|The path of the ramdisk this boot entry should preload.|
|`args`|String|N/A|✘|A list of arguments to be passed into the boot entry.|

### Supported Filesystems
 - fat12
 - fat16
 - fat32

## Contribution
Contributions are more than welcome and will be processed whenever possible. Please adhere to the following guidelines:
 - Avoid using programming languages other than Rust - inline assembly is acceptable where absolutely crucial
 - LF, not CRLF

### Languages
This project will ideally be multi-lingual. As such, translations would be appreciated for any natural-language material produced (e.g. Markdown files, manuals, messages, etc.). UEFI only outputs characters in extended ASCII (see [ISO/IEC 8859-1:1998](https://en.wikipedia.org/wiki/ISO/IEC_8859-1)), so only languages containing these glyphs can be properly supported.

|Language|Status (✅ - Fully supported, ❌ - Pending Support, 🚧 - In progress)|
|---|---|
|Afrikaans (Afrikaans)|❌|
|Albanian (Shqip)|❌|
|Basque (Euskara)|❌|
|British English (British English)|✅|
|Catalan (Català)|❌|
|Danish (Dansk)|❌|
|Dutch (Nederlands)|❌|
|Estonian (Eesti)|❌|
|Finnish (Suomi)|❌|
|French (Français)|❌|
|German (Deutsch)|🚧|
|Icelandic (Íslenska)|❌|
|Irish (Gaeilge)|❌|
|Indonesian (Bahasa Indonesia)|❌|
|Italian (Italiano)|❌|
|[Klingon (thlIngan Hol)](docs/tlh/README.md)|🚧|
|Malay (Bahasa Melayu)|❌|
|Norwegian (Norsk)|❌|
|Portugese (Português)|❌|
|Romansh (Romontsch)|❌|
|Scottish Gaelic (Gàidhlig)|❌|
|Spanish (Español)|❌|
|Swahili (Kiswahili)|❌|
|Swedish (Svenska)|❌|
|Tagalog (Wikang Tagalog)|❌|

## Testing
The `tests` directory contains some useful scripts for testing out Wakatiwai as well as creating disk images. You can find them documented [here](tests/README.md).