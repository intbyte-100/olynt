mod frontend;
mod backend;
mod util;

use frontend::lexer::Lexer;
use util::logger::Logger;
use util::mask::Mask;


use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, stdout},
};

fn main() {
    let file = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut binding = Logger::new(util::logger::LoggerType::ERROR_AND_WARNS, Box::new(stdout()));
    
    
    let mut lexer = Lexer::new(&mut lines, &mut binding);

    
    let tokens = lexer.parse();

    

    return;
}
