mod memory;
mod types;
use crate::memory::{Memory, ManageMemory, ManageRegisters};

mod processor;
use crate::processor::{Processor, ReadProgramCode};
use crate::types::Register;

fn main() {
    let mut cpu : Processor = Processor::new();
    cpu.load_program(("D:/vcpu/temp.fae").to_string());
    cpu.step();


    // init memory
    let mut  mem : Memory = Memory::new();

    // writing 0x12 and 0x34, then reading both bytes together and print them
    mem.write8(0, 0x12);
    mem.write8(1, 0x34);
    println!("1234: {:#02x}", mem.read16(0));

    // writing 0x1337 and then print both single bytes
    mem.write16(2, 0x1337);
    println!("13: {:#02x}", mem.read8(2));
    println!("37: {:#02x}", mem.read8(3));

    // printing memory addresses of a few registers:
    println!("EBX {:#02x}", mem.get_register_address(Register::EBX));
    println!("AX  {:#02x}", mem.get_register_address(Register::AX));
    println!("DH  {:#02x}", mem.get_register_address(Register::DH));
    println!("CL  {:#02x}", mem.get_register_address(Register::CL));
    println!("ESP {:#02x}", mem.get_register_address(Register::ESP));
}
