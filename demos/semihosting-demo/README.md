# semihosting-demo

Minimal "hello world" demo using semihosting for communication between the embedded target and the host during a debug session

## Background

### What is semihosting?

Semihosting is a technique that allows embedded devices to communicate with the with host computer (e.g. print debug information to the host console). This technique requires the establishment of an active debugging session — code running on the embedded target (e.g. the microcontroller) communicates with the host or development machine through a hardware debugger (e.g. ST-Link). There is some overhead introduced to implement semihosting and it is not the best for time-critical tasks, thus it is mainly used for development and is usually disabled in production code. [^1] 

## Hardware Setup 

```
Dev machine (host): Lenovo T480 running Arch Linux
Development board (target): WeAct Studio STM32F411CEU6 BlackPill v3.1
HW debugger: ST-Link V2 
```

<img src="img/hw-debugger-setup.jpg">

*Black Pill development board with STM32 MCU connected to ST-Link V2 hardware debugger via SWD (serial wire debug) interface.*

SWD is a two-wire protocol used for debugging ARM-based microcontrollers.  In the picture above, there are (4) wires connecting the microcontroller to the debugger; technically only two of these wires are used for communication (SDIO and SWCLK), the other two wires (GND and VCC), are used to establish ground and to power the target device from the debugger or to inform the debugger of the target devices voltage levels.[^3]

## Configuration

```
embedded-rust-stm32/
├── .cargo/
│   └── config.toml
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── demos/
│   └── semihosting-demo/
│       ├── build.rs
│       ├── Cargo.toml
│       ├── openocd.cfg
│       ├── openocd.gdb
│       └── src/
│           └── main.rs
├── memory.x
└── target/
```

The following configuration files are required:
- `.cargo/config.toml`
- `memory.x`
  - specifies memory layout of embedded target
- `build.rs`
  - build script that points to `memory.x` and sets linker flags 
  - placing a file named build.rs in the root of a package will cause Cargo to compile that script and execute it just before building the package
- `openocd.cfg`
  - specifies location of openocd configuration files for HW debugger and the MCU 
  - saves the developer from including the relative paths in the command line arguments
    - e.g. `openocd -f interface/stlink.cfg -f target/stm32f4x.cfg`
- `openocd.gdb`
  - optional, but useful to automate commands for SW debugger (`arm-non-eabi-gdb` in my case)

## Usage

1. Connect the HW debugger to the development board through the SW interface and to the development machine's USB port. Refer to the [Hardware Setup](#hardware-setup) section of this README.md.

2. Open a new terminal and run this command: `openocd -f interface/stlink.cfg -f target/stm32f4x.cfg`.  Alternatively, you can just run `openocd` if you've defined your `openocd.cfg` file

3. Open another terminal and run this command: `arm-none-eabi-gdb -q -x openocd.gdb`.  This will run through the commands in `openocd.gdb` sequentially.  Alternatively, you can run this command without the `-x openocd.gdb` option and argument to enter commands manually.

4. You should see the string printed from call to `hprintln!()` in `main.rs` in the openocd console

## Useful commands (if TCP port is blocked):

`sudo lsof -i :<PORT>`
- Lists open files assoiated with the specified port.

`sudo kill -9 <PID>`
- Terminates the process with the specified PID

[^1]: The Embedded Rust Book. *Semihosting*. Retrieved from https://docs.rust-embedded.org/book/ 