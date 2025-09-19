use crate::elf::ehdr::{
    e_ident::{EI_NIDENT, EIdent},
    e_machine::EMachine,
    e_type::EType,
};

pub(crate) mod e_ident;
pub(crate) mod e_machine;
pub(crate) mod e_type;

pub(crate) const EHDR32_SIZE: usize = EI_NIDENT + 4;
// pub(crate) const EHDR64_SIZE: usize = EI_NIDENT;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ehdr32 {
    pub e_ident: EIdent,
    pub e_type: EType,
    pub e_machine: EMachine,
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
            let e_type = EType::try_from(u16::from_ne_bytes([e_type_hi, e_type_lo]))?;

            let e_machine_hi = value[EI_NIDENT + 2];
            let e_machine_lo = value[EI_NIDENT + 3];
            let e_machine = EMachine::try_from(u16::from_ne_bytes([e_machine_hi, e_machine_lo]))?;

            Ok(Self {
                e_ident,
                e_type,
                e_machine,
            })
        }
    }
}
