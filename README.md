# Wakatiwai
<p align="right">A simple and configurable UEFI bootloader, written in Rust.</p>

<h2>Table of Contents</h2>

- [Wakatiwai](#wakatiwai)
	- [About](#about)
	- [Contribution](#contribution)
	- [Testing](#testing)

## About
Wakatiwai (named after the Māori watercraft "waka tīwai", a simple canoe) is a bootloader for x86_64 UEFI written in Rust. It is engineered to support booting the Eisen operating system.

## Contribution
Contributions are more than welcome and will be processed whenever possible. Please adhere to the following guidelines:
 - Use British English
 - Avoid using programming languages other than Rust - inline assembly is acceptable where absolutely crucial
 - Tabs, not spaces
 - LF, not CRLF

## Testing
The `tests` directory contains some useful scripts for testing out Wakatiwai as well as creating disk images. You can find them documented [here](tests/README.md).