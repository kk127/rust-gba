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

pub struct Cpu {
    register: Register,
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
        let flag_z = self.register.cpsr.is_valid_flag(CpsrFlag::Z);
        let flag_c = self.register.cpsr.is_valid_flag(CpsrFlag::C);
        let flag_n = self.register.cpsr.is_valid_flag(CpsrFlag::N);
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
