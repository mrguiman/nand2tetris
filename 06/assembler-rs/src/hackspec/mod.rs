use std::collections::HashMap;

pub mod c_instruction;
pub mod a_instruction;

pub trait HackASMInstruction {
    fn to_binary(&self) -> String;
}
pub fn get_predefined_symbols() -> HashMap<String, i16> {
    HashMap::from([
                  (String::from("R0"), 0),
                  (String::from("R1"), 1),
                  (String::from("R2"), 2),
                  (String::from("R3"), 3),
                  (String::from("R4"), 4),
                  (String::from("R5"), 5),
                  (String::from("R6"), 6),
                  (String::from("R7"), 7),
                  (String::from("R8"), 8),
                  (String::from("R9"), 9),
                  (String::from("R10"), 10),
                  (String::from("R11"), 11),
                  (String::from("R12"), 12),
                  (String::from("R13"), 13),
                  (String::from("R14"), 14),
                  (String::from("R15"), 15),
                  (String::from("SCREEN"), 16384),
                  (String::from("KBD"), 24576),
                  (String::from("SP"), 0),
                  (String::from("LCL"), 1),
                  (String::from("ARG"), 2),
                  (String::from("THIS"), 3),
                  (String::from("THAT"), 4),
                  ])
}
