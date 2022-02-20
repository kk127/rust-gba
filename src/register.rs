use num_derive::FromPrimitive;

use crate::cpu::CpuState;

pub struct Register {
    r0_r7: [u32; 8],
    r8_r12: [u32; 10],
    r13_r14: [u32; 12],
    r15: u32,

    pub(super) cpsr: CPSR,
    spsr: [CPSR; 6],
}

impl Register {
    pub fn new() -> Self {
        Register {
            r0_r7: [0; 8],
            r8_r12: [0; 10],
            r13_r14: [0; 12],
            r15: 0,

            cpsr: CPSR::new(0x0000_00D3),
            spsr: [CPSR::new(0); 6],
        }
    }

    pub fn read(&self, index: usize) -> u32 {
        match index {
            0..=7 => self.r0_r7[index],
            8..=12 => self.read_r8_r12(index),
            13..=14 => self.read_r13_r14(index),
            15 => self.r15,
            _ => panic!("Register read error: Can't read from {} index", index),
        }
    }

    pub fn write(&mut self, index: usize, value: u32) {
        match index {
            0..=7 => self.r0_r7[index] = value,
            8..=12 => self.write_r8_r12(index, value),
            13..=14 => self.write_r13_r14(index, value),
            15 => self.r15 = value,
            _ => panic!(
                "Register write error: Can't write to {} index, value: {}",
                index, value
            ),
        }
    }

    /// r8_r12[0]: R8
    /// r8_r12[1]: R8_FIQ
    /// ...
    /// r8_r12[8]: R12
    /// r8_r12[9]: R12_FIQ
    fn read_r8_r12(&self, index: usize) -> u32 {
        let bank_index = self.get_bank_index_r8_r12();
        let register_index = 2 * (index - 8) + bank_index;
        self.r8_r12[register_index]
    }

    fn write_r8_r12(&mut self, index: usize, value: u32) {
        let bank_index = self.get_bank_index_r8_r12();
        let register_index = 2 * (index - 8) + bank_index;
        self.r8_r12[register_index] = value;
    }

    /// r13_r14[0]: R13
    /// r13_r14[1]: R13_FIQ
    /// ...
    /// r13_r14[10]: R14_IRQ
    /// r13_r14[11]: R14_Undefined
    fn read_r13_r14(&self, index: usize) -> u32 {
        let bank_index = self.get_bank_index_r13_14();
        let register_index = 6 * (index - 13) + bank_index;
        self.r13_r14[register_index]
    }

    fn write_r13_r14(&mut self, index: usize, value: u32) {
        let bank_index = self.get_bank_index_r13_14();
        let register_index = 6 * (index - 13) + bank_index;
        self.r13_r14[register_index] = value;
    }

    fn get_bank_index_r8_r12(&self) -> usize {
        match self.cpsr.get_mode() {
            CPUMode::FIQ => 1,
            _ => 0,
        }
    }

    #[rustfmt::skip]
    fn get_bank_index_r13_14(&self) -> usize {
        match self.cpsr.get_mode() {
            CPUMode::System | CPUMode::User => 0,
            CPUMode::FIQ                    => 1,
            CPUMode::Supervisor             => 2,
            CPUMode::Abort                  => 3,
            CPUMode::IRQ                    => 4,
            CPUMode::Undefined              => 5,
        }
    }

    /// Used for test only
    #[rustfmt::skip]
    fn set_mode(&mut self, mode: CPUMode) {
        match mode {
            CPUMode::User       => self.cpsr.write((self.cpsr.read() & 0b00000) | 0b10000),
            CPUMode::FIQ        => self.cpsr.write((self.cpsr.read() & 0b00000) | 0b10001),
            CPUMode::IRQ        => self.cpsr.write((self.cpsr.read() & 0b00000) | 0b10010),
            CPUMode::Supervisor => self.cpsr.write((self.cpsr.read() & 0b00000) | 0b10011),
            CPUMode::Abort      => self.cpsr.write((self.cpsr.read() & 0b00000) | 0b10111),
            CPUMode::Undefined  => self.cpsr.write((self.cpsr.read() & 0b00000) | 0b11011),
            CPUMode::System     => self.cpsr.write((self.cpsr.read() & 0b00000) | 0b11111),
        }
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, FromPrimitive, Debug, Clone, Copy)]
enum CPUMode {
    User       = 0b10000,
    FIQ        = 0b10001,
    IRQ        = 0b10010,
    Supervisor = 0b10011,
    Abort      = 0b10111,
    Undefined  = 0b11011,
    System     = 0b11111,
}

pub enum CpsrFlag {
    N,
    Z,
    C,
    V,
}

#[derive(Clone, Copy)]
pub struct CPSR(u32);

impl CPSR {
    pub fn new(x: u32) -> Self {
        CPSR(x)
    }

    pub(crate) fn is_valid_flag(&self, flag: CpsrFlag) -> bool {
        match flag {
            CpsrFlag::N => ((self.0 >> 31) & 1) == 1,
            CpsrFlag::Z => ((self.0 >> 30) & 1) == 1,
            CpsrFlag::C => ((self.0 >> 29) & 1) == 1,
            CpsrFlag::V => ((self.0 >> 28) & 1) == 1,
        }
    }

    pub(crate) fn set_cpu_state(&mut self, state: CpuState) {
        match state {
            CpuState::THUMB => self.0 |= 0x0000_0020,
            CpuState::ARM => self.0 &= 0xffff_ffdf,
        }
    }

