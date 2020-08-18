use crate::instruction::{Condition, Instruction, Instruction::*};
use crate::instruction::Condition::*;
use crate::instruction::Light::*;

static PROG0: [Instruction] = *[
    Write { addr: 0, value: 2000 },
    SoundOn { frequency_addr: 0 },
    WaitConst { value: 3000 },
    SoundOff,
];

static PROG1: [Instruction] = *[
    Write { addr: 0, value: 2000 },
    Write { addr: 1, value: 3000 },
    Wait { value_addr: 0 },
    LightOn { which: IR },
    Wait { value_addr: 1 },
    LightsOff,
    Wait { value_addr: 0 },
    LightOn { which: UV },
    Wait { value_addr: 1 },
    LightsOff,
    Wait { value_addr: 0 },
    LightOn { which: Green },
    Wait { value_addr: 1 },
    LightsOff,
    Wait { value_addr: 0 },
    LightOn { which: Amber },
    Wait { value_addr: 1 },
    LightsOff,
    Wait { value_addr: 0 },
];

static PROG11: [Instruction] = *[
    Write { addr: 0, value: 20 * 60 * 1000 },
    Write { addr: 1, value: 12 },
    Call { program: 21 },
    Call { program: 21 },
    Call { program: 21 },
    Call { program: 21 },
    Call { program: 21 },
    Wait { value_addr: 1 },
    Dec { addr: 1 },
    If {
        condition: MoreThanZero,
        value_addr: 1,
        jump_true_offset: -14,
    },
];

static PROG12: [Instruction] = *[
    Write { addr: 0, value: 20 * 60 * 1000 },
    Write { addr: 1, value: 12 },
    Call { program: 22 },
    Call { program: 22 },
    Wait { value_addr: 1 },
    Dec { addr: 1 },
    If {
        condition: MoreThanZero,
        value_addr: 1,
        jump_true_offset: -8,
    },
];

static PROG13: [Instruction] = *[
    Write { addr: 0, value: 20 * 60 * 1000 },
    Write { addr: 1, value: 12 },
    Call { program: 23 },
    Call { program: 23 },
    Wait { value_addr: 0 },
    AddConst { in_out_addr: 1, constant: -1 },
    If {
        condition: MoreThanZero,
        value_addr: 1,
        jump_true_offset: -9,
    },
];


static PROG21: [Instruction] = *[
    Write { addr: 0, value: 2 },
    // Start outer Loop
    Write { addr: 1, value: 52 },
    Write { addr: 2, value: 3000 },
    // Start inner Loop
    SoundOn { frequency_addr: 2 },
    RandomLightOn,
    AddConst { in_out_addr: 2, constant: 100 },
    WaitConst { value: 50 },
    Dec,
    If {
        condition: MoreThanZero,
        value_addr: 1,
        jump_true_offset: -11,
    },
    AddConst { in_out_addr: 0, constant: -1 },
    If {
        condition: MoreThanZero,
        value_addr: 0,
        jump_true_offset: -11,
    },
    WaitConst { value: 300 },
    SoundOff,
    LightsOff,
    Write { addr: 0, value: 10 },
    WriteInRange {
        addr: 1,
        value_from: 4000,
        value_to: 20000,
    },
    SoundOn { frequency_addr: 0 },
    RandomLightOn,
    WaitConst { value: 200 },
    AddConst { in_out_addr: 0, constant: -1 },
    If {
        condition: MoreThanZero,
        value_addr: 0,
        jump_true_offset: -18,
    },
    SoundOff,
    LightsOff,
    WaitConst { value: 1500 },
    Write { addr: 3, value: 3 },
    Write { addr: 1, value: 1000 },
    // Start outer Loop
    Write { addr: 2, value: 10 },
    // Start inner Loop
    Write { addr: 0, value: 10000 },
    SoundOn { frequency_addr: 0 },
    RandomLightOn,
    WaitConst { value: 250 },
    LightsOff,
    Add { in_out_addr: 0, input_addr: 1 },
    If {
        condition: MoreThanZero,
        value_addr: 2,
        jump_true_offset: -16,
    },
    If {
        condition: MoreThanZero,
        value_addr: 3,
        jump_true_offset: -26,
    },
    SoundOff,
];

static PROG22: [Instruction] = *[
//     new StartLoop(),
// new ActivateSoundAndLight(new Range(3000, 9000)),
// new Wait(new Fixed(50)),
// new EndLoop(new Fixed(60)),
// new DeactivateSoundAndLight(),
// new Wait(new Fixed(2500)),
// new StartLoop(),
// new ActivateSoundAndLight(new Fixed(6000)),
// new Wait(new Fixed(20)),
// new DeactivateSoundAndLight(),
// new Wait(new Fixed(50)),
// new EndLoop(new Fixed(50)),
// new StartLoop(),
// new ActivateSoundAndLight(new Range(9000, 20000)),
// new Wait(new Fixed(200)),
// new EndLoop(new Fixed(10)),
// new Wait(new Fixed(2000)),
// new DeactivateSoundAndLight(),
// new Wait(new Fixed(1700)),
// new StartLoop(),
// new ActivateSoundAndLight(new Range(20000, 30000)),
// new Wait(new Fixed(300)),
// new EndLoop(new Fixed(9)),
// new DeactivateSoundAndLight(),
// new EndProgram(),
];

static PROG23: [Instruction] = *[
    Write { addr: 1, value: 60 },
    WriteInRange {
        addr: 0,
        value_from: 1000,
        value_to: 20000,
    },
    SoundOn { frequency_addr: 0 },
    RandomLightOn,
    WriteInRange {
        addr: 0,
        value_from: 10,
        value_to: 500,
    },
    Wait { value_addr: 0 },
    LightsOff,
    SoundOff,
    WriteInRange {
        addr: 0,
        value_from: 10,
        value_to: 500,
    },
    Wait { value_addr: 0 },
    Dec { addr: 1 },
    If {
        condition: MoreThanZero,
        value_addr: 1,
        jump_true_offset: -120, // TODO: Wrong
    },
    WriteInRange {
        addr: 0,
        value_from: 1000,
        value_to: 10000,
    },
    Wait { value_addr: 0 },
];