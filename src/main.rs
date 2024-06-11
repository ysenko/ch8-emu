mod chip8;

fn main() {
    let system = chip8::Chip8::new();
    println!("{:?}", system)
}
