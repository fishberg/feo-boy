//! Additional functionality for working with bytes.

/// Extension trait providing additional methods for `u8`.
pub trait ByteExt {
    /// Returns whether the byte has its nth bit set.
    fn has_bit_set(&self, n: u8) -> bool;

    /// If `set` is true, flips bit `n` on, and vice-versa.
    fn set_bit(&mut self, n: u8, set: bool);
}

impl ByteExt for u8 {
    fn has_bit_set(&self, n: u8) -> bool {
        if n > 7 {
            panic!("bit {} is out of range for u8", n);
        }

        (self & (1 << n)) != 0
    }

    fn set_bit(&mut self, n: u8, set: bool) {
        if n > 7 {
            panic!("bit {} is out of range for u8", n);
        }

        if set {
            *self |= 1 << n;
        } else {
            *self &= !(1 << n);
        }
    }
}

/// Extension trait providing additional methods for `u16`.
pub trait WordExt {
    /// Returns the low byte (bits 0-7) of the word.
    fn lo(&self) -> u8;

    /// Returns the high byte (bits 8-15) of the word.
    fn hi(&self) -> u8;
}

impl WordExt for u16 {
    fn lo(&self) -> u8 {
        *self as u8
    }

    fn hi(&self) -> u8 {
        ((self >> 8) & 0xff_u16) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{ByteExt, WordExt};

    #[test]
    fn has_bit_set() {
        let byte = 0x80;
        assert!(byte.has_bit_set(7));
        assert!(!byte.has_bit_set(0));
    }

    #[test]
    #[should_panic(expected = "bit 8 is out of range for u8")]
    fn has_bit_out_of_range() {
        0xFF.has_bit_set(8);
    }

    #[test]
    fn set_bit() {
        let mut byte = 0xF0;
        byte.set_bit(0, true);
        byte.set_bit(1, false);
        byte.set_bit(3, true);
        byte.set_bit(7, false);
        byte.set_bit(6, true);

        assert_eq!(byte, 0b01111001);
    }

    #[test]
    #[should_panic(expected = "bit 8 is out of range for u8")]
    fn set_bit_out_of_range() {
        let mut byte = 0xFF;
        byte.set_bit(8, true);
    }

    #[test]
    fn high_and_low() {
        assert_eq!(0xabcd.lo(), 0xcd);
        assert_eq!(0xabcd.hi(), 0xab);
        assert_eq!(0xff00.lo(), 0x00);
        assert_eq!(0xff00.hi(), 0xff);
    }
}
