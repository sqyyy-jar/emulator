use emulator::emulator::{and, input, not, or, Emulator};

fn main() {
    let emu = Emulator::new(4, and([input(0), or([input(1), input(2)]), not(input(3))])).unwrap();
    println!("{}", emu.emulate_all().unwrap());
}
