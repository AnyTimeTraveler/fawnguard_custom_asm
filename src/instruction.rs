#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Instruction {
    Noop,
    // Variables
    Write {
        addr: u8,
        value: i32,
    },
    WriteInRange {
        addr: u8,
        value_from: i32,
        value_to: i32,
    },
    Add {
        in_out_addr: u8,
        input_addr: u8,
    },
    AddConst {
        in_out_addr: u8,
        constant: i8,
    },
    Inc {
        addr: u8,
    },
    Dec {
        addr: u8,
    },
    Mul {
        in_out_addr: u8,
        input_addr: u8,
    },
    Div {
        in_out_addr: u8,
        input_addr: u8,
    },

    // Control
    If {
        condition: Condition,
        value_addr: u8,
        jump_true_offset: i8,
    },
    Wait {
        value_addr: u8,
    },
    WaitConst {
        value: u16,
    },
    Call {
        program: u8
    },
    LoopCall {
        program: u8,
        repetitions: u8
    },

    // I/O
    SoundOn {
        frequency_addr: u8
    },
    SoundOff,
    LightOn {
        which: Light,
    },
    LightsOff,
    RandomLightOn,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Condition {
    True,
    Zero,
    NotZero,
    LessThanZero,
    MoreThanZero,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Light {
    IR,
    UV,
    Green,
    Amber,
}
