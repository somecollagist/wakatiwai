# Wakatiwai
<p align="right">A simple and configurable UEFI bootloader, written in Rust.</p>

<h2>Table of Contents</h2>

- [Wakatiwai](#wakatiwai)
	- [About](#about)
	- [`wtconfig.json`](#wtconfigjson)
	- [Contribution](#contribution)
	- [Testing](#testing)

## About
Wakatiwai (named after the Māori watercraft "waka tīwai", a simple canoe) is a bootloader for x86_64 UEFI written in Rust. It is engineered to support booting the Eisen operating system.

## Installation
Wakatiwai needs to occupy an EFI System partition on your disk. This partition:
 - must be formatted to a FAT32 filesystem
 - must be large enough to function as an EFI System partition (At least 64MB but 300MB-1GB is usually recommended)
 - should be the first partition on disk - this is not necessarily required but is the safest and most conventional position

## `wtconfig.json`
The Wakatiwai Bootloader is configured via a file called `wtconfig.json`, located in the root of the EFI partition in which the bootloader resides. It may take the following **case-sensitive** properties and values:

(N.B. fields without a default value **must** be provided.)
|Property|Type|Default|Notes|
|---|---|---|---|
|`loglevel`|String|`"NORMAL"`|Describes how much logging information will be outputted by the bootloader. Options are: <ul><li>`"SILENT"` (Outputs critical failures only)</li><li>`"QUIET"` (Outputs critical failures and warnings only)</li><li>`"NORMAL"` (Outputs regular messages)</li><li> `"DEBUG"` (Outputs debug messages)</li></ul>|
|`timeout`|Integer|5|Amount of time in seconds to wait until booting the default boot entry. May also be set to 0 to immediately boot or to a negative number to wait for user input. **Integer must be a signed long integer (-2,147,483,648 to 2,147,483,647)**|
|`bootentries`|[BootEntry]|N/A|An array of boot entries to be used by the bootloader. They will be booted preferentially from the start of the array.|

### Boot Entries
Boot entries are themselves respresented in JSON within the `bootentries` array of the `wtconfig.json`. They may take the following **case-sensitive** properties and values:

|Property|Type|Default|Notes|
|---|---|---|---|
|`name`|String|N/A|The name of the boot entry.|
|`partition`|Integer|N/A|The partition in which this boot option resides.|

## Contribution
Contributions are more than welcome and will be processed whenever possible. Please adhere to the following guidelines:
 - Use British English
 - Avoid using programming languages other than Rust - inline assembly is acceptable where absolutely crucial
 - Tabs, not spaces
 - LF, not CRLF

## Testing
The `tests` directory contains some useful scripts for testing out Wakatiwai as well as creating disk images. You can find them documented [here](tests/README.md).