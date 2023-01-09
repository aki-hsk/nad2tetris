mod parser;
mod writer;
use parser::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
    // ファイル入力
    let mut command_line = std::env::args();
    command_line.next();
    let pathname = command_line.next().unwrap();
    let file = File::open(pathname).unwrap();
    let file = BufReader::new(file);

    let mut p = Parser::new(file);
    p.parse();
}
