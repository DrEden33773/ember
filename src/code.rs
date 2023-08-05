#![doc = "
# Code
A definition of bytecode format.
"]

#[doc = "
## Byte
A single byte.
- type: u8
"]
pub type Byte = u8;

#[doc = "
## Instructions
A vector of bytecode instructions.
- type: Vec<u8>
"]
pub type Instructions = Vec<u8>;

#[doc = "
## Opcode
An enum of bytecode instructions.
- repr: u8
"]
#[repr(u8)]
#[derive(Debug, Clone, Copy, enum_repr::EnumU8)]
pub enum Opcode {
  OpConstant,
}

#[doc = "
## Definition
A definition of bytecode format.
- name: &str
- operand_widths: Vec<usize>
"]
pub struct Definition {
  pub name: String,
  pub operand_widths: Vec<usize>,
}
