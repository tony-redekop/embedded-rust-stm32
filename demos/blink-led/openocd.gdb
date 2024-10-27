target remote :3333

monitor arm semihosting enable

load ../../target/thumbv7em-none-eabihf/debug/semihosting-demo

break main

continue

step
