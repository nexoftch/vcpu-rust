mod memory;

use crate::memory::{Memory, ManageMemory};

fn main() {
    let mut mem : Memory = Memory::new();

    mem.write8(0, 0x12);
    mem.write8(1, 0x34);
    let val = mem.read16(0);

    println!("{:#02x}", val);

    println!("Hello, world!");
}
