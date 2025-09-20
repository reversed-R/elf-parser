use crate::elf::Elf32Off;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EShoff(pub Elf32Off);
