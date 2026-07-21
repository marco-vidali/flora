default:
	cargo clean

	cp -r uefi/* /Volumes/MICROSD/

	cargo build --release
	mkdir -p /Volumes/MICROSD/EFI/BOOT/
	cp target/aarch64-unknown-uefi/release/flora.efi /Volumes/MICROSD/EFI/BOOT/bootaa64.efi