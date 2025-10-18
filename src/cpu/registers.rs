use std::fmt;

use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Flags: u8 {
        const ZERO = 0b1000_0000;
        const ADD_SUBTRACT = 0b0100_0000;
        const HALF_CARRY = 0b0010_0000;
        const CARRY = 0b0001_0000;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Clone, Copy, Debug)]
pub struct Registers {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            pc: 0,
            sp: 0,
            a: 0,
            f: Flags::empty(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn read16(&self, reg: Reg16) -> u16 {
        use self::Reg16::*;

        match reg {
            AF => ((self.a as u16) << 8) | (self.f.bits() as u16),
            BC => ((self.b as u16) << 8) | (self.c as u16),
            DE => ((self.d as u16) << 8) | (self.e as u16),
            HL => ((self.h as u16) << 8) | (self.l as u16),
            SP => self.sp,
        }
    }

    pub fn write16(&mut self, reg: Reg16, val: u16) {
        use self::Reg16::*;

        match reg {
            AF => {
                self.a = (val >> 8) as u8;
                self.f = Flags::from_bits_truncate(val as u8);
            }
            BC => {
                self.b = (val >> 8) as u8;
                self.c = (val & 0xF) as u8;
            }
            DE => {
                self.d = (val >> 8) as u8;
                self.e = (val & 0xF) as u8;
            }
            HL => {
                self.h = (val >> 8) as u8;
                self.l = (val & 0xF) as u8;
            }
            SP => {
                self.sp = val;
            }
        }
    }

    pub fn read8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }

    pub fn write8(&mut self, reg: Reg8, val: u8) {
        match reg {
            Reg8::A => self.a = val,
            Reg8::B => self.b = val,
            Reg8::C => self.c = val,
            Reg8::D => self.d = val,
            Reg8::E => self.e = val,
            Reg8::H => self.h = val,
            Reg8::L => self.l = val,
        }
    }

    pub fn add_signed_to_sp(&mut self, offset: i8) -> u16 {
        let offset_u16 = offset as i16 as u16;
        let result = self.sp.wrapping_add(offset_u16);

        // TODO: These should probably move to the functions that call this - side-effects shouldn't be hidden
        self.set_zf(false);
        self.set_nf(false);
        self.set_hf(((self.sp & 0xF) + (offset_u16 & 0xF)) > 0xF);
        self.set_cf(((self.sp & 0xFF) + (offset_u16 & 0xFF)) > 0xFF);

        result
    }

    #[inline]
    pub fn zf(&self) -> bool {
        self.f.contains(Flags::ZERO)
    }

    #[inline]
    pub fn nf(&self) -> bool {
        self.f.contains(Flags::ADD_SUBTRACT)
    }

    #[inline]
    pub fn hf(&self) -> bool {
        self.f.contains(Flags::HALF_CARRY)
    }

    #[inline]
    pub fn cf(&self) -> bool {
        self.f.contains(Flags::CARRY)
    }

    #[inline]
    pub fn set_zf(&mut self, zf: bool) {
        self.f.set(Flags::ZERO, zf);
    }

    #[inline]
    pub fn set_nf(&mut self, nf: bool) {
        self.f.set(Flags::ADD_SUBTRACT, nf);
    }

    #[inline]
    pub fn set_hf(&mut self, hf: bool) {
        self.f.set(Flags::HALF_CARRY, hf);
    }

    #[inline]
    pub fn set_cf(&mut self, cf: bool) {
        self.f.set(Flags::CARRY, cf);
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PC: 0x{:04X} SP: 0x{:04X} \
            A: 0x{:02X} F: 0b{:04b} B: 0x{:02X} C: 0x{:02X} \
            D: 0x{:02X} E: 0b{:04b} H: 0x{:02X} L: 0x{:02X}",
            self.pc, self.sp, self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l
        )
    }
}
