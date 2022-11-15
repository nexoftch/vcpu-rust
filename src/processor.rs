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
    
    pub fn load_program(&mut self, path : String) -> bool {
        self.file_path = path.to_string();
        println!("Trying to load program '{}'...", &self.file_path);

        match fs::read(&self.file_path) {
            Ok(file) => {
                for byte in file {
                    print!("{:#02x} ", byte);
                }
                println!("");
                return true;
            },
            Err(error) => {
                println!("Error while loading program... --> {}", error);
                return false;
            }
        }
    }
}