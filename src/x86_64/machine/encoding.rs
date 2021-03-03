pub const LOCK: u8 = 0xF0;
pub const OVERRIDE_OPERAND: u8 = 0x66;
pub const OVERRIDE_ADDRESS: u8 = 0x67;
pub const MULTI_OPCODE_ESCAPE_PREFIX: u8 = 0x0F;
pub const MOD_REGISTER_INDIRECT: u8 = 0b0000_0000;
pub const MOD_ONE_BYTE_SIGNED_DISPLACE: u8 = 0b0000_0001;
pub const MOD_FOUR_BYTE_SIGNED_DISPLACE: u8 = 0b0000_0010;
pub const MOD_REGISTER_ADDRESSING: u8 = 0b0000_0011;
pub const SIB_SCALE_1: u8 = 0b0000_0000;
pub const SIB_SCALE_2: u8 = 0b0000_0001;
pub const SIB_SCALE_4: u8 = 0b0000_0010;
pub const SIB_SCALE_8: u8 = 0b0000_0011;
pub const SEGMENT_OVERRIDE_CS: u8 = 0x2E;
pub const SEGMENT_OVERRIDE_SS: u8 = 0x36;
pub const SEGMENT_OVERRIDE_DS: u8 = 0x3E;
pub const SEGMENT_OVERRIDE_ES: u8 = 0x26;
pub const SEGMENT_OVERRIDE_FS: u8 = 0x64;
pub const SEGMENT_OVERRIDE_GS: u8 = 0x65;

#[inline]
pub const fn pack_byte_mod_rm_sib(mut bits_01: u8, mut bits_234: u8, mut bits_567: u8) -> u8 {
    bits_567 &= !0b1111_1000;
    bits_234 &= !0b1111_1000;
    bits_01 &= !0b1111_1100;
    bits_234 <<= 3;
    bits_01 <<= 6;
    bits_567 |= bits_234;
    bits_567 |= bits_01;
    bits_567
}

#[inline]
pub const fn pack_byte_rex_prefix(w: bool, r: bool, x: bool, b: bool) -> u8 {
    let mut rex = 0b0100_0000;
    rex ^= ((b as u8).wrapping_neg() ^ rex) & 1 << 0;
    rex ^= ((x as u8).wrapping_neg() ^ rex) & 1 << 1;
    rex ^= ((r as u8).wrapping_neg() ^ rex) & 1 << 2;
    rex ^= ((w as u8).wrapping_neg() ^ rex) & 1 << 3;
    rex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_mod_rm_sib() {
        let modrm = pack_byte_mod_rm_sib(0b11, 0b101, 0b100);
        assert_eq!(modrm, 0b1110_1100)
    }

    #[test]
    fn pack_byte_rex_1100() {
        let rex = pack_byte_rex_prefix(true, true, false, false);
        assert_eq!(rex, 0b0100_1100);
    }

    #[test]
    fn pack_byte_rex_0000() {
        let rex = pack_byte_rex_prefix(false, false, false, false);
        assert_eq!(rex, 0b0100_0000);
    }

    #[test]
    fn pack_byte_rex_1111() {
        let rex = pack_byte_rex_prefix(true, true, true, true);
        assert_eq!(rex, 0b0100_1111);
    }

    #[test]
    fn pack_byte_rex_1010() {
        let rex = pack_byte_rex_prefix(true, false, true, false);
        assert_eq!(rex, 0b0100_1010);
    }

    #[test]
    fn pack_byte_rex_x48() {
        let rex = pack_byte_rex_prefix(true, false, false, false);
        assert_eq!(rex, 0x48);
    }
}
