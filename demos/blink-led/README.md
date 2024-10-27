# blink-led

Blinking an LED using baremetal Rust and the BlackPill STM32 development board

## Background

### What is bare metal programming?

Bare Metal (MCU) --> PAC --> HAL --> BSP

**Definitions**

- Bare metal programming represents the lowest level of abstraction in embedded programming.
- Memory addresses pertaining to peripheral registers are referenced (cast to pointer type) and manipulated directly.
- Accessing peripheral registers must be done in a volatile way
    - The state of memory can change due to HW or other events outside of the CPU's control.
    - The compiler is restricted from implementing some optimizations
        - caching is disabled
        - omitting multiple reads or writes to the same memory address is disabled.

## Hardware Setup 
```
Dev machine (host): Lenovo T480 running Arch Linux
Development board (target): WeAct Studio STM32F411CEU6 BlackPill v3.1
HW debugger: ST-Link V2 
```
## Result

<img src="img/blink-led.gif">
