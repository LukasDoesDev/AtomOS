# AtomOS
## Tasks done so far
### Stage 1
* Add Rustup target `thumbv7em-none-eabihf`
* Switch to the `nightly` target (Done in rust-toolchain.toml)
* Add the target specification JSON file
* Try to build with `cargo build --target x86_64-atom_os.json`
* ``error[E0463]: can't find crate for `core` ``
* Add `.cargo/config.toml` to fix everything
* Build with `cargo build` (It works!)
### Stage 2
* Add the `bootloader` create
* Install `bootimage` from Cargo globally.
* Add the Rustup component `llvm-tools-preview`.
* Run `cargo bootimage`
* Run the OS in QEMU with the following command.
  ```bash
  qemu-system-x86_64 -drive format=raw,file=target/x86_64-atom_os/debug/bootimage-atom_os.bin
  ```
### Stage 3
* Create VGA buffer implementation