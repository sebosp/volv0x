# Volv0X distributed embedded systems

## Status

Used as module inside the sebosp/esp-wifi fork.

https://youtu.be/feOZ0bY691I

[![Youtube Project Overview](http://img.youtube.com/vi/feOZ0bY691I/0.jpg)](https://youtu.be/feOZ0bY691I) 


## Requirements

- `rustup`
- `espup`

## Flow

Compilen on a beefy server, example for esp32c6 it's 

### Building on the compiling server
```
[seb@corvid esp32]$ .  /home/seb/export-esp.sh
[seb@corvid esp32]$ export CARGO_TARGET_DIR=/tmp/esp32 # To have cargo watch not be blocked by nvim/rust-analyzer/etc
[seb@corvid esp32]$ cargo watch -x "build --release --no-default-features --features=esp32c6 --target=riscv32imac-unknown-none-elf --bin ble_bas_peripheral"
```

### Flashing on your laptop/etc.

```
[seb@shadesmar esp32]$ .  /home/seb/export-esp.sh
[seb@shadesmar esp32]$ cargo watch -w target/riscv32imac-unknown-none-elf/release/ble_bas_peripheral -x "run --release --no-default-features --features=esp32c6 --target=riscv32imac-unknown-none-elf --bin ble_bas_peripheral"
```
