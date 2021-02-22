use std::fs;

fn main() {
    let file = fs::read_to_string("docs/instructions.csv").unwrap();
    let mut lines = file.lines();
    // skip header
    lines.next().unwrap();
    let mut ops = vec![(String::from("UNI"), String::from("Unimplemented")); 256];
    for line in lines {
        let mut cols = line.split_whitespace();
        let opcode = cols.next().unwrap();
        let opcode = u8::from_str_radix(opcode, 16).unwrap();
        let mnemnoic = cols.next().unwrap();
        let addressing_mode = cols.next().unwrap();
        ops[opcode as usize] = (String::from(mnemnoic), String::from(addressing_mode));
    }
    for opcode in 0..256 {
        let (mnemnoic, addressing_mode) = &ops[opcode];
        println!(
            "{:02X} {} {}",
            opcode,
            mnemnoic.to_uppercase(),
            addressing_mode
        );
    }
}
