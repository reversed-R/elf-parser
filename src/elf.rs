pub(crate) mod ehdr;
pub(crate) mod phdr;
pub(crate) mod shdr;

use crate::elf::ehdr::{EHDR32_SIZE, Ehdr32, EhdrParseError, e_ident::data::EIData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elf32 {
    pub ehdr: Ehdr32,
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

impl TryFrom<&[u8]> for Elf32 {
    type Error = ElfParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < EHDR32_SIZE {
            Err(ElfParseError::EhdrParseError(
                EhdrParseError::InvalidEhdrSize(0),
            ))
        } else {
            let ehdr =
                Ehdr32::try_from(&value[..EHDR32_SIZE]).map_err(ElfParseError::EhdrParseError)?;

            Ok(Self { ehdr })
        }
    }
}

pub fn u16_from_classed_bytes(bytes: [u8; 2], data: &EIData) -> Option<u16> {
    match data {
        EIData::Lsb => Some(u16::from_le_bytes(bytes)),
        EIData::Msb => Some(u16::from_be_bytes(bytes)),
        _ => None,
    }
}

pub fn u32_from_classed_bytes(bytes: [u8; 4], data: &EIData) -> Option<u32> {
    match data {
        EIData::Lsb => Some(u32::from_le_bytes(bytes)),
        EIData::Msb => Some(u32::from_be_bytes(bytes)),
        _ => None,
    }
}

pub fn u64_from_classed_bytes(bytes: [u8; 8], data: &EIData) -> Option<u64> {
    match data {
        EIData::Lsb => Some(u64::from_le_bytes(bytes)),
        EIData::Msb => Some(u64::from_be_bytes(bytes)),
        _ => None,
    }
}
