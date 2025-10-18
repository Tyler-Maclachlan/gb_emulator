use crate::cpu::registers::{Reg8, Reg16};

// 8-bit operands used by arithmetic, load, etc.
#[derive(Clone, Copy, Debug)]
pub enum Operand8 {
    Reg(Reg8),       // A, B, C, D, E, H, L
    Immediate(u8),   // n8
    IndirectHL,      // (HL)
    IndirectHLInc,   // (HL+) — HL is incremented after access
    IndirectHLDec,   // (HL-) — HL is decremented after access
    Indirect(Reg16), // (BC), (DE)
}

// 16-bit operands used by load, jumps, stack ops, etc
#[derive(Clone, Copy, Debug)]
pub enum Operand16 {
    Reg(Reg16),     // BC, DE, HL
    Immediate(u16), // n16
    Address(u16),
}

#[derive(Clone, Copy, Debug)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
}

// CB-prefixed instructions
#[derive(Clone, Copy, Debug)]
pub enum CBInstruction {
    RLC(Reg8),
    RRC(Reg8),
    RL(Reg8),
    RR(Reg8),
    SLA(Reg8),
    SRA(Reg8),
    SWAP(Reg8),
    SRL(Reg8),
    BIT(u8, Reg8),
    RES(u8, Reg8),
    SET(u8, Reg8),
}

pub enum Instruction {
    // 8-bit loads
    LD(Operand8, Operand8),

    // 16-bit loads / stack ops
    LD16(Operand16, Operand16),
    PUSH(Reg16),
    POP(Reg16),
    LDSPHL, // LD SP, HL

    // 8-bit arithmetic & logic
    ADD(Operand8),
    ADC(Operand8),
    SUB(Operand8),
    SBC(Operand8),
    AND(Operand8),
    OR(Operand8),
    XOR(Operand8),
    CP(Operand8),
    INC(Operand8),
    DEC(Operand8),

    // 16-bit arithmetic
    ADDHL(Reg16),
    INC16(Reg16),
    DEC16(Reg16),

    // Special
    ADDSPIMM(i8),
    LDHLSPPLUS(i8),

    // Rotate / shift (non-CB)
    RLCA,
    RLA,
    RRCA,
    RRA,

    // Misc
    DAA,
    CPL,
    SCF,
    CCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,

    // Jumps / calls / returns
    JP(Option<Condition>, Operand16),
    JR(Option<Condition>, i8),
    CALL(Option<Condition>, Operand16),
    RET(Option<Condition>),
    RETI,
    RST(u8),

    // CB-prefixed
    CB(CBInstruction),
}

pub struct DecodedInstruction {
    pub instruction: Instruction,
    pub length: u8,       // Number of bytes
    pub cycles: u8,       // Base machine cycles
    pub extra_cycles: u8, // Extra cyles needed for branching (jumps/calls)
}

