use std::str::FromStr;
use crate::hackspec::HackASMInstruction;

pub struct AInstruction {
    value: i16
}
impl HackASMInstruction for AInstruction {
    fn to_binary(&self) -> String {
        let bin_value = format!("{:016b}", self.value);

        // Replace the resulting MSB with 0 as per the Hack language specification
        // for denoting an A Instruction
        format!("0{}", bin_value.split_at(1).1)
    }
}
impl FromStr for AInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s[1..].parse() {
            Ok(value) => Ok(AInstruction { value }),
            Err(_) => Err(())
        }
    }
}
