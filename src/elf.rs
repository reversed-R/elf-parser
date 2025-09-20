pub(crate) mod ehdr;
pub(crate) mod phdr;
pub(crate) mod shdr;

use crate::elf::ehdr::{EHDR64_SIZE, Ehdr, Ehdr32, Ehdr64, EhdrParseError, e_ident::data::EIData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Elf {
    Elf32(Elf32),
    Elf64(Elf64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elf32 {
    pub ehdr: Ehdr32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elf64 {
    pub ehdr: Ehdr64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfParseError {
    EhdrParseError(EhdrParseError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf64Addr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf64Off(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf32Addr(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf32Off(pub u32);

impl TryFrom<&[u8]> for Elf {
    type Error = ElfParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < EHDR64_SIZE {
            Err(ElfParseError::EhdrParseError(
                EhdrParseError::InvalidEhdrSize(0),
            ))
        } else {
            match Ehdr::try_from(&value[..EHDR64_SIZE]).map_err(ElfParseError::EhdrParseError)? {
                Ehdr::Ehdr32(ehdr) => Ok(Self::Elf32(Elf32 { ehdr })),
                Ehdr::Ehdr64(ehdr) => Ok(Self::Elf64(Elf64 { ehdr })),
            }
        }
    }
}

pub fn u16_from_classed_bytes(bytes: &[u8; 2], data: &EIData) -> Option<u16> {
    match data {
        EIData::Lsb => Some(u16::from_le_bytes(bytes.to_owned())),
        EIData::Msb => Some(u16::from_be_bytes(bytes.to_owned())),
        _ => None,
    }
}

pub fn u32_from_classed_bytes(bytes: &[u8; 4], data: &EIData) -> Option<u32> {
    match data {
        EIData::Lsb => Some(u32::from_le_bytes(bytes.to_owned())),
        EIData::Msb => Some(u32::from_be_bytes(bytes.to_owned())),
        _ => None,
    }
}

pub fn u64_from_classed_bytes(bytes: &[u8; 8], data: &EIData) -> Option<u64> {
    match data {
        EIData::Lsb => Some(u64::from_le_bytes(bytes.to_owned())),
        EIData::Msb => Some(u64::from_be_bytes(bytes.to_owned())),
        _ => None,
    }
}
