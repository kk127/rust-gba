use crate::cpu::{Cpu, CpuState};
use crate::register::CpsrFlag;

#[derive(PartialEq, Eq, Debug)]
enum ArmInstruction {
    SWI,
    BL,
    B,
    BX,
    LDM,
    STM,
    LDR,
    STR,
    LDRH,
    LDRSB,
    LDRSH,
    STRH,
    MRS,
    MSR,
    SWP,
    MPY,
    ALU,
}

impl Cpu {
    fn arm_decode(&self, inst: u32) -> ArmInstruction {
        if (inst & 0x0f00_0000) == 0x0f00_0000 {
            ArmInstruction::SWI
        } else if (inst & 0x0f00_0000) == 0x0b00_0000 {
            ArmInstruction::BL
        } else if (inst & 0x0f00_0000) == 0x0a00_0000 {
            ArmInstruction::B
        } else if (inst & 0x0fff_fff0) == 0x012f_ff10 {
            ArmInstruction::BX
        } else if (inst & 0x0e10_0000) == 0x0810_0000 {
            ArmInstruction::LDM
        } else if (inst & 0x0e10_0000) == 0x0800_0000 {
            ArmInstruction::STM
        } else if (inst & 0x0c10_0000) == 0x0410_0000 {
            ArmInstruction::LDR
        } else if (inst & 0x0c10_0000) == 0x0400_0000 {
            ArmInstruction::STR
        } else if (inst & 0x0e10_00f0) == 0x0010_00b0 {
            ArmInstruction::LDRH
        } else if (inst & 0x0e10_00f0) == 0x0010_00d0 {
            ArmInstruction::LDRSB
        } else if (inst & 0x0e10_00f0) == 0x0010_00f0 {
            ArmInstruction::LDRSH
        } else if (inst & 0x0e10_00f0) == 0x0000_00b0 {
            ArmInstruction::STRH
        } else if (inst & 0x0fbf_0fff) == 0x010f_0000 {
            ArmInstruction::MRS
        } else if (inst & 0x0db0_f000) == 0x0120_f000 {
            ArmInstruction::MSR
        } else if (inst & 0x0fb0_0ff0) == 0x0100_0090 {
            ArmInstruction::SWP
        } else if (inst & 0x0e00_00f0) == 0x0000_0090 {
            ArmInstruction::MPY
        } else if (inst & 0x0c00_0000) == 0x0000_0000 {
            ArmInstruction::ALU
        } else {
            panic!("Arm instruction decode error: Can't decode {:08x}", inst);
        }
    }

    fn arm_b(&mut self, inst: u32) {
        let offset = ((inst as i32) << 8) >> 6;
        let pc = (self.register.read(15) as i32).wrapping_add(offset) as u32;
        self.register.write(15, pc);
        // TODO pipeline
    }

    fn arm_bl(&mut self, inst: u32) {
        let offset = ((inst as i32) << 8) >> 6;
        let pc = (self.register.read(15) as i32).wrapping_add(offset) as u32;
        self.register
            .write(14, self.register.read(15).wrapping_add(4));
        self.register.write(15, pc);
        // TODO pipeline
    }

    fn arm_bx(&mut self, inst: u32) {
        let register_index = (inst & 0b1111) as usize;
        let register_value = self.register.read(register_index);

        if register_value & 1 == 1 {
            self.register.cpsr.set_cpu_state(CpuState::THUMB);
            self.register.write(15, register_value);
            // TODO pipeline
        } else {
            self.register.cpsr.set_cpu_state(CpuState::ARM);
            self.register.write(15, register_value);
            // TODO pipeline
        }
    }

    fn arm_mpy(&mut self, inst: u32) {
        let opcode = (inst >> 21) & 0b1111;
        match opcode {
            0b0000 => self.arm_mul(inst),
            0b0001 => self.arm_mla(inst),
            0b0010 => panic!("UMAAL is not supported."),
            0b0100 => self.arm_umull(inst),
            0b0101 => self.arm_umlal(inst),
            0b0110 => self.arm_smull(inst),
            0b0111 => self.arm_smlal(inst),
            _ => panic!("Invalid MPY opcode: {:8x}, inst: {:8x}", opcode, inst),
        }
    }

