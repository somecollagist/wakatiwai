qemu-system-x86_64          \
	-L OVMF/                \
	-pflash OVMF/OVMF.fd    \
	-net none               \
	-usb $1                 \
	-vga std                \
	-m 256M