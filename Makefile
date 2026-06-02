ARMGNU ?= aarch64-none-elf
BOOTMNT ?= /Volumes/bootfs

default:
	cargo clean
	cargo build --release

	$(ARMGNU)-objcopy -O binary target/aarch64-unknown-none/release/flora build/kernel8.img # transform elf file to img

	cp -r build/kernel8.img $(BOOTMNT)/
	cp -r config.txt $(BOOTMNT)/

	sync