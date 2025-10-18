use crate::cpu::registers::{Reg8, Reg16};

// 8-bit operands used by arithmetic, load, etc.
#[derive(Clone, Copy, Debug)]
pub enum Operand8 {
    Reg(Reg8),       // A, B, C, D, E, H, L
    Immediate(u8),   // d8
    IndirectHL,      // (HL)
    IndirectHLInc,   // (HL+) — HL is incremented after access
    IndirectHLDec,   // (HL-) — HL is decremented after access
    Indirect(Reg16), // (BC), (DE)
}

// 16-bit operands used by load, jumps, stack ops, etc
#[derive(Clone, Copy, Debug)]
pub enum Operand16 {
    Reg(Reg16),
    Immediate(u16),
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
    pub length: u8,              // Number of bytes
    pub cycles: u8,              // Base machine cycles
    pub extra_cycle: Option<u8>, // If an extra cycle is needed for branching (jumps/calls)
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
            extra_cycle: None,
        },
        // LD BC n16
        0x01 => {
            let imm = u16::from_le_bytes([next1, next2]);

            DecodedInstruction {
                instruction: LD16(O16Reg(Reg16::BC), Immediate16(imm)),
                length: 3,
                cycles: 12,
                extra_cycle: None,
            }
        }
        // LD BC A
        0x02 => DecodedInstruction {
            instruction: LD(Indirect(Reg16::BC), Reg(Reg8::A)),
            length: 1,
            cycles: 8,
            extra_cycle: None,
        },
        // INC BC
        0x03 => DecodedInstruction {
            instruction: INC16(Reg16::BC),
            length: 1,
            cycles: 8,
            extra_cycle: None,
        },
        _ => todo!("Not yet implemented"),
    }
}
