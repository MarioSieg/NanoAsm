#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(usize)]
pub enum Register {
    RAX = 0,
    EAX = 1,
    AX = 2,
    AH = 3,
    AL = 4,

    RBX = 5,
    EBX = 6,
    BX = 7,
    BH = 8,
    BL = 9,

    RCX = 10,
    ECX = 11,
    CX = 12,
    CH = 13,
    CL = 14,

    RDX = 15,
    EDX = 16,
    DX = 17,
    DH = 18,
    DL = 19,

    RSI = 20,
    ESI = 21,
    SI = 22,
    SIL = 23,

    RDI = 24,
    EDI = 25,
    DI = 26,
    DIL = 27,

    RBP = 28,
    EBP = 29,
    BP = 30,
    BPL = 31,

    RSP = 32,
    ESP = 33,
    SP = 34,
    SPL = 35,

    R8 = 36,
    R8D = 37,
    R8W = 38,
    R8B = 39,

    R9 = 40,
    R9D = 41,
    R9W = 42,
    R9B = 43,

    R10 = 44,
    R10D = 45,
    R10W = 46,
    R10B = 47,

    R11 = 48,
    R11D = 49,
    R11W = 50,
    R11B = 51,

    R12 = 52,
    R12D = 53,
    R12W = 54,
    R12B = 55,

    R13 = 56,
    R13D = 57,
    R13W = 58,
    R13B = 59,

    R14 = 60,
    R14D = 61,
    R14W = 62,
    R14B = 63,

    R15 = 64,
    R15D = 65,
    R15W = 66,
    R15B = 67,

    ST0 = 68,
    ST1 = 69,
    ST2 = 70,
    ST3 = 71,
    ST4 = 72,
    ST5 = 73,
    ST6 = 74,
    ST7 = 75,

    MM0 = 76,
    MM1 = 77,
    MM2 = 78,
    MM3 = 79,
    MM4 = 80,
    MM5 = 81,
    MM6 = 82,
    MM7 = 83,

    ES = 84,
    CS = 85,
    SS = 86,
    DS = 87,
    FS = 88,
    GS = 89,

    XMM0 = 90,
    XMM1 = 91,
    XMM2 = 92,
    XMM3 = 93,
    XMM4 = 94,
    XMM5 = 95,
    XMM6 = 96,
    XMM7 = 97,
    XMM8 = 98,
    XMM9 = 99,
    XMM10 = 100,
    XMM11 = 101,
    XMM12 = 102,
    XMM13 = 103,
    XMM14 = 104,
    XMM15 = 105,

    YMM0 = 106,
    YMM1 = 107,
    YMM2 = 108,
    YMM3 = 109,
    YMM4 = 110,
    YMM5 = 111,
    YMM6 = 112,
    YMM7 = 113,
    YMM8 = 114,
    YMM9 = 115,
    YMM10 = 116,
    YMM11 = 117,
    YMM12 = 118,
    YMM13 = 119,
    YMM14 = 120,
    YMM15 = 121,

    RIP = 122,
    EIP = 123,
    IP = 124,

    CR0 = 125,
    CR1 = 126,
    CR2 = 127,
    CR3 = 128,
    CR4 = 129,
    CR5 = 130,
    CR6 = 131,
    CR7 = 132,
    CR8 = 133,
    CR9 = 134,
    CR10 = 135,
    CR11 = 136,
    CR12 = 137,
    CR13 = 138,
    CR14 = 139,
    CR15 = 140,

    DR0 = 141,
    DR1 = 142,
    DR2 = 143,
    DR3 = 144,
    DR4 = 145,
    DR5 = 146,
    DR6 = 147,
    DR7 = 148,
    DR8 = 149,
    DR9 = 150,
    DR10 = 151,
    DR11 = 152,
    DR12 = 153,
    DR13 = 154,
    DR14 = 155,
    DR15 = 156,

    Count = 157,
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
    pub fn is_extended(&self) -> bool {
        (Self::R8 as usize..=Self::R15B as usize).contains(&(*self as usize))
    }

    #[inline]
    pub fn is_vector(&self) -> bool {
        (Self::XMM0 as usize..Self::YMM15 as usize).contains(&(*self as usize))
    }

    #[inline]
    pub fn id(&self) -> u8 {
        Self::ID_TABLE[*self as usize]
    }

    #[inline]
    pub fn mnemonic(&self) -> &'static str {
        Self::MNEMONIC_TABLE[*self as usize]
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
        0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F,
    ];

    /// Contains the mnemonic for each register.
    pub const MNEMONIC_TABLE: [&'static str; Self::Count as _] = [
        "rax", "eax", "ax", "ah", "al", "rbx", "ebx", "bx", "bh", "bl", "rcx", "ecx", "cx", "ch",
        "cl", "rdx", "edx", "dx", "dh", "dl", "rsi", "esi", "si", "sil", "rdi", "edi", "di", "dil",
        "rbp", "ebp", "bp", "bpl", "rsp", "esp", "sp", "spl", "r8", "r8d", "r8w", "r8b", "r9",
        "r9d", "r9w", "r9b", "r10", "r10d", "r10w", "r10b", "r11", "r11d", "r11w", "r11b", "r12",
        "r12d", "r12w", "r12b", "r13", "r13d", "r13w", "r13b", "r14", "r14d", "r14w", "r14b",
        "r15", "r15d", "r15w", "r15b", "st0", "st1", "st2", "st3", "st4", "st5", "st6", "st7",
        "mm0", "mm1", "mm2", "mm3", "mm4", "mm5", "mm6", "mm7", "es", "cs", "ss", "ds", "fs", "gs",
        "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7", "xmm8", "xmm9", "xmm10",
        "xmm11", "xmm12", "xmm13", "xmm14", "xmm15", "ymm0", "ymm1", "ymm2", "ymm3", "ymm4",
        "ymm5", "ymm6", "ymm7", "ymm8", "ymm9", "ymm10", "ymm11", "ymm12", "ymm13", "ymm14",
        "ymm15", "rip", "eip", "ip", "cr0", "cr1", "cr2", "cr3", "cr4", "cr5", "cr6", "cr7", "cr8",
        "cr9", "cr10", "cr11", "cr12", "cr13", "cr14", "cr15", "dr0", "dr1", "dr2", "dr3", "dr4",
        "dr5", "dr6", "dr7", "dr8", "dr9", "dr10", "dr11", "dr12", "dr13", "dr14", "dr15",
    ];

    /// Contains the size of each register in bytes.
    pub const SIZE_TABLE: [u8; Register::Count as _] = [
        8, 4, 2, 1, 1, 8, 4, 2, 1, 1, 8, 4, 2, 1, 1, 8, 4, 2, 1, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4,
        2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1, 8, 4, 2, 1,
        8, 4, 2, 1, 8, 4, 2, 1, 10, 10, 10, 10, 10, 10, 10, 10, 8, 8, 8, 8, 8, 8, 8, 8, 2, 2, 2, 2,
        2, 2, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 8, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
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
