# Minimal cortex-m fault example

This repository demonstrates a null pointer issue during rustc optimization.

This example is designed for a cortex-m STM32H743ZITx device, but the memory file can likely be
updated for any appropriate device.

To observe the fault, break immedaitely after calling `Test::new()` and attempt to print out the
`id` variable. Observe that it reports as null data 200 bytes long.

To make the fault disappear, instead change the [profile.dev] opt-level from 1 -> 0.

# Error Description
When compiled using rust 1.49.0, breaking immediately after calling `Test::new()` results in the
following output:
```
Breakpoint 5, fail_demo::__cortex_m_rt_main () at src/main.rs:36
36          let _ = Test::new("a").unwrap();
(gdb) s
halted: PC: 0x08000648
fail_demo::Test::new (id=...) at src/main.rs:29
29              take_objects(id)?;
(gdb) p id
$1 = '\000' <repeats 200 times>...
(gdb)
```
