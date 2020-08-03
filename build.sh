./clean.sh
RUSTFLAGS="-C link-arg=-Tlinker.ld -C target-cpu=cortex-a53" cargo rustc --target=aarch64-unknown-none --release
rust-objcopy --strip-all -O binary target/aarch64-unknown-none/release/kos kernel8.img
qemu-system-aarch64 -M raspi3 -kernel kernel8.img -serial stdio