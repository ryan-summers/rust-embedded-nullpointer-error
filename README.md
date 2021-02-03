# Minimal cortex-m fault example

This repository demonstrates a null pointer issue during rustc optimization.

This example is designed for a cortex-m STM32H743ZITx device, but the memory file can likely be
updated for any appropriate device.

To observe the fault, break immedaitely after calling `Test::new()` and attempt to print out the
`id` variable. Observe that it reports as null data 200 bytes long.

To make the fault disappear, instead change the [profile.dev] opt-level from 1 -> 0.

