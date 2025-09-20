pub(crate) mod e_entry;
pub(crate) mod e_ident;
pub(crate) mod e_machine;
pub(crate) mod e_phoff;
pub(crate) mod e_shoff;
pub(crate) mod e_type;
pub(crate) mod e_version;

use crate::elf::{
    Elf32Addr, Elf32Off,
    ehdr::{
        e_entry::EEntry,
        e_ident::{EI_NIDENT, EIdent, data::EIData},
        e_machine::EMachine,
        e_phoff::EPhoff,
        e_shoff::EShoff,
        e_type::EType,
        e_version::EVersion,
    },
    u16_from_classed_bytes, u32_from_classed_bytes,
};

pub(crate) const EHDR32_SIZE: usize = EI_NIDENT + 20;
// pub(crate) const EHDR64_SIZE: usize = EI_NIDENT;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ehdr32 {
    pub e_ident: EIdent,
    pub e_type: EType,
    pub e_machine: EMachine,
    pub e_version: EVersion,
    pub e_entry: EEntry,
    pub e_phoff: EPhoff,
    pub e_shoff: EShoff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EhdrParseError {
    InvalidEhdrSize(usize),
    InvalidElfMagic,
    InvalidEIdentClass,
    InvalidEIdentData,
    InvalidEIdentVersion,
    InvalidEType(u16),
    InvalidEMachine(u16),
    EndianNotSupported(EIData),
}

impl TryFrom<&[u8]> for Ehdr32 {
    type Error = EhdrParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != EHDR32_SIZE {
            Err(EhdrParseError::InvalidEhdrSize(value.len()))
        } else {
            let e_ident = EIdent::try_from(&value[..EI_NIDENT])?;

            let e_type_hi = value[EI_NIDENT];
            let e_type_lo = value[EI_NIDENT + 1];
            let e_type = EType::try_from(
                u16_from_classed_bytes([e_type_hi, e_type_lo], &e_ident.data)
                    .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
            )?;

            let e_machine_hi = value[EI_NIDENT + 2];
            let e_machine_lo = value[EI_NIDENT + 3];
            let e_machine = EMachine::try_from(
                u16_from_classed_bytes([e_machine_hi, e_machine_lo], &e_ident.data)
                    .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
            )?;

            let e_version_0 = value[EI_NIDENT + 4];
            let e_version_1 = value[EI_NIDENT + 5];
            let e_version_2 = value[EI_NIDENT + 6];
            let e_version_3 = value[EI_NIDENT + 7];
            let e_version = EVersion::try_from(
                u32_from_classed_bytes(
                    [e_version_0, e_version_1, e_version_2, e_version_3],
                    &e_ident.data,
                )
                .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
            )?;

            let e_entry_0 = value[EI_NIDENT + 8];
            let e_entry_1 = value[EI_NIDENT + 9];
            let e_entry_2 = value[EI_NIDENT + 10];
            let e_entry_3 = value[EI_NIDENT + 11];
            let e_entry = EEntry(Elf32Addr(
                u32_from_classed_bytes([e_entry_0, e_entry_1, e_entry_2, e_entry_3], &e_ident.data)
                    .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
            ));

            let e_phoff_0 = value[EI_NIDENT + 12];
            let e_phoff_1 = value[EI_NIDENT + 13];
            let e_phoff_2 = value[EI_NIDENT + 14];
            let e_phoff_3 = value[EI_NIDENT + 15];
            let e_phoff = EPhoff(Elf32Off(
                u32_from_classed_bytes([e_phoff_0, e_phoff_1, e_phoff_2, e_phoff_3], &e_ident.data)
                    .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
            ));

            let e_shoff_0 = value[EI_NIDENT + 16];
            let e_shoff_1 = value[EI_NIDENT + 17];
            let e_shoff_2 = value[EI_NIDENT + 18];
            let e_shoff_3 = value[EI_NIDENT + 19];
            let e_shoff = EShoff(Elf32Off(
                u32_from_classed_bytes([e_shoff_0, e_shoff_1, e_shoff_2, e_shoff_3], &e_ident.data)
                    .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
            ));

            Ok(Self {
                e_ident,
                e_type,
                e_machine,
                e_version,
                e_entry,
                e_phoff,
                e_shoff,
            })
        }
    }
}
