use crate::instruction::{Instruction, Instruction::*};
use crate::instruction::Condition::*;

mod programs;// #![no_std]

mod serde;
mod instruction;

// extern crate alloc;
// use alloc_cortex_m::CortexMHeap;
// use cortex_m_rt::entry;

// #[global_allocator]
// static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
//
// pub fn init() {
//     let start = cortex_m_rt::heap_start() as usize;
//     let size = 16 * 1024; // in bytes
//     unsafe { ALLOCATOR.init(start, size) }
// }

pub(crate) fn execute_instruction(instruction: Instruction, registers: &mut [i32]) -> i8 {
    match instruction {
        Noop => 1,
        Write { addr, value } => {
            registers[addr as usize] = value;
            6
        }
        Add { in_out_addr, input_addr } => {
            let a = registers[input_addr as usize];
            let b = registers[in_out_addr as usize];
            registers[in_out_addr as usize] = a + b;
            3
        }
        Mul { in_out_addr, input_addr } => {
            let a = registers[input_addr as usize];
            let b = registers[in_out_addr as usize];
            registers[in_out_addr as usize] = a * b;
            3
        }
        Div { in_out_addr, input_addr } => {
            let a = registers[input_addr as usize];
            let b = registers[in_out_addr as usize];
            registers[in_out_addr as usize] = a / b;
            3
        }
        If { condition, value_addr, jump_true_offset } => {
            let compare_result = match condition {
                True => true,
                Zero => registers[value_addr as usize] == 0,
                NotZero => registers[value_addr as usize] != 0,
                LessThanZero => registers[value_addr as usize] < 0,
                MoreThanZero => registers[value_addr as usize] > 0,
            };
            if compare_result {
                jump_true_offset
            } else {
                4
            }
        }
        Wait { value_addr } => {
            println!("Wait for {} millis", registers[value_addr as usize]);
            2
        }
        Call { program } => {
            println!("Run program {}", program);
            2
        }

        SoundOn { frequency_addr } => {
            println!("Sound on: {}", registers[frequency_addr as usize]);
            2
        }
        SoundOff => {
            println!("Sound off");
            1
        }
        LightOn { which } => {
            println!("Light on: {:?}", which);
            1
        }
        LightsOff => {
            println!("Lights off");
            1
        }
    }
}

macro_rules! build_program_memory {
    ($($x:expr),+ $(,)?) => (
        {
            let mut temp_vec = Vec::new();
            $(
                {
                    let temp_item: Vec<u8> = $x.into();
                    temp_vec.extend(temp_item);
                }
            )+
            temp_vec
        }
    );
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    static DEAD_BEEF: u32 = 0xDEADBEEF;

    fn test_serde(instruction_under_test: Instruction) {
        let program_memory = build_program_memory!(
            instruction_under_test,
        );
        let instruction = Instruction::try_from(&program_memory[0..]);

        assert!(instruction.is_ok());
        assert_eq!(instruction.unwrap(), instruction_under_test);
    }

    #[test]
    fn write_into_first_register() {
        let instruction = Write { addr: 0, value: DEAD_BEEF as i32 };
        test_serde(instruction);

        let mut registers = [0i32; 1];
        let program_counter = execute_instruction(instruction, &mut registers);

        assert_eq!(program_counter, 5);
        assert_eq!(registers[0] as u32, DEAD_BEEF);
    }

    #[test]
    fn add_one_to_first_register() {
        let instruction_under_test = Add { in_out_addr: 0, input_addr: 1 };

        test_serde(instruction_under_test);

        let mut registers = [0, 1];
        let program_counter = execute_instruction(instruction_under_test, &mut registers);

        assert_eq!(program_counter, 2);
        assert_eq!(registers[0], 1);
    }

    #[test]
    fn decode_instruction_write() {
        let program_memory = [0u8; 16];
        let instruction = Instruction::try_from(&program_memory[0..]);

        assert!(instruction.is_ok());
        assert_eq!(instruction.unwrap(), Write { addr: 0, value: 0 })
    }
}