pub fn decode(opcode: u8, next1: u8, next2: u8) -> DecodedInstruction {
    use Condition::*;
    use Instruction::*;
    use Operand8::*;
    use Operand16::{Address, Immediate as Immediate16, Reg as O16Reg};

    match opcode {
        // NOP
        0x00 => DecodedInstruction {
            instruction: NOP,
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD BC n16
        0x01 => {
            let imm = u16::from_le_bytes([next1, next2]);

            DecodedInstruction {
                instruction: LD16(O16Reg(Reg16::BC), Immediate16(imm)),
                length: 3,
                cycles: 12,
                extra_cycles: 0,
            }
        }
        // LD BC A
        0x02 => DecodedInstruction {
            instruction: LD(Indirect(Reg16::BC), Reg(Reg8::A)),
            length: 1,
            cycles: 8,
            extra_cycles: 0,
        },
        // INC BC
        0x03 => DecodedInstruction {
            instruction: INC16(Reg16::BC),
            length: 1,
            cycles: 8,
            extra_cycles: 0,
        },
        // ADD HL BC
        0x09 => DecodedInstruction {
            instruction: ADDHL(Reg16::BC),
            length: 1,
            cycles: 8,
            extra_cycles: 0,
        },
        // ADD HL DE
        0x19 => DecodedInstruction {
            instruction: ADDHL(Reg16::DE),
            length: 1,
            cycles: 8,
            extra_cycles: 0,
        },
        // ADD HL HL
        0x29 => DecodedInstruction {
            instruction: ADDHL(Reg16::HL),
            length: 1,
            cycles: 8,
            extra_cycles: 0,
        },
        // ADD HL SP
        0x39 => DecodedInstruction {
            instruction: ADDHL(Reg16::SP),
            length: 1,
            cycles: 8,
            extra_cycles: 0,
        },
        // LD B B
        0x40 => DecodedInstruction {
            instruction: LD(Reg(Reg8::B), Reg(Reg8::B)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD B C
        0x41 => DecodedInstruction {
            instruction: LD(Reg(Reg8::B), Reg(Reg8::C)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD B D
        0x42 => DecodedInstruction {
            instruction: LD(Reg(Reg8::B), Reg(Reg8::D)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD B E
        0x43 => DecodedInstruction {
            instruction: LD(Reg(Reg8::B), Reg(Reg8::E)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD B H
        0x44 => DecodedInstruction {
            instruction: LD(Reg(Reg8::B), Reg(Reg8::H)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD B L
        0x45 => DecodedInstruction {
            instruction: LD(Reg(Reg8::B), Reg(Reg8::L)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD B A
        0x47 => DecodedInstruction {
            instruction: LD(Reg(Reg8::B), Reg(Reg8::A)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD C B
        0x48 => DecodedInstruction {
            instruction: LD(Reg(Reg8::C), Reg(Reg8::B)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD C C
        0x49 => DecodedInstruction {
            instruction: LD(Reg(Reg8::C), Reg(Reg8::C)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD C D
        0x4A => DecodedInstruction {
            instruction: LD(Reg(Reg8::C), Reg(Reg8::D)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD C E
        0x4B => DecodedInstruction {
            instruction: LD(Reg(Reg8::C), Reg(Reg8::E)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD C H
        0x4C => DecodedInstruction {
            instruction: LD(Reg(Reg8::C), Reg(Reg8::H)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD C L
        0x4D => DecodedInstruction {
            instruction: LD(Reg(Reg8::C), Reg(Reg8::L)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD C A
        0x4F => DecodedInstruction {
            instruction: LD(Reg(Reg8::C), Reg(Reg8::A)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD D B
        0x50 => DecodedInstruction {
            instruction: LD(Reg(Reg8::D), Reg(Reg8::B)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD D C
        0x51 => DecodedInstruction {
            instruction: LD(Reg(Reg8::D), Reg(Reg8::C)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD D D
        0x52 => DecodedInstruction {
            instruction: LD(Reg(Reg8::D), Reg(Reg8::D)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD D E
        0x53 => DecodedInstruction {
            instruction: LD(Reg(Reg8::D), Reg(Reg8::E)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD D H
        0x54 => DecodedInstruction {
            instruction: LD(Reg(Reg8::D), Reg(Reg8::H)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD D L
        0x55 => DecodedInstruction {
            instruction: LD(Reg(Reg8::D), Reg(Reg8::L)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD D A
        0x57 => DecodedInstruction {
            instruction: LD(Reg(Reg8::D), Reg(Reg8::A)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD E B
        0x58 => DecodedInstruction {
            instruction: LD(Reg(Reg8::E), Reg(Reg8::B)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD E C
        0x59 => DecodedInstruction {
            instruction: LD(Reg(Reg8::E), Reg(Reg8::C)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD E D
        0x5A => DecodedInstruction {
            instruction: LD(Reg(Reg8::E), Reg(Reg8::D)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD E E
        0x5B => DecodedInstruction {
            instruction: LD(Reg(Reg8::E), Reg(Reg8::E)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD E H
        0x5C => DecodedInstruction {
            instruction: LD(Reg(Reg8::E), Reg(Reg8::H)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD E L
        0x5D => DecodedInstruction {
            instruction: LD(Reg(Reg8::E), Reg(Reg8::L)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD E A
        0x5F => DecodedInstruction {
            instruction: LD(Reg(Reg8::E), Reg(Reg8::A)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD H B
        0x60 => DecodedInstruction {
            instruction: LD(Reg(Reg8::H), Reg(Reg8::B)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD H C
        0x61 => DecodedInstruction {
            instruction: LD(Reg(Reg8::H), Reg(Reg8::C)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD H D
        0x62 => DecodedInstruction {
            instruction: LD(Reg(Reg8::H), Reg(Reg8::D)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD H E
        0x63 => DecodedInstruction {
            instruction: LD(Reg(Reg8::H), Reg(Reg8::E)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD H H
        0x64 => DecodedInstruction {
            instruction: LD(Reg(Reg8::H), Reg(Reg8::H)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD H L
        0x65 => DecodedInstruction {
            instruction: LD(Reg(Reg8::H), Reg(Reg8::L)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD H A
        0x67 => DecodedInstruction {
            instruction: LD(Reg(Reg8::H), Reg(Reg8::A)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD L B
        0x68 => DecodedInstruction {
            instruction: LD(Reg(Reg8::L), Reg(Reg8::B)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD L C
        0x69 => DecodedInstruction {
            instruction: LD(Reg(Reg8::L), Reg(Reg8::C)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD L D
        0x6A => DecodedInstruction {
            instruction: LD(Reg(Reg8::L), Reg(Reg8::D)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD L E
        0x6B => DecodedInstruction {
            instruction: LD(Reg(Reg8::L), Reg(Reg8::E)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD L H
        0x6C => DecodedInstruction {
            instruction: LD(Reg(Reg8::L), Reg(Reg8::H)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD L L
        0x6D => DecodedInstruction {
            instruction: LD(Reg(Reg8::L), Reg(Reg8::L)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD L A
        0x6F => DecodedInstruction {
            instruction: LD(Reg(Reg8::L), Reg(Reg8::A)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD A B
        0x78 => DecodedInstruction {
            instruction: LD(Reg(Reg8::A), Reg(Reg8::B)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD A C
        0x79 => DecodedInstruction {
            instruction: LD(Reg(Reg8::A), Reg(Reg8::C)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD A D
        0x7A => DecodedInstruction {
            instruction: LD(Reg(Reg8::A), Reg(Reg8::D)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD A E
        0x7B => DecodedInstruction {
            instruction: LD(Reg(Reg8::A), Reg(Reg8::E)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD A H
        0x7C => DecodedInstruction {
            instruction: LD(Reg(Reg8::A), Reg(Reg8::H)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD A L
        0x7D => DecodedInstruction {
            instruction: LD(Reg(Reg8::A), Reg(Reg8::L)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // LD A A
        0x7F => DecodedInstruction {
            instruction: LD(Reg(Reg8::A), Reg(Reg8::A)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // ADD A B
        0x80 => DecodedInstruction {
            instruction: ADD(Reg(Reg8::B)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // ADD A C
        0x81 => DecodedInstruction {
            instruction: ADD(Reg(Reg8::C)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // ADD A D
        0x82 => DecodedInstruction {
            instruction: ADD(Reg(Reg8::D)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // ADD A E
        0x83 => DecodedInstruction {
            instruction: ADD(Reg(Reg8::E)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // ADD A H
        0x84 => DecodedInstruction {
            instruction: ADD(Reg(Reg8::H)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // ADD A L
        0x85 => DecodedInstruction {
            instruction: ADD(Reg(Reg8::L)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // ADD A HL
        0x86 => DecodedInstruction {
            instruction: ADD(IndirectHL),
            length: 1,
            cycles: 8,
            extra_cycles: 0,
        },
        // ADD A A
        0x87 => DecodedInstruction {
            instruction: ADD(Reg(Reg8::A)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // SUB A B
        0x90 => DecodedInstruction {
            instruction: SUB(Reg(Reg8::B)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // SUB A C
        0x91 => DecodedInstruction {
            instruction: SUB(Reg(Reg8::C)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // SUB A D
        0x92 => DecodedInstruction {
            instruction: SUB(Reg(Reg8::D)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // SUB A E
        0x93 => DecodedInstruction {
            instruction: SUB(Reg(Reg8::E)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // SUB A H
        0x94 => DecodedInstruction {
            instruction: SUB(Reg(Reg8::H)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // SUB A L
        0x95 => DecodedInstruction {
            instruction: SUB(Reg(Reg8::L)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // SUB A HL
        0x96 => DecodedInstruction {
            instruction: SUB(IndirectHL),
            length: 1,
            cycles: 8,
            extra_cycles: 0,
        },
        // SUB A A
        0x97 => DecodedInstruction {
            instruction: SUB(Reg(Reg8::A)),
            length: 1,
            cycles: 4,
            extra_cycles: 0,
        },
        // ADD A n8
        0xC6 => DecodedInstruction {
            instruction: ADD(Immediate(next1)),
            length: 2,
            cycles: 8,
            extra_cycles: 0,
        },
        // SUB A n8
        0xD6 => DecodedInstruction {
            instruction: SUB(Immediate(next1)),
            length: 2,
            cycles: 8,
            extra_cycles: 0,
        },
        // ADD SP e8
        0xE8 => DecodedInstruction {
            instruction: ADDSPIMM(0), // placeholder - actual imm filled at exec time
            length: 2,
            cycles: 16,
            extra_cycles: 0,
        },
        0xF8 => DecodedInstruction {
            instruction: LDHLSPPLUS(0), // placeholder
            length: 2,
            cycles: 12,
            extra_cycles: 0,
        },
        _ => unimplemented!("Not yet implemented"),
    }
}
