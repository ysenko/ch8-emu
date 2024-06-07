# ch8-emu a Chip8 emulator written for fun

## Project Structure
```
chip8/
├── src/
│   ├── main.rs
│   ├── chip8.rs
│   ├── memory.rs
│   ├── registers.rs
│   ├── stack.rs
│   ├── timers.rs
│   ├── input.rs
│   ├── display.rs
│   └── opcode.rs
├── tests/
│   ├── chip8_tests.rs
│   ├── memory_tests.rs
│   ├── registers_tests.rs
│   ├── stack_tests.rs
│   ├── timers_tests.rs
│   ├── input_tests.rs
│   ├── display_tests.rs
│   └── opcode_tests.rs
└── Cargo.toml
```

`main.rs`: The entry point of your program. This is where your main loop will live.
`chip8.rs`: The main Chip-8 structure that ties all the other components together.
`memory.rs`: The implementation of the Chip-8 memory.
`registers.rs`: The implementation of the Chip-8 registers.
`stack.rs`: The implementation of the Chip-8 stack.
`timers.rs`: The implementation of the Chip-8 timers.
`input.rs`: The handling of the Chip-8 input.
`display.rs`: The implementation of the Chip-8 display.
`opcode.rs`: The decoding and execution of the Chip-8 opcodes.
