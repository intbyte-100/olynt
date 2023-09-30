mod backend;
mod frontend;
mod util;

use frontend::parser::Parser;
use util::logger::Logger;
use util::mask::Mask;

use std::{
    env,
    fs::File,
    io::{stdout, BufRead, BufReader},
};

fn main() {
    let file = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut binding = Logger::new(
        util::logger::LoggerType::ERROR_AND_WARNS,
        Box::new(stdout()),
    );

    let mut lexer = Parser::new(&mut lines, &mut binding);

    let tokens = lexer.parse();

    return;
}
