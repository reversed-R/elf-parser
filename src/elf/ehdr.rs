pub(crate) mod e_ident;
pub(crate) mod e_machine;
pub(crate) mod e_type;
pub(crate) mod e_version;

use crate::elf::{
    Elf32Addr, Elf32Off, Elf64Addr, Elf64Off,
    ehdr::{
        e_ident::{EI_NIDENT, EIdent, class::EIClass, data::EIData},
        e_machine::EMachine,
        e_type::EType,
        e_version::EVersion,
    },
    u16_from_classed_bytes, u32_from_classed_bytes, u64_from_classed_bytes,
};

pub(crate) const EHDR32_SIZE: usize = EI_NIDENT + 20;
pub(crate) const EHDR64_SIZE: usize = EI_NIDENT + 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Ehdr {
    Ehdr32(Ehdr32),
    Ehdr64(Ehdr64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ehdr32 {
    pub e_ident: EIdent,
    pub e_type: EType,
    pub e_machine: EMachine,
    pub e_version: EVersion,
    pub e_entry: Elf32Addr,
    pub e_phoff: Elf32Off,
    pub e_shoff: Elf32Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ehdr64 {
    pub e_ident: EIdent,
    pub e_type: EType,
    pub e_machine: EMachine,
    pub e_version: EVersion,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
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
    BitClassNotSupported(EIClass),
}

impl TryFrom<&[u8]> for Ehdr {
    type Error = EhdrParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < EI_NIDENT {
            Err(EhdrParseError::InvalidEhdrSize(value.len()))
        } else {
            let e_ident = EIdent::try_from(&value[..EI_NIDENT])?;

            match &e_ident.class {
                EIClass::Class32 => {
                    if value.len() < EHDR32_SIZE {
                        Err(EhdrParseError::InvalidEhdrSize(value.len()))
                    } else {
                        let e_type = EType::try_from(
                            u16_from_classed_bytes(
                                &value[EI_NIDENT..=EI_NIDENT + 1].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        )?;

                        let e_machine = EMachine::try_from(
                            u16_from_classed_bytes(
                                &value[EI_NIDENT + 2..=EI_NIDENT + 3].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        )?;

                        let e_version = EVersion::try_from(
                            u32_from_classed_bytes(
                                &value[EI_NIDENT + 4..=EI_NIDENT + 7].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        )?;

                        let e_entry = Elf32Addr(
                            u32_from_classed_bytes(
                                &value[EI_NIDENT + 8..=EI_NIDENT + 11].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        );

                        let e_phoff = Elf32Off(
                            u32_from_classed_bytes(
                                &value[EI_NIDENT + 12..=EI_NIDENT + 15].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        );

                        let e_shoff = Elf32Off(
                            u32_from_classed_bytes(
                                &value[EI_NIDENT + 16..=EI_NIDENT + 19].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        );

                        Ok(Self::Ehdr32(Ehdr32 {
                            e_ident,
                            e_type,
                            e_machine,
                            e_version,
                            e_entry,
                            e_phoff,
                            e_shoff,
                        }))
                    }
                }
                EIClass::Class64 => {
                    if value.len() < EHDR64_SIZE {
                        Err(EhdrParseError::InvalidEhdrSize(value.len()))
                    } else {
                        let e_type = EType::try_from(
                            u16_from_classed_bytes(
                                &value[EI_NIDENT..=EI_NIDENT + 1].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        )?;

                        let e_machine = EMachine::try_from(
                            u16_from_classed_bytes(
                                &value[EI_NIDENT + 2..=EI_NIDENT + 3].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        )?;

                        let e_version = EVersion::try_from(
                            u32_from_classed_bytes(
                                &value[EI_NIDENT + 4..=EI_NIDENT + 7].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        )?;

                        let e_entry = Elf64Addr(
                            u64_from_classed_bytes(
                                &value[EI_NIDENT + 8..=EI_NIDENT + 15].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        );

                        let e_phoff = Elf64Off(
                            u64_from_classed_bytes(
                                &value[EI_NIDENT + 16..=EI_NIDENT + 23].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        );

                        let e_shoff = Elf64Off(
                            u64_from_classed_bytes(
                                &value[EI_NIDENT + 24..=EI_NIDENT + 31].try_into().unwrap(),
                                &e_ident.data,
                            )
                            .ok_or(EhdrParseError::EndianNotSupported(e_ident.data))?,
                        );

                        Ok(Self::Ehdr64(Ehdr64 {
                            e_ident,
                            e_type,
                            e_machine,
                            e_version,
                            e_entry,
                            e_phoff,
                            e_shoff,
                        }))
                    }
                }
                _ => Err(EhdrParseError::BitClassNotSupported(e_ident.class)),
            }
        }
    }
}
