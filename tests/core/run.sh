qemu-system-x86_64					\
	-L OVMF/                	\
	-pflash OVMF/OVMF.4m.fd		\
	-net none               	\
	-usb $1                 	\
	-vga std                	\
	-enable-kvm								\
	-cpu host									\
	-m 4G