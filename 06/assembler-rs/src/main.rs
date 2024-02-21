mod hackspec;
use std::env;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use hackspec::{HackASMInstruction, a_instruction::AInstruction, c_instruction::CInstruction};


fn main() {
    let mut symbols = hackspec::get_predefined_symbols();
    let file_path: String = env::args().into_iter().last().expect("Please provide a file to compile as last parameter");
    let file_result = File::open(&file_path);
    let mut a_and_c_instructions: Vec<String> = Vec::new();

    // There are 2 passes made to translate an asm file into hack binary representation
    // First pass references labels and puts A and C Instructions into a stack, which is then
    // used to output the result
    if let Ok(file) = file_result {
        for line in BufReader::new(file).lines() {
            let l = line.expect("Should be able to parse line");

            match l.trim().chars().nth(0) {
                Some('/') | None => (),
                Some('(') => {
                    if let Some(label) = l.strip_prefix('(').expect("Satisfied by match pattern").strip_suffix(')') {
                        symbols.insert(label.to_string(), a_and_c_instructions.len() as i16);
                    }
                },
                _ => { 
                    a_and_c_instructions.push(l.trim().to_string());
                }
            }
        }
    }

    let mut binary_instructions: Vec<String> = Vec::new();
    let mut address = 16;
    for instruction in a_and_c_instructions {
        match instruction.chars().nth(0) {
            Some('@') => {
                if let Ok(a_instr)= instruction.parse::<AInstruction>() {
                    binary_instructions.push(a_instr.to_binary());
                } else {
                    let variable = &instruction[1..];
                    match symbols.get(variable) {
                        Some(address) => {
                            if let Ok(a_instr) = format!("@{}", address).parse::<AInstruction>() {
                                binary_instructions.push(a_instr.to_binary());
                            }
                        },
                        None => {
                            symbols.insert(variable.to_string(), address);
                            if let Ok(a_instr) = format!("@{}", address).parse::<AInstruction>() {
                                binary_instructions.push(a_instr.to_binary());
                            }
                            address += 1;
                        }
                    }
                }
            },
            _ => {
                if let Ok(c_instr) = instruction.parse::<CInstruction>() {
                    binary_instructions.push(c_instr.to_binary());
                }
            }
        }
    }

    let target_path = file_path.replace(".asm", ".hack");
    if let Ok(output) = File::create(target_path).as_mut() {
        for instr in &binary_instructions {
            let _ = writeln!(output, "{}", instr);
        }
    }
}