    fn arm_mul(&mut self, inst: u32) {
        let rd_index = ((inst >> 16) & 0b1111) as usize;
        let rs_index = ((inst >> 8) & 0b1111) as usize;
        let rm_index = (inst & 0b1111) as usize;

        let rs = self.register.read(rs_index);
        let rm = self.register.read(rm_index);
        let result = rm * rs;

        self.register.write(rd_index, result);

        if (inst >> 20) & 1 == 1 {
            let flag_n = (result >> 31) & 1 == 1;
            let flag_z = result == 0;

            self.register.cpsr.set_nzcv_flag(CpsrFlag::N, flag_n);
            self.register.cpsr.set_nzcv_flag(CpsrFlag::Z, flag_z);
        }
        // TODO cycle
    }

    fn arm_mla(&mut self, inst: u32) {
        let rd_index = ((inst >> 16) & 0b1111) as usize;
        let rn_index = ((inst >> 12) & 0b1111) as usize;
        let rs_index = ((inst >> 8) & 0b1111) as usize;
        let rm_index = (inst & 0b1111) as usize;

        let rn = self.register.read(rn_index);
        let rs = self.register.read(rs_index);
        let rm = self.register.read(rm_index);
        let result = rm * rs + rn;

        self.register.write(rd_index, result);

        if (inst >> 20) & 1 == 1 {
            let flag_n = (result >> 31) & 1 == 1;
            let flag_z = result == 0;

            self.register.cpsr.set_nzcv_flag(CpsrFlag::N, flag_n);
            self.register.cpsr.set_nzcv_flag(CpsrFlag::Z, flag_z);
        }
        // TODO cycle
    }

    fn arm_umull(&mut self, inst: u32) {
        let rdhi_index = ((inst >> 16) & 0b1111) as usize;
        let rdlo_index = ((inst >> 12) & 0b1111) as usize;
        let rs_index = ((inst >> 8) & 0b1111) as usize;
        let rm_index = (inst & 0b1111) as usize;

        let rs = self.register.read(rs_index) as u64;
        let rm = self.register.read(rm_index) as u64;
        let result = rm * rs;

        let rdhi = (result >> 32) as u32;
        let rdlo = result as u32;
        self.register.write(rdhi_index, rdhi);
        self.register.write(rdlo_index, rdlo);

        if (inst >> 20) & 1 == 1 {
            let flag_n = (result >> 63) & 1 == 1;
            let flag_z = result == 0;

            self.register.cpsr.set_nzcv_flag(CpsrFlag::N, flag_n);
            self.register.cpsr.set_nzcv_flag(CpsrFlag::Z, flag_z);
        }

        // TODO cycle
    }

    fn arm_umlal(&mut self, inst: u32) {
        let rdhi_index = ((inst >> 16) & 0b1111) as usize;
        let rdlo_index = ((inst >> 12) & 0b1111) as usize;
        let rs_index = ((inst >> 8) & 0b1111) as usize;
        let rm_index = (inst & 0b1111) as usize;

        let rdhi = self.register.read(rdhi_index) as u64;
        let rdlo = self.register.read(rdlo_index) as u64;
        let rs = self.register.read(rs_index) as u64;
        let rm = self.register.read(rm_index) as u64;
        let result = rm * rs + (rdhi << 32 | rdlo);

        let rdhi = (result >> 32) as u32;
        let rdlo = result as u32;
        self.register.write(rdhi_index, rdhi);
        self.register.write(rdlo_index, rdlo);

        if (inst >> 20) & 1 == 1 {
            let flag_n = (result >> 63) & 1 == 1;
            let flag_z = result == 0;

            self.register.cpsr.set_nzcv_flag(CpsrFlag::N, flag_n);
            self.register.cpsr.set_nzcv_flag(CpsrFlag::Z, flag_z);
        }
        // TODO cycle
    }

