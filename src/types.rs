
pub enum MemoryOperationSize {
    Byte = 1,
    Word = 2
}


pub struct BinaryConstant {
    pub size : MemoryOperationSize,
    pub value : u32
}

pub struct BinaryRegister {
    pub size : MemoryOperationSize,
    pub register : Register
}

/**
 * Registers
 * 
 * Enum with all registers for the virtual cpu.
 * Comments contain some information how to quickly
 * calculate properties for each register and can be ignored.
 */
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

pub enum Instruction {
    mov_reg_const = 70
}

impl Instruction {
    pub fn from_u8(val : u8) -> Instruction {
        match val {
            0xa0 => Instruction::mov_reg_const,
            _ => panic!("Instruction not found")
        }
    }
}