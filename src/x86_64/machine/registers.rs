#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(usize)]
pub enum Register {
    RAX,
    EAX,
    AX,
    AH,
    AL,

    RBX,
    EBX,
    BX,
    BH,
    BL,

    RCX,
    ECX,
    CX,
    CH,
    CL,

    RDX,
    EDX,
    DX,
    DH,
    DL,

    RSI,
    ESI,
    SI,
    SIL,

    RDI,
    EDI,
    DI,
    DIL,

    RBP,
    EBP,
    BP,
    BPL,

    RSP,
    ESP,
    SP,
    SPL,

    R8,
    R8D,
    R8W,
    R8B,

    R9,
    R9D,
    R9W,
    R9B,

    R10,
    R10D,
    R10W,
    R10B,

    R11,
    R11D,
    R11W,
    R11B,

    R12,
    R12D,
    R12W,
    R12B,

    R13,
    R13D,
    R13W,
    R13B,

    R14,
    R14D,
    R14W,
    R14B,

    R15,
    R15D,
    R15W,
    R15B,

    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,

    MM0,
    MM1,
    MM2,
    MM3,
    MM4,
    MM5,
    MM6,
    MM7,

    ES,
    CS,
    SS,
    DS,
    FS,
    GS,

    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM8,
    XMM9,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,

    YMM0,
    YMM1,
    YMM2,
    YMM3,
    YMM4,
    YMM5,
    YMM6,
    YMM7,
    YMM8,
    YMM9,
    YMM10,
    YMM11,
    YMM12,
    YMM13,
    YMM14,
    YMM15,

    RIP,
    EIP,
    IP,

    CR0,
    CR1,
    CR2,
    CR3,
    CR4,
    CR5,
    CR6,
    CR7,
    CR8,
    CR9,
    CR10,
    CR11,
    CR12,
    CR13,
    CR14,
    CR15,

    DR0,
    DR1,
    DR2,
    DR3,
    DR4,
    DR5,
    DR6,
    DR7,
    DR8,
    DR9,
    DR10,
    DR11,
    DR12,
    DR13,
    DR14,
    DR15,

    ZMM0,
    ZMM1,
    ZMM2,
    ZMM3,
    ZMM4,
    ZMM5,
    ZMM6,
    ZMM7,
    ZMM8,
    ZMM9,
    ZMM10,
    ZMM11,
    ZMM12,
    ZMM13,
    ZMM14,
    ZMM15,
    ZMM16,
    ZMM17,
    ZMM18,
    ZMM19,
    ZMM20,
    ZMM21,
    ZMM22,
    ZMM23,
    ZMM24,
    ZMM25,
    ZMM26,
    ZMM27,
    ZMM28,
    ZMM29,
    ZMM30,
    ZMM31,

    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,

    BND0,
    BND1,
    BND2,
    BND3,

    Count,
}

impl Register {
    #[inline]
    pub const fn is_accumulator(&self) -> bool {
        matches!(*self, Self::AL | Self::AX | Self::EAX | Self::RAX)
    }

    #[inline]
    pub const fn is_high_byte(&self) -> bool {
        matches!(*self, Self::AH | Self::BH | Self::CH | Self::DH)
    }

    #[inline]
    pub const fn is_uniform_byte(&self) -> bool {
        matches!(*self, Self::SPL | Self::BPL | Self::SIL | Self::DIL)
    }

    #[inline]
    pub const fn is_64bit_or_larger(&self) -> bool {
        Self::SIZE_TABLE[*self as usize] >= 8
    }

    #[inline]
    pub const fn is_extended(&self) -> bool {
        *self as usize >= Self::R8 as usize && *self as usize <= Self::R15B as usize
    }

    #[inline]
    pub fn is_mmx(&self) -> bool {
        *self as usize >= Self::MM0 as usize && *self as usize <= Self::MM7 as usize
    }

    #[inline]
    pub fn is_sse(&self) -> bool {
        *self as usize >= Self::XMM0 as usize && *self as usize <= Self::XMM15 as usize
    }

    #[inline]
    pub fn is_avx(&self) -> bool {
        *self as usize >= Self::YMM0 as usize && *self as usize <= Self::YMM15 as usize
    }

    #[inline]
    pub fn is_avx512(&self) -> bool {
        self.is_avx512_mask()
            || *self as usize >= Self::ZMM0 as usize && *self as usize <= Self::ZMM31 as usize
    }

    #[inline]
    pub fn is_avx512_mask(&self) -> bool {
        *self as usize >= Self::K0 as usize && *self as usize <= Self::K7 as usize
    }

    #[inline]
    pub fn id(&self) -> u8 {
        Self::ID_TABLE[*self as usize]
    }

    #[inline]
    pub fn size(&self) -> u8 {
        Self::SIZE_TABLE[*self as usize]
    }

    /// Contains the machine code id for each register.
    pub const ID_TABLE: [u8; Self::Count as _] = [
        0x00, 0x00, 0x00, 0x04, 0x00, 0x03, 0x03, 0x03, 0x07, 0x03, 0x01, 0x01, 0x01, 0x05, 0x01,
        0x02, 0x02, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06, 0x07, 0x07, 0x07, 0x07, 0x05, 0x05,
        0x05, 0x05, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x02,
        0x02, 0x02, 0x02, 0x03, 0x03, 0x03, 0x03, 0x04, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x05,
        0x06, 0x06, 0x06, 0x06, 0x07, 0x07, 0x07, 0x07, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
        0x37, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75,
        0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E,
        0x5F, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D,
        0x5E, 0x5F, 0x15, 0x15, 0x15, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79,
        0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88,
        0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0x90, 0x91, 0x92, 0x93,
    ];

    /// Contains the size of each register in bytes.
    pub const SIZE_TABLE: [u8; Register::Count as _] = [
        8, 4, 2, 1, 1, 8, 4, 2, 1, 1, 8, 4, 2, 1, 1, 8, 4, 2, 1, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4,
        2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1,
        8, 4, 2, 1, 8, 4, 2, 1, 10, 10, 10, 10, 10, 10, 10, 10, 8, 8, 8, 8, 8, 8, 8, 8, 2, 2, 2, 2,
        2, 2, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 8, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 64, 64, 64, 64, 64, 64, 64, 64,
        64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
        64, 2, 2, 2, 2, 2, 2, 2, 2, 16, 16, 16, 16,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_id() {
        assert_eq!(Register::XMM0.id(), 0x50);
    }

    #[test]
    fn register_size() {
        assert_eq!(Register::XMM0.size(), 16);
    }

    #[test]
    fn register_mnemonic() {
        assert_eq!(Register::XMM0.mnemonic(), "xmm0");
    }
}
