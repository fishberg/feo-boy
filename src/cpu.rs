//! Functionality related to the CPU
//!
//! Contains an implementation of the registers and instruction set.

use std::default::Default;

use memory::Mmu;

/// The registers.
#[derive(Debug, Default)]
pub struct Registers {
    /// Accumulator
    pub a: u8,

    /// Flags
    pub f: u8,

    // General registers
    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    /// Program counter
    pub pc: u16,

    /// Stack pointer
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read_bc(&self) -> u16 {
        self.c as u16 + ((self.b as u16) << 8)
    }

    pub fn read_de(&self) -> u16 {
        self.e as u16 + ((self.d as u16) << 8)
    }

    pub fn read_hl(&self) -> u16 {
        self.l as u16 + ((self.h as u16) << 8)
    }

    pub fn write_bc(&mut self, value: u16) {
        self.c = value as u8;
        self.b = (value >> 8) as u8;
    }

    pub fn write_de(&mut self, value: u16) {
        self.e = value as u8;
        self.d = (value >> 8) as u8;
    }

    pub fn write_hl(&mut self, value: u16) {
        self.l = value as u8;
        self.h = (value >> 8) as u8;
    }
}

/// The clock.
#[derive(Debug, Default)]
pub struct Clock {
    /// Machine cycle state. One machine cycle = 4 clock cycles.
    pub m: u32,
    /// Clock cycle state.
    pub t: u32,
}

impl Clock {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn reset(&mut self) {
        self.m = 0;
        self.t = 0;
    }
}

/// The CPU.
#[derive(Debug)]
pub struct Cpu<'a> {
    /// Registers
    pub r: Registers,
    /// The clock corresponding to the last instruction cycle.
    pub clock: Clock,
    /// Memory unit
    pub mmu: &'a Mmu,
}

impl<'a> Cpu<'a> {
    pub fn new(mmu: &Mmu) -> Cpu {
        Cpu {
            r: Registers::new(),
            clock: Clock::new(),
            mmu: mmu,
        }
    }
}
