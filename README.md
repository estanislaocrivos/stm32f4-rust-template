# 🦀 Bare Metal STM32F4 Rust Template

This is a template project for embedded Rust development on the **STM32F4** microcontroller family. It provides a minimal but complete setup to start writing `no_std` Rust code on Cortex-M microcontrollers. Based on [The Rusty Bits](https://www.youtube.com/@therustybits) tutorial series.

## Getting Started

### Prerequisites

An `install.sh` script is provided to automate the installation of the required tools and dependencies. You can run it with:

```sh
./install.sh
```

If you prefer to set up the development environment manually, follow these steps:

1. **Install the Rust toolchain:**

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install target support:**

   You must add the target architecture to your Rust toolchain by running the following command:

   ```sh
   rustup target add thumbv7em-none-eabihf
   ```

   You can list all available targets by running:

   ```sh
   rustup show
   ```

   You may find more information about the available targets on the [Rust platform support page](https://doc.rust-lang.org/beta/rustc/platform-support.html).

3. **Install `probe-rs`:**

   `probe-rs` is a tool for flashing and debugging embedded devices. You can install it using Cargo:

   ```sh
   cargo install probe-rs
   ```

4. **Install `llvm-tools` for `cargo-binutils`:**

   As optional tools for working with LLVM and binary utilities, you can install `llvm-tools` and `cargo-binutils`. These tools are useful for inspecting and manipulating binaries, which can be helpful in embedded development.

   ```sh
   rustup component add llvm-tools
   ```

   ```sh
    cargo install cargo-binutils
   ```

5. **Install `cargo-embed`:**

   ```sh
   cargo install cargo-embed
   ```

### Building the Project

To check the project for errors, run:

```sh
cargo check
```

To build the project, run the following command in the terminal:

```sh
cargo build
```

### Flashing the firmware to the target device

To flash the firmware to the STM32F4 microcontroller, you can use the `cargo flash` command. This command uses `probe-rs` to connect to the device and upload the compiled binary. The chip name must be specified, and you can use the `--release` flag to build the project in release mode.

You can check the available STM32F4 platforms supported by `probe-rs` by running:

```sh
probe-rs chip list | grep STM32F4
```

Before flashing, ensure that the flash and RAM memory regions specified in your `memory.x` file match the memory map of your target STM32F4 device. This is important to avoid issues during flashing and runtime. In this template, the `memory.x` file is configured for the STM32F401RE microcontroller. To flash the firmware, run the following command:

```sh
cargo flash --release --chip STM32F401RE
```

Replace `STM32F401RE` with the appropriate chip name for your target device if you are using a different STM32F4 microcontroller.

## Porting this template to other ARM platforms

- **Adding support for other ARM targets:**

   When developing for embedded targets, you need to tell Rust which target architecture you are compiling for. For example, the STM32F4 family of microcontrollers are based on the ARM Cortex-M4 architecture, which implements the ARMv7E-M architecture. You can find out the architecture of your ARM microcontroller by searching its model on the [ARM developer website](https://developer.arm.com/). On Rust's [platform support page](https://doc.rust-lang.org/beta/rustc/platform-support.html) you can find the corresponding target for your architecture. In this case, the target for ARMv7E-M architecture is `thumbv7em-none-eabihf`, which also supports hardware floating point operations. Target's name is usually in the format `<arch><sub>-<vendor>-<sys>-<env>`, where:

  - `<arch>`: The architecture of the target (e.g., `thumb` for ARM Cortex-M4).
  - `<sub>`: The sub-architecture or variant (e.g., `v7em` for ARMv7E-M).
  - `<vendor>` [optional]: The vendor of the target (e.g., `eabi` for the Embedded Application Binary Interface).
  - `<sys>`: The operating system or environment (e.g., `none` for bare-metal targets).
  - `<env>`: The environment or ABI (e.g., `eabi` for the Embedded ABI).

   You can add target support by running the following command:

   ```bash
      rustup target add thumbv7em-none-eabihf
   ```

   Indicating the target inside the `.cargo/config.toml` file tells Cargo to use this target when building the project, and allows you to simply run `cargo build` without specifying the target each time.

   If using the `rust-analyzer` extension in Visual Studio Code, you may add the corresponding target to your `.vscode/settings.json` file:

   ```json
   {
    "rust-analyzer.check.allTargets": false,
    "rust-analyzer.cargo.target": "thumbv7em-none-eabihf"
   }  
   ```

   This ensures that it uses the correct target for code analysis and checks.

- **The cortex-m-rt crate:**

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
