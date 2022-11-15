use std::fs;

pub struct Processor {
    file_path : String,
    program_code : Vec<u8>

}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            file_path: String::new(),
            program_code: Vec::new()
        }
    }
    
    // todo: check wtf that is xddd
    pub fn load_program(&mut self, path : String) -> Result<(), Box<dyn std::error::Error + 'static>> {
        #[cfg(debug_assertions)]
        println!("Trying to load program '{}'...", path);
        self.file_path = path.to_string();

        // load file
        let mut file = fs::read(&self.file_path)?;
        for byte in file {
            print!("{:#02x} ", byte);
        }
        println!("");
        Ok(())
    }
}