    fn arm_smull(&mut self, inst: u32) {
        let rdhi_index = ((inst >> 16) & 0b1111) as usize;
        let rdlo_index = ((inst >> 12) & 0b1111) as usize;
        let rs_index = ((inst >> 8) & 0b1111) as usize;
        let rm_index = (inst & 0b1111) as usize;

        let rs = (self.register.read(rs_index) as i32) as i64;
        let rm = (self.register.read(rm_index) as i32) as i64;
        let result = (rm * rs) as u64;

        let rdhi = (result >> 32) as u32;
        let rdlo = result as u32;
        self.register.write(rdhi_index, rdhi);
        self.register.write(rdlo_index, rdlo);

        if (inst >> 20) & 1 == 1 {
            let flag_n = (result >> 63) & 1 == 1;
            let flag_z = result == 0;

            self.register.cpsr.set_nzcv_flag(CpsrFlag::N, flag_n);
            self.register.cpsr.set_nzcv_flag(CpsrFlag::Z, flag_z);
        }

        // TODO cycle
    }

    fn arm_smlal(&mut self, inst: u32) {
        let rdhi_index = ((inst >> 16) & 0b1111) as usize;
        let rdlo_index = ((inst >> 12) & 0b1111) as usize;
        let rs_index = ((inst >> 8) & 0b1111) as usize;
        let rm_index = (inst & 0b1111) as usize;

        let rdhi = (self.register.read(rdhi_index) as i32) as i64;
        let rdlo = (self.register.read(rdlo_index) as i32) as i64;
        let rs = (self.register.read(rs_index) as i32) as i64;
        let rm = (self.register.read(rm_index) as i32) as i64;
        let result = (rm * rs + (rdhi << 32 | rdlo)) as u64;

        let rdhi = (result >> 32) as u32;
        let rdlo = result as u32;
        self.register.write(rdhi_index, rdhi);
        self.register.write(rdlo_index, rdlo);

        if (inst >> 20) & 1 == 1 {
            let flag_n = (result >> 63) & 1 == 1;
            let flag_z = result == 0;

            self.register.cpsr.set_nzcv_flag(CpsrFlag::N, flag_n);
            self.register.cpsr.set_nzcv_flag(CpsrFlag::Z, flag_z);
        }
        // TODO cycle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm_decode() {
        let instructions = vec![
            0b0000_1111_0000_0000_0000_0000_0000_0000, // SWI
            0b0000_1011_0000_0000_0000_0000_0000_0000, // BL
            0b0000_1010_0000_0000_0000_0000_0000_0000, // B
            0b0000_0001_0010_1111_1111_1111_0001_0000, // BX
            0b0000_1000_0001_0000_0000_0000_0000_0000, // LDM
            0b0000_1000_0000_0000_0000_0000_0000_0000, // STM
            0b0000_0100_0001_0000_0000_0000_0000_0000, // LDR
            0b0000_0100_0000_0000_0000_0000_0000_0000, // STR
            0b0000_0000_0001_0000_0000_0000_1011_0000, // LDRH
            0b0000_0000_0001_0000_0000_0000_1101_0000, // LDRSB
            0b0000_0000_0001_0000_0000_0000_1111_0000, // LDRSH
            0b0000_0000_0000_0000_0000_0000_1011_0000, // STRH
            0b0000_0001_0000_1111_0000_0000_0000_0000, // MRS
            0b0000_0001_0010_0000_1111_0000_0000_0000, // MSR
            0b0000_0001_0000_0000_0000_0000_1001_0000, // SWP
            0b0000_0000_0000_0000_0000_0000_1001_0000, // MPY
            0b0000_0000_0000_0000_0000_0000_0000_0000, // ALU
        ];

        let answers = vec![
            ArmInstruction::SWI,
            ArmInstruction::BL,
            ArmInstruction::B,
            ArmInstruction::BX,
            ArmInstruction::LDM,
            ArmInstruction::STM,
            ArmInstruction::LDR,
            ArmInstruction::STR,
            ArmInstruction::LDRH,
            ArmInstruction::LDRSB,
            ArmInstruction::LDRSH,
            ArmInstruction::STRH,
            ArmInstruction::MRS,
            ArmInstruction::MSR,
            ArmInstruction::SWP,
            ArmInstruction::MPY,
            ArmInstruction::ALU,
        ];

        let cpu = Cpu::new();
        for (inst, answer) in instructions.iter().zip(answers.iter()) {
            assert_eq!(cpu.arm_decode(*inst), *answer);
        }
    }
}
