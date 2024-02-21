use std::str::FromStr;
use crate::hackspec::HackASMInstruction;

pub struct CInstruction {
    dest: Option<String>,
    comp: String,
    jmp: Option<String>,
}
impl HackASMInstruction for CInstruction {
    fn to_binary(&self) -> String {
        format!("111{}{}{}{}", 
                self.get_a_bit(), 
                self.get_comp_binary(), 
                self.get_dest_binary(),
                self.get_jmp_binary())
    }
}
impl CInstruction {
    fn get_comp_binary(&self) -> String {
        String::from(match self.comp.as_str() {
            "0" => "101010",
            "1" => "111111",
            "-1" => "111010",
            "D" => "001100",
            "A" | "M" => "110000",
            "!D" => "001101",
            "!A" | "!M" => "110001",
            "D+1" => "011111",
            "A+1" |  "M+1" => "110111",
            "D-1" => "001110",
            "A-1" | "M-1" => "110010",
            "D+A" | "D+M" => "000010",
            "D-A" | "D-M" => "010011",
            "A-D" | "M-D" => "000111",
            "D&A" | "D&M" => "000000",
            "D|A" | "D|M" => "010101",
            _ => ""
        })
    }

    fn get_a_bit(&self) -> &str {
        match self.comp.contains("M") {
            true => "1",
            false => "0",
        }
    }

    fn get_dest_binary(&self) -> &str {
        if let Some(x) = &self.dest {
            match x.as_str() {
                "M" => "001",
                "D" => "010",
                "MD" => "011",
                "A" => "100",
                "AM" => "101",
                "AD" => "110",
                "AMD" => "111",
                _ => "000"
            }
        } else {
            "000"
        }
    }
    fn get_jmp_binary(&self) -> &str {
        if let Some(x) = &self.jmp {
            match x.as_str() {
                "JGT" => "001",
                "JEQ" => "010",
                "JGE" => "011",
                "JLT" => "100",
                "JNE" => "101",
                "JLE" => "110",
                "JMP" => "111",
                _ => "000"
            }
        } else {
            "000"
        }
    }
}
impl FromStr for CInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instr = CInstruction {
            dest: None,
            comp: String::new(),
            jmp: None
        };
        let mut parsed_str = s;
        if let Some((dest, rest)) = parsed_str.split_once('=') {
            instr.dest = Some(String::from(dest));
            parsed_str = rest;
        }
        if let Some((comp, jmp)) = parsed_str.split_once(';') {
            instr.comp = String::from(comp);
            instr.jmp = Some(String::from(jmp));
        } else {
            instr.comp = String::from(parsed_str);
        }

        Ok(instr)
    }
}
