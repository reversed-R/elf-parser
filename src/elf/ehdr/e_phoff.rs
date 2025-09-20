use crate::elf::Elf32Off;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EPhoff(pub Elf32Off);
