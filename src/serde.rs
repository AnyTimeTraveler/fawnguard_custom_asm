use std::convert::{TryFrom, TryInto};

use crate::instruction::{Condition, Instruction};
use crate::instruction::Condition::*;
use crate::instruction::Instruction::*;
use crate::instruction::Light::*;

impl TryFrom<&[u8]> for Instruction {
    type Error = ();

    fn try_from(program_memory: &[u8]) -> Result<Self, Self::Error> {
        match program_memory[0] {
            0 => Ok(Noop),
            // Variables
            1 => Ok(Write {
                addr: program_memory[1],
                value: 0 |
                    (program_memory[2] as i32) << 24 |
                    (program_memory[3] as i32) << 16 |
                    (program_memory[4] as i32) << 8 |
                    (program_memory[5] as i32) << 0,
            }),
            2 => Ok(Add { in_out_addr: program_memory[1], input_addr: program_memory[2] }),
            3 => Ok(Mul { in_out_addr: program_memory[1], input_addr: program_memory[2] }),
            4 => Ok(Div { in_out_addr: program_memory[1], input_addr: program_memory[2] }),

            // Control
            20 => Ok(If {
                condition: program_memory[1].try_into()?,
                value_addr: program_memory[2],
                jump_true_offset: program_memory[3] as i8,
            }),
            21 => Ok(Wait { value_addr: program_memory[1] }),

            // I/O
            40 => Ok(SoundOn {
                frequency_addr: program_memory[1],
            }),
            41 => Ok(SoundOff),
            42 => Ok(LightOn { which: IR }),
            43 => Ok(LightOn { which: UV }),
            44 => Ok(LightOn { which: Green }),
            45 => Ok(LightOn { which: Amber }),
            46 => Ok(LightsOff),

            _ => Err(()),
        }
    }
}


impl Into<Vec<u8>> for Instruction {
    fn into(self) -> Vec<u8> {
        match self {
            Noop => vec![0],
            // Variables
            Write { addr, value } =>
                vec![
                    1,
                    addr,
                    (value >> 24) as u8,
                    (value >> 16) as u8,
                    (value >> 8) as u8,
                    (value >> 0) as u8,
                ],
            Add { in_out_addr, input_addr } => vec![2, in_out_addr, input_addr],
            Mul { in_out_addr, input_addr } => vec![3, in_out_addr, input_addr],
            Div { in_out_addr, input_addr } => vec![4, in_out_addr, input_addr],
            // Control
            If { condition, value_addr, jump_true_offset } => vec![20, condition.into(), value_addr, jump_true_offset as u8],
            Wait { value_addr } => vec![21, value_addr],
            // I/O
            SoundOn { frequency_addr } => vec![40, frequency_addr],
            SoundOff => vec![41],
            LightOn { which: IR } => vec![42],
            LightOn { which: UV } => vec![43],
            LightOn { which: Green } => vec![44],
            LightOn { which: Amber } => vec![45],
            LightsOff => vec![46],
        }
    }
}

impl TryFrom<u8> for Condition {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(True),
            1 => Ok(Zero),
            2 => Ok(NotZero),
            3 => Ok(LessThanZero),
            4 => Ok(MoreThanZero),
            _ => Err(()),
        }
    }
}

impl Into<u8> for Condition {
    fn into(self) -> u8 {
        match self {
            True => 0,
            Zero => 1,
            NotZero => 2,
            LessThanZero => 3,
            MoreThanZero => 4
        }
    }
}