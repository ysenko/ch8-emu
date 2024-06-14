mod chip8;

fn main() {
    let mut system = chip8::Chip8::new();

    system.boot().unwrap();

    println!("{:?}", system)
}
