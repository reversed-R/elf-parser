use crate::elf::Elf32Addr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EEntry(pub Elf32Addr);
