pub mod arm;

use num_derive::FromPrimitive;

use crate::register::CpsrFlag;
use crate::register::Register;

#[rustfmt::skip]
#[derive(FromPrimitive)]
enum Cond {
    EQ   = 0x0,
    NE   = 0x1,
    CSHS = 0x2,
    CCLO = 0x3,
    MI   = 0x4,
    PL   = 0x5,
    VS   = 0x6,
    VC   = 0x7,
    HI   = 0x8,
    LS   = 0x9,
    GE   = 0xa,
    LT   = 0xb,
    GT   = 0xc,
    LE   = 0xd,
    AL   = 0xe,
    NV   = 0xf,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum CpuState {
    THUMB,
    ARM,
}

pub struct Cpu {
    pub(crate) register: Register,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register: Register::new(),
        }
    }

    #[rustfmt::skip]
    fn cond(&self, inst: u32) -> bool {
        let nzcv = inst >> 28;
        let cond = num::FromPrimitive::from_u32(nzcv).unwrap();
        let flag_n = self.register.cpsr.is_valid_flag(CpsrFlag::N);
        let flag_z = self.register.cpsr.is_valid_flag(CpsrFlag::Z);
        let flag_c = self.register.cpsr.is_valid_flag(CpsrFlag::C);
        let flag_v = self.register.cpsr.is_valid_flag(CpsrFlag::V);

        match cond {
            Cond::EQ   =>  flag_z,
            Cond::NE   => !flag_z,
            Cond::CSHS =>  flag_c,
            Cond::CCLO => !flag_c,
            Cond::MI   =>  flag_n,
            Cond::PL   => !flag_n,
            Cond::VS   =>  flag_v,
            Cond::VC   => !flag_v,
            Cond::HI   =>  flag_c && !flag_z,
            Cond::LS   => !flag_c ||  flag_z,
            Cond::GE   =>  flag_n ==  flag_v,
            Cond::LT   =>  flag_n !=  flag_v,
            Cond::GT   => !flag_z && (flag_n == flag_v),
            Cond::LE   =>  flag_z || (flag_n != flag_v),
            Cond::AL   => true,
            Cond::NV   => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cond() {
        // Initial state of all nzcv flag is false.
        // flag_n = false
        // flag_z = false
        // flag_c = false
        // flag_v = false
        let cpu = Cpu::new();

        let insts = vec![
            0x0000_0000, // EQ
            0x1000_0000, // NE
            0x2000_0000, // CSHS
            0x3000_0000, // CCLO
            0x4000_0000, // MI
            0x5000_0000, // PL
            0x6000_0000, // VS
            0x7000_0000, // VC
            0x8000_0000, // HI
            0x9000_0000, // LS
            0xa000_0000, // GE
            0xb000_0000, // LT
            0xc000_0000, // GT
            0xd000_0000, // LE
            0xe000_0000, // AL
            0xf000_0000, // NV
        ];

        let answers = vec![
            false, //  flag_z
            true,  // !flag_z
            false, //  flag_c
            true,  // !flag_c
            false, //  flag_n
            true,  // !flag_n
            false, //  flag_v
            true,  // !flag_v
            false, //  flag_c && !flag_z,
            true,  // !flag_c ||  flag_z,
            true,  //  flag_n ==  flag_v,
            false, //  flag_n !=  flag_v,
            true,  // !flag_z && (flag_n == flag_v),
            false, //  flag_z || (flag_n != flag_v),
            true,  // true,
            false, // false,
        ];

        for (&inst, &ans) in insts.iter().zip(answers.iter()) {
            assert_eq!(cpu.cond(inst), ans);
        }
    }
}
