use crate::cpu::Cpu;

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
