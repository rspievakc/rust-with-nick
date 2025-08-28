install rustup (Remove any rust installation done by the brew package manager first)
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
install espup
- For ubuntu and debian run:
```shell
sudo apt-get install -y gcc build-essential curl pkg-config
```
- Then:
```shell
cargo install espup --locked
espup install
```
Add `. /Users/rodrigo/export-esp.sh` to your shell profile or create an alias to initialize it on demand.

Install Toolchains
```
rustup target add xtensa-esp32s3-none-elf

```

Install espflash
```
cargo install espflash
```
Install probe-rs -- Debug probe to use the JTAG on the ESP32
```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```
Install cargo binutils
```shell
cargo install cargo-binutils
```
Install cargo generate:
```shell
cargo install cargo-generate
cargo install 
esp-generate
```

Install cargo [bininstall](https://github.com/cargo-bins/cargo-binstall):
```
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
```

Install rust-owl - Helps to visualise ownership and lifetimes
```
cargo bininstall rustowl
```

Recommended VSCode extensions:

Rust Analyser -> Verify semantics and dependencies
Dependi -> Help with toml files
Error Lens -> Show inline errors
CodeLLDB -> Debugging
Probe-RS -> Debugging
RustOwl VSCode -> Lifetimes and Borrowing visualiser

Create and example project
```
esp-generate --chip esp32s3 -o alloc -o wifi -o probe-rs -o embassy -o unstable-hal
cargo build
cargo run
```

Enabling the vscode launcher (.vscode/launch.json). Make sure to configure the log level or calls to the rtt are not going to send data.
```json
{

"version": "0.2.0",

"configurations": [

{

"type": "probe-rs-debug",

"request": "launch",

"name": "Launch",

"cwd": "${workspaceFolder}",

"preLaunchTask": "build-debug",

"chip": "esp32s3",

"flashingConfig": {

"flashingEnabled": true,

"haltAfterReset": true,

"formatOptions": {

"binaryFormat": "idf"

}

},

"coreConfigs": [

{

"coreIndex": 0,

"programBinary": "target/xtensa-esp32s3-none-elf/debug/${workspaceFolderBasename}",

"rttEnabled": true,

}

],

"env": {

//!MODIFY (or remove)

"RUST_LOG": "info" // If you set this variable, check the VSCode console log window for the location of the log file.

},

"consoleLogLevel": "Console" //Info, Debug

},

{

"type": "probe-rs-debug",

"request": "attach",

"name": "Attach",

"cwd": "${workspaceFolder}",

"chip": "esp32s3",

"coreConfigs": [

{

"coreIndex": 0,

"programBinary": "target/xtensa-esp32s3-none-elf/debug/${workspaceFolderBasename}",

"rttEnabled": true,

}

],

"env": {

//!MODIFY (or remove)

"RUST_LOG": "info" // If you set this variable, check the VSCode console log window for the location of the log file.

},

"consoleLogLevel": "Console" //Info, Debug

}

]

}
```


https://github.com/johnthagen/min-sized-rust

[Synchronisation primitives][(https://blog.theembeddedrustacean.com/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives)







