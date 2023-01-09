pub const INITIAL_SP_ADDRESS: u32 = 256;

const PUSH_ASM: &str = "@SP
A=M
M=D
@SP
M=M+1";

const POP_ASM: &str = "@SP
M=M-1
@SP
A=M
D=M";

pub fn set_ram(symbol: &str, value: u32) {
    println!("@{}\nD=A\n@{}\nM=D", value, symbol);
}

pub fn push_segment(segment: &str, index: u32) {
    match segment {
        "constant" => println!("@{}\nD=A", index),
        _ => println!(""),
    }
    push();
}

pub fn push() {
    println!("{}", PUSH_ASM);
}

pub fn pop() {
    println!("{}", POP_ASM);
}

pub fn binary_op(op: &str) {
    pop();
    println!("{}", "@R13\nM=D");
    pop();
    let op = match op {
        "add" => "+",
        "sub" => "-",
        _ => "",
    };
    println!("@R13\nD=D{}M", op);
}
