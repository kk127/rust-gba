use anyhow::Result;
use num_derive::FromPrimitive;

#[rustfmt::skip]
#[derive(PartialEq, Eq, FromPrimitive, Debug)]
enum CPUMode {
    User       = 0b10000,
    FIQ        = 0b10001,
    IRQ        = 0b10010,
    Supervisor = 0b10011,
    Abort      = 0b10111,
    Undefined  = 0b11011,
    System     = 0b11111,
}

pub struct CPSR(u32);

impl CPSR {
    pub fn new(x: u32) -> Self {
        CPSR(x)
    }

    fn get_mode(&self) -> Option<CPUMode> {
        let index = self.0 & 0b11111;
        num::FromPrimitive::from_u32((index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_mode() {
        let cpsrs = vec![
            CPSR::new(0b10000),
            CPSR::new(0b10001),
            CPSR::new(0b10010),
            CPSR::new(0b10011),
            CPSR::new(0b10111),
            CPSR::new(0b11011),
            CPSR::new(0b11111),
        ];
        let answers = vec![
            CPUMode::User,
            CPUMode::FIQ,
            CPUMode::IRQ,
            CPUMode::Supervisor,
            CPUMode::Abort,
            CPUMode::Undefined,
            CPUMode::System,
        ];

        for (cpsr, answer) in cpsrs.into_iter().zip(answers.into_iter()) {
            assert_eq!(cpsr.get_mode().unwrap(), answer)
        }
    }
}
