use crate::cpu::registers::Registers;

mod instructions;
mod registers;

#[derive(Clone)]
pub struct Cpu {
    pub regs: Registers,
    pub ime: bool,
}
