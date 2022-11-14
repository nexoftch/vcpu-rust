pub trait ManageMemory {
    
    fn read8(&self, position : usize) -> i8;
    fn read16(&self, position : usize) -> i16;

    fn write8(&mut self, position : usize, value : i8);
    fn write16(&self, position : usize, value : i16);
}

pub struct Memory {
    raw_memory:  Vec<i8>
}

impl Memory {
    pub fn new() -> Memory {
        let mut result = Memory {
            raw_memory: Vec::new()
        };

        for index in 0..4096  {
            result.raw_memory.push(0);
        }

        result
    }
}


impl ManageMemory for Memory {
    fn read8(&self, position : usize) -> i8 {
        return self.raw_memory[position];
    }

    fn read16(&self, position : usize) -> i16 {
        return (i16::from(self.raw_memory[position]) << 8) + i16::from(self.raw_memory[position + 1]);
    }

    fn write8(&mut self, position : usize, value : i8) {
        self.raw_memory[position] = value;
    }

    fn write16(&self, position : usize, value : i16) {

    }
}