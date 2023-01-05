mod assembler {
    use core::{str};
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    enum CommandType {
        A,
        C,
        L,
    }

    pub struct Parser {
        input_file: BufReader<File>,
        texts: Vec<String>,
        symbol_table: SymbolTable,
    }

    impl Parser {
        pub fn new() -> Parser {
            // ファイル入力
            let mut command_line = std::env::args();
            command_line.next();
            let pathname = command_line.next().unwrap();
            let f = File::open(pathname).unwrap();
            let f = BufReader::new(f);
            Parser {
                input_file: f,
                texts: vec!["".to_string(); 0],
                symbol_table: SymbolTable::new(),
            }
        }
        // 空行とコメント(//)を削除して File -> Vec<String> へ変換
        pub fn set_texts(&mut self) {
            let f: &mut BufReader<File> = &mut self.input_file;
            for line in f.lines() {
                let mut line_text: String = line.unwrap().to_string();
                line_text = match line_text.find("//") {
                    Some(n) => (&line_text[0..n]).to_string(), 
                    None => line_text,
                };
                if line_text.len() == 0 {
                    continue;
                }
                if line_text.starts_with("//") {
                    continue;
                }
                self.texts.push(line_text.trim().to_string());
            }
        }
        pub fn set_symbol_table(&mut self) {
            self.symbol_table.create_symbol_table(&self.texts);
        }
        pub fn parse(&mut self) {
            let texts: &mut Vec<String> = &mut self.texts.clone();
            for command in texts {
                let parsed_command: String;
                let command_type: CommandType = self.command_type(&command);
                match command_type {
                    CommandType::A => {
                        let symbol = self.symbol(command.to_string());
                        parsed_command = self.parse_a_command(&symbol);
                        println!("{}", parsed_command);
                    }
                    CommandType::C => {
                        parsed_command = self.parse_c_command(command);
                        println!("{}", parsed_command);
                    }
                    CommandType::L => {}
                }
            }
        }
        fn parse_a_command(&mut self, a_command: &str) -> String {
            let num: u32;
            if a_command.chars().all(|c| c.is_numeric()) {
                num = a_command.parse().unwrap();
                return format!("{:016b}", num).to_string();
            }
            self.symbol_table.get_address(a_command)
        }
        fn parse_c_command(&self, c_command: &str) -> String {
            let mut binary: String = "111".to_string();
            binary += self.parse_comp(c_command);
            binary += self.parse_dest(c_command);
            binary += self.parse_jump(c_command);
            binary
        }
        fn command_type(&self, text: &String) -> CommandType {
            if text.chars().nth(0).unwrap() == '@' {
                CommandType::A
            } else if text.chars().nth(0).unwrap() == '(' {
                CommandType::L
            } else {
                CommandType::C
            }
        }
        // シンボルに含まれる不要な文字列を削除
        fn symbol(&self, mut text: String) -> String {
            text = text.replace("@", "");
            text = text.replace("(", "");
            text = text.replace(")", "");
            text
        }
        fn parse_dest(&self, c_command: &str) -> &str {
            match c_command {
                v if v.starts_with("M=") => "001",
                v if v.starts_with("D=") => "010",
                v if v.starts_with("MD=") => "011",
                v if v.starts_with("A=") => "100",
                v if v.starts_with("AM=") => "101",
                v if v.starts_with("AD=") => "110",
                v if v.starts_with("AMD=") => "111",
                _ => "000",
            }
        }
        fn parse_comp(&self, c_command: &str) -> &str {
            let comp: &str;
            if c_command.contains("=") {
                let dest_comp: Vec<&str> = c_command.split("=").collect();
                comp = dest_comp[1]
            } else if c_command.contains(";") {
                let comp_jump: Vec<&str> = c_command.split(";").collect();
                comp = comp_jump[0]
            } else {
                comp = "";
            }
            match comp {
                "0" => "0101010",
                "1" => "0111111",
                "-1" => "0111010",
                "D" => "0001100",
                "A" => "0110000",
                "!D" => "0001101",
                "!A" => "0110001",
                "-D" => "0001111",
                "-A" => "0110011",
                "D+1" => "0011111",
                "A+1" => "0110111",
                "D-1" => "0001110",
                "A-1" => "0110010",
                "D+A" => "0000010",
                "D-A" => "0010011",
                "A-D" => "0000111",
                "D&A" => "0000000",
                "D|A" => "0010101",
                "M" => "1110000",
                "!M" => "1110001",
                "-M" => "1110011",
                "M+1" => "1110111",
                "M-1" => "1110010",
                "D+M" => "1000010",
                "D-M" => "1010011",
                "M-D" => "1000111",
                "D&M" => "1000000",
                "D|M" => "1010101",
                _ => "",
            }
        }
        fn parse_jump(&self, c_command: &str) -> &str {
            match c_command {
                v if v.ends_with(";JGT") => "001",
                v if v.ends_with(";JEQ") => "010",
                v if v.ends_with(";JGE") => "011",
                v if v.ends_with(";JLT") => "100",
                v if v.ends_with(";JNE") => "101",
                v if v.ends_with(";JLE") => "110",
                v if v.ends_with(";JMP") => "111",
                _ => "000",
            }
        }
    }

    pub struct SymbolTable {
        symbol_table: HashMap<String, String>,
        var_address: u16,
    }
    impl SymbolTable {
        pub fn new() -> SymbolTable {
            let mut m = HashMap::new();
            m.insert("SP".to_string(), "0000000000000000".to_string());
            m.insert("LCL".to_string(), "0000000000000001".to_string());
            m.insert("ARG".to_string(), "0000000000000010".to_string());
            m.insert("THIS".to_string(), "0000000000000011".to_string());
            m.insert("THAT".to_string(), "0000000000000100".to_string());
            m.insert("SCREEN".to_string(), "0100000000000000".to_string());
            m.insert("KBD".to_string(), "110000000000000".to_string());
            m.insert("R0".to_string(), "0000000000000000".to_string());
            m.insert("R1".to_string(), "0000000000000001".to_string());
            m.insert("R2".to_string(), "0000000000000010".to_string());
            m.insert("R3".to_string(), "0000000000000011".to_string());
            m.insert("R4".to_string(), "0000000000000100".to_string());
            m.insert("R5".to_string(), "0000000000000101".to_string());
            m.insert("R6".to_string(), "0000000000000110".to_string());
            m.insert("R7".to_string(), "0000000000000111".to_string());
            m.insert("R8".to_string(), "0000000000001000".to_string());
            m.insert("R9".to_string(), "0000000000001001".to_string());
            m.insert("R10".to_string(), "0000000000001010".to_string());
            m.insert("R11".to_string(), "0000000000001011".to_string());
            m.insert("R12".to_string(), "0000000000001100".to_string());
            m.insert("R13".to_string(), "0000000000001101".to_string());
            m.insert("R14".to_string(), "0000000000001110".to_string());
            m.insert("R15".to_string(), "0000000000001111".to_string());

            SymbolTable {
                symbol_table: m,
                var_address: 16,
            }
        }
        fn add_entry(&mut self, symbol: &str) {
            self.symbol_table
                .insert(symbol.to_string(), format!("{:016b}", self.var_address).to_string());
            self.var_address += 1;
        }
        fn contains(&self, symbol: &str) -> bool {
            self.symbol_table.contains_key(symbol)
        }
        fn get_address(&mut self, symbol: &str) -> String {
            if !self.contains(symbol) {
                self.add_entry(symbol);
            }
            self.symbol_table.get(symbol).unwrap().to_string()
        }
        fn create_symbol_table(&mut self, texts: &Vec<String>) {
            let mut current_address: u32 = 0;
            for text in texts {
                if text.starts_with("(") {
                    let mut sym = text.replace("(", "");
                    sym = sym.replace(")", "");
                    self.symbol_table.insert(sym, format!("{:016b}", current_address).to_string());
                }
                else {
                    current_address += 1;
                }
            }
        }
    }
}

fn main() {
    let mut asm = assembler::Parser::new();
    asm.set_texts();
    asm.set_symbol_table();
    asm.parse();
}
