use std::fs;

use crate::types::{
    BinaryConstant,
    BinaryRegister,
    MemoryOperationSize,
    Register,
    Instruction
};

pub struct Processor {
    file_path : String,

    program_code : Vec<u8>,
    program_counter : usize
}


pub trait ReadProgramCode {
    fn read_byte(&self, steps_taken : &mut u32) -> i8;
    fn read_word(&self, steps_taken : &mut u32) -> i16;
    fn read_dword(&self, steps_taken : &mut u32) -> i32;

    fn read_constant(&self, steps_taken : &mut u32) -> BinaryConstant;
    fn read_register(&self, steps_taken : &mut u32) -> BinaryRegister;

    // tries to exectute a command and returns if the program has reached its end
    fn step(&self) -> bool;
}

pub trait ExecuteInstructions {
    fn mov_reg_const(&self);
    fn mov_reg_reg(&self);
}


impl Processor {
    // returns a new instance of "Processor"
    pub fn new() -> Processor {
        Processor {
            file_path: String::new(),
            program_code: Vec::new(),
            program_counter: 0
        }
    }
    
    // loads program into local "program_code" vector
    pub fn load_program(&mut self, path : String) -> bool {
        self.file_path = path.to_string();

        #[cfg(debug_assertions)]
        print!("Trying to load program '{}'... ", &self.file_path);

        match fs::read(&self.file_path) {
            Ok(file) => {
                #[cfg(debug_assertions)]
                println!("\x1b[0;32mOK!\x1b[0m", );

                self.program_code = file;
                return true;
            },
            Err(error) => {
                #[cfg(debug_assertions)]
                println!("\x1b[0;31mERROR!\x1b[0m\n ---> {}", error);

                return false;
            }
        }
    }
}

impl ReadProgramCode for Processor {
    fn read_byte(&self, steps_taken : &mut u32) -> i8 {
        *steps_taken += 1;
        return self.program_code[self.program_counter] as i8;
    }

    fn read_word(&self, steps_taken : &mut u32) -> i16 {
        *steps_taken += 2;
        let value : i16 = (i16::from(self.program_code[self.program_counter]) << 8) +
                          (i16::from(self.program_code[self.program_counter + 1]));

        return value;
    }

    fn read_dword(&self, steps_taken : &mut u32) -> i32 {
        *steps_taken += 4;
        let value : i32 = (i32::from(self.program_code[self.program_counter]) << 24) +
                          (i32::from(self.program_code[self.program_counter + 1]) << 16) +
                          (i32::from(self.program_code[self.program_counter + 2]) << 8) +
                          (i32::from(self.program_code[self.program_counter + 3]));

        return value;
    }

    
    fn read_constant(&self, steps_taken : &mut u32) -> BinaryConstant {
        return BinaryConstant {
            size: MemoryOperationSize::Byte,
            value: 0
        };
    }

    fn read_register(&self, steps_taken : &mut u32) -> BinaryRegister {
        return BinaryRegister {
            size: MemoryOperationSize::Byte,
            register: Register::EAX
        };
    }

    fn step(&self) -> bool {
        let steps_taken : &mut u32 = &mut 0;

        // read instruction
        let instruction = self.read_byte(steps_taken);
        
        match instruction {
            70 => { self.mov_reg_const() }
            71 => { self.mov_reg_reg() }
            _ => println!("unknown instruction {}", instruction)
        }


        return true;
    }
}

impl ExecuteInstructions for Processor {
    fn mov_reg_const(&self) {
        println!("instruction 'mov_reg_const' called");
        self.read_byte(&mut 0);
    }
    
    fn mov_reg_reg(&self) {
        println!("instruction 'mov_reg_reg' called");
        self.read_byte(&mut 0);
    }
}