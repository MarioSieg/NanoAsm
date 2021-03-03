#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operand {
    Register(usize),
    Imm8(i8),
    Imm16(i16),
    Imm32(i32),
    Imm64(i64),
}
