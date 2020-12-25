use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, GasFormatter};

const EXAMPLE_CODE_BITNESS: u32 = 64;
const HEXBYTES_COLUMN_BYTE_LENGTH: usize = 10;

pub fn disassemble_x86(bytes: &[u8], host_code_addr: *const u8) {
    let mut decoder = Decoder::new(EXAMPLE_CODE_BITNESS, bytes, DecoderOptions::NONE);
    decoder.set_ip(unsafe { host_code_addr.offset(0) as u64 });

    // Formatters: Masm*, Nasm*, Gas* (AT&T) and Intel* (XED)
    let mut formatter = GasFormatter::new();

    // Change some options, there are many more
    formatter.options_mut().set_digit_separator("_");
    formatter.options_mut().set_first_operand_char_index(10);

    // String implements FormatterOutput
    let mut output = String::new();

    // Initialize this outside the loop because decode_out() writes to every field
    let mut instruction = Instruction::default();

    // The decoder also implements Iterator/IntoIterator so you could use a for loop:
    //      for instruction in &mut decoder { /* ... */ }
    // or collect():
    //      let instructions: Vec<_> = decoder.into_iter().collect();
    // but can_decode()/decode_out() is a little faster:
    while decoder.can_decode() {
        // There's also a decode() method that returns an instruction but that also
        // means it copies an instruction (32 bytes):
        //     instruction = decoder.decode();
        decoder.decode_out(&mut instruction);

        // Format the instruction ("disassemble" it)
        output.clear();
        formatter.format(&instruction, &mut output);

        // Eg. "00007FFAC46ACDB2 488DAC2400FFFFFF     lea       rbp,[rsp-100h]"
        eprint!("{:016X} ", instruction.ip());
        let start_index = (instruction.ip() - unsafe { host_code_addr.offset(0) as u64 }) as usize;
        let instr_bytes = &bytes[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            eprint!("{:02X}", b);
        }
        if instr_bytes.len() < HEXBYTES_COLUMN_BYTE_LENGTH {
            for _ in 0..HEXBYTES_COLUMN_BYTE_LENGTH - instr_bytes.len() {
                eprint!("  ");
            }
        }
        eprintln!(" {}", output);
    }
}


