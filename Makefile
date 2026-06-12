ARMGNU ?= aarch64-none-elf
BOOTMNT ?= /Volumes/bootfs

.PHONY: default armstub clean

default: armstub | build
	cargo clean
	cargo build --release

	$(ARMGNU)-objcopy -O binary target/aarch64-unknown-none/release/flora build/kernel8.img # transform elf file to img

	cp -r build/kernel8.img $(BOOTMNT)/
	cp -r config.txt $(BOOTMNT)/

	sync

build:
	mkdir -p build # create build folder

build/armstub_s.o: src/board/armstub.s
	mkdir -p $(@D)
	$(ARMGNU)-as $< -o $@

armstub: build/armstub_s.o | build
	$(ARMGNU)-ld --section-start=.text=0 -o build/armstub.elf build/armstub_s.o # build armstub
	$(ARMGNU)-objcopy build/armstub.elf -O binary build/armstub.bin

	cp build/armstub.bin $(BOOTMNT)/

clean:
	cargo clean
	rm -rf build