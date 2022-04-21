use nes::bus::Bus;
use nes::cpu::Cpu;

fn main() {
    let mut bus = Bus::new();

    bus.write(0x00, 42);

    println!("Value at address 0x00: {}", bus.read(0x00));
}
