use nes::cpu::Cpu;
use nes::bus::Bus;

fn main() {
    let bus = Bus::new();
    let mut cpu = Cpu::new();
    cpu.connect_bus(bus);
    
    cpu.write(0x00, 42);
    println!("Value at address 0x00: {}", cpu.read(0x00, false));
}