    pub(crate) fn get_cpu_state(&self) -> CpuState {
        if (self.0 >> 5) & 1 == 1 {
            CpuState::THUMB
        } else {
            CpuState::ARM
        }
    }

    fn get_mode(&self) -> CPUMode {
        let index = self.0 & 0b11111;
        num::FromPrimitive::from_u32(index).unwrap()
    }

    fn write(&mut self, value: u32) {
        self.0 = value;
    }

    fn read(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_write_read_r0_r7() {
        let mut register = Register::new();

        let cpu_modes = vec![
            CPUMode::System,
            CPUMode::User,
            CPUMode::FIQ,
            CPUMode::Supervisor,
            CPUMode::Abort,
            CPUMode::IRQ,
            CPUMode::Undefined,
        ];

        register.set_mode(CPUMode::System);
        for index in 0..8 {
            register.write(index, index as u32 + 10);
        }

        for index in 0..8 {
            for cpu_mode in &cpu_modes {
                register.set_mode(*cpu_mode);
                assert_eq!(register.read(index), index as u32 + 10);
            }
        }
    }

    #[test]
    fn register_write_read_r8_r12() {
        let mut register = Register::new();
        for index in 8..13 {
            for cpu_mode in [CPUMode::System, CPUMode::FIQ] {
                register.set_mode(cpu_mode);
                let value = match cpu_mode {
                    CPUMode::System => index as u32,
                    CPUMode::FIQ => index as u32 + 10,
                    _ => panic!(""),
                };
                register.write(index, value)
            }
        }

        for index in 8..13 {
            let cpu_modes = vec![
                CPUMode::System,
                CPUMode::User,
                CPUMode::FIQ,
                CPUMode::Supervisor,
                CPUMode::Abort,
                CPUMode::IRQ,
                CPUMode::Undefined,
            ];

            for cpu_mode in &cpu_modes {
                register.set_mode(*cpu_mode);
                let answer = if *cpu_mode == CPUMode::FIQ {
                    index as u32 + 10
                } else {
                    index as u32
                };

                assert_eq!(register.read(index), answer);
            }
        }
    }

    #[test]
    fn register_write_read_13_r14() {
        let mut register = Register::new();

        let cpu_modes_other_than_user = vec![
            CPUMode::System,
            CPUMode::FIQ,
            CPUMode::Supervisor,
            CPUMode::Abort,
            CPUMode::IRQ,
            CPUMode::Undefined,
        ];

        for register_index in 13..15 {
            for (mode_index, cpu_mode) in cpu_modes_other_than_user.iter().enumerate() {
                register.set_mode(*cpu_mode);
                let value = register_index as u32 + mode_index as u32;
                register.write(register_index, value)
            }
        }

        let cpu_modes_all = vec![
            CPUMode::System,
            CPUMode::User,
            CPUMode::FIQ,
            CPUMode::Supervisor,
            CPUMode::Abort,
            CPUMode::IRQ,
            CPUMode::Undefined,
        ];

        for register_index in 13..15 {
            for cpu_mode in &cpu_modes_all {
                register.set_mode(*cpu_mode);

                let answer = match *cpu_mode {
                    CPUMode::System | CPUMode::User => register_index,
                    CPUMode::FIQ => register_index + 1,
                    CPUMode::Supervisor => register_index + 2,
                    CPUMode::Abort => register_index + 3,
                    CPUMode::IRQ => register_index + 4,
                    CPUMode::Undefined => register_index + 5,
                };

                assert_eq!(register.read(register_index), answer as u32);
            }
        }
    }

    #[test]
    fn register_write_read_r15() {
        let mut register = Register::new();
        register.set_mode(CPUMode::System);
        register.write(15, 10);

        let cpu_modes_all = vec![
            CPUMode::System,
            CPUMode::User,
            CPUMode::FIQ,
            CPUMode::Supervisor,
            CPUMode::Abort,
            CPUMode::IRQ,
            CPUMode::Undefined,
        ];

        for cpu_mode in &cpu_modes_all {
            register.set_mode(*cpu_mode);
            assert_eq!(register.read(15), 10);
        }
    }

    #[test]
    fn is_valid_flag() {
        let cpsr = CPSR::new(0);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::N), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::Z), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::C), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::V), false);

        let cpsr = CPSR::new(0b1000_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::N), true);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::Z), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::C), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::V), false);

        let cpsr = CPSR::new(0b0100_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::N), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::Z), true);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::C), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::V), false);

        let cpsr = CPSR::new(0b0010_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::N), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::Z), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::C), true);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::V), false);

        let cpsr = CPSR::new(0b0001_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::N), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::Z), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::C), false);
        assert_eq!(cpsr.is_valid_flag(CpsrFlag::V), true);
    }

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
            assert_eq!(cpsr.get_mode(), answer)
        }
    }

    #[test]
    fn cpu_state() {
        let mut register = Register::new();
        // initial cpu state of cpsr is ARM.
        assert_eq!(register.cpsr.get_cpu_state(), CpuState::ARM);

        register.cpsr.set_cpu_state(CpuState::THUMB);
        assert_eq!(register.cpsr.get_cpu_state(), CpuState::THUMB);

        register.cpsr.set_cpu_state(CpuState::ARM);
        assert_eq!(register.cpsr.get_cpu_state(), CpuState::ARM);
    }
}
