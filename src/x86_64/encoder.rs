use super::ast::Operand;
use super::machine::instructions::Instruction;
use smallvec::SmallVec;

#[derive(Eq, PartialEq, Debug)]
pub enum EncoderError {}

pub fn encode(
    _instr: Instruction,
    _operands: &[Operand],
) -> Result<SmallVec<[u8; 16]>, &'static str> {
    Ok(SmallVec::new())
}
