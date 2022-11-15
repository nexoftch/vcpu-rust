#[allow(unused_variables)]
use std::collections::HashMap;

pub enum MemoryOperationSize {
    Byte = 1,
    Word = 2,
    Dword = 4
}

/**
 * Registers
 * 
 * Enum with all registers for the virtual cpu.
 * Comments contain some information how to quickly
 * calculate properties for each register and can be ignored.
 */
#[allow(dead_code)]
pub enum Register {
    Unknwown = -1,
    // General purpose registers
    // Least significant bytes 000000[00]
    // floor(n / 4) = 0
    AL = 0, // mod 4 = 0
    BL = 1, // mod 4 = 1
    CL = 2, // mod 4 = 2
    DL = 3, // mod 4 = 3
    
    // Significant byte of lower byte-pair 0000[00]00
    // floor(n / 4) = 1
    AH = 4, // mod 4 = 0
    BH = 5, // mod 4 = 1
    CH = 6, // mod 4 = 2
    DH = 7, // mod 4 = 3
    
    // Two most significant bytes [0000]0000
    // floor(n / 4) = 2
    AX = 8,  // mod 4 = 0
    BX = 9,  // mod 4 = 1
    CX = 10, // mod 4 = 2
    DX = 11, // mod 4 = 3
    
    // All four bytes [00000000]
    // floor(n / 4) = 3
    EAX = 12, // mod 4 = 0
    EBX = 13, // mod 4 = 1
    ECX = 14, // mod 4 = 2
    EDX = 15, // mod 4 = 3
    
    // Stack registers
    // floor(n / 4) = 4
    ESP = 16, // mod 4 = 0
    EBP = 17  // mod 4 = 1
}

/**
 * ManageMemory Trait
 * 
 * Contains definitions to read and write memory in
 * either 1-, 2- or 4-byte chunks.
 */
pub trait ManageMemory {
    // Read operations
    // will call either "read8" or  "read16" on size parameter
    fn read(&self, position : usize, size : MemoryOperationSize);
    // reads 1 byte from the memory
    fn read8(&self, position : usize) -> i8;
    // reads 2 bytes from the memory
    fn read16(&self, position : usize) -> i16;

    // Write operations
    // will call either "write8" or  "write16" on size parameter
    fn write(&mut self, position : usize, value : i32, size : MemoryOperationSize);
    // reads 1 byte from the memory
    fn write8(&mut self, position : usize, value : i8);
    // reads 2 bytes from the memory
    fn write16(&mut self, position : usize, value : i16);
}

pub trait ManageRegisters {
    // returns the address location of the register in parameter
    fn get_register_address(&self, register: Register) -> usize;
    // returns the memory size of the register in parameter
    fn get_register_size(register: Register) -> MemoryOperationSize;
}

/**
 * ManageHeap Trait
 * 
 * Contains definition to manage the heap memory.
 * Allocates or de-allocates specific ranges of memory for
 * programs or data within those programs.
 */
pub trait ManageHeap {
    // Finds available heap location with enough space
    fn find_available_heap(size: u32) -> usize;

    // Allocates heap memory with a specified amount of
    // bytes at the specified position.
    fn allocate_heap(size: u32) -> usize;

    // Frees the heap at a specified position
    fn free_heap(position : usize);
}

pub struct Memory {
    // internal array / vector containing the complete memory
    raw_memory:  Vec<i8>,

    // hash map containing memory locations for each register
    register_lookup_table: HashMap<i32, i32>,
}

impl Memory {
    fn init_register_lookup_table(memory : &mut Memory) {
        // fill registers lookup table
        // ToDo: Find nicer solution?
        memory.register_lookup_table.insert(Register::AL as i32, 3);
        memory.register_lookup_table.insert(Register::BL as i32, 7);
        memory.register_lookup_table.insert(Register::CL as i32, 11);
        memory.register_lookup_table.insert(Register::DL as i32, 15);
        memory.register_lookup_table.insert(Register::AH as i32, 2);
        memory.register_lookup_table.insert(Register::BH as i32, 6);
        memory.register_lookup_table.insert(Register::CH as i32, 10);
        memory.register_lookup_table.insert(Register::DH as i32, 14);
        
        memory.register_lookup_table.insert(Register::AX as i32, 0);
        memory.register_lookup_table.insert(Register::BX as i32, 4);
        memory.register_lookup_table.insert(Register::CX as i32, 8);
        memory.register_lookup_table.insert(Register::DX as i32, 12);
        
        memory.register_lookup_table.insert(Register::EAX as i32, 0);
        memory.register_lookup_table.insert(Register::EBX as i32, 4);
        memory.register_lookup_table.insert(Register::ECX as i32, 8);
        memory.register_lookup_table.insert(Register::EDX as i32, 12);
        
        memory.register_lookup_table.insert(Register::ESP as i32, 16);
        memory.register_lookup_table.insert(Register::EBP as i32, 20);
    }


    // initializes the internal state of the memory implementation
    fn init() -> Memory {
        let mut result = Memory {
            raw_memory: Vec::new(),
            register_lookup_table: HashMap::new()
        };

        Memory::init_register_lookup_table(&mut result);

        // init memory
        for _index in 0..4096  { 
            result.raw_memory.push(0);
        }

        return result;
    }

    // returns an initialized memory struct
    pub fn new() -> Memory {
        Memory::init()
    }
}

// ToDo: Check out of bounds when reading and writing
// ToDo: Implement 4 byte operations
impl ManageMemory for Memory {
    fn read(&self, position : usize, size : MemoryOperationSize) {
        if matches!(size, MemoryOperationSize::Byte) {
            self.read8(position);
        } else if matches!(size, MemoryOperationSize::Word) {
            self.read16(position);
        }
    }

    fn read8(&self, position : usize) -> i8 {
        return self.raw_memory[position];
    }

    fn read16(&self, position : usize) -> i16 {
        return (i16::from(self.raw_memory[position]) << 8) + i16::from(self.raw_memory[position + 1]);
    }

    fn write(&mut self, position : usize, value : i32, size : MemoryOperationSize) {
        if matches!(size, MemoryOperationSize::Byte) {
            self.write8(position, value as i8);
        } else if matches!(size, MemoryOperationSize::Word) {
            self.write16(position, value as i16);
        }
    }

    fn write8(&mut self, position : usize, value : i8) {
        self.raw_memory[position] = value;
    }

    fn write16(&mut self, position : usize, value : i16) {
        self.raw_memory[position] = (value >> 8) as i8;
        self.raw_memory[position+1] = value as i8;
    }
} // impl ManageMemory for Memory


impl ManageRegisters for Memory {
    fn get_register_address(&self, register: Register) -> usize {
        return self.register_lookup_table[&(register as i32)] as usize; 
    }

    fn get_register_size(register: Register) -> MemoryOperationSize {
        let val : i8 = register as i8;

        if val >= 0 && val <= 7 {
            return MemoryOperationSize::Byte;
        }
        else if val >= 8 && val <= 11 {
            return MemoryOperationSize::Word;
        }
        else if val >= 12 {
            return MemoryOperationSize::Dword;
        }

        // if none of the above, return byte
        // might need change to unknown
        return MemoryOperationSize::Byte;
    }
}