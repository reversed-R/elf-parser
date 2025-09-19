pub(crate) mod ehdr;
pub(crate) mod phdr;
pub(crate) mod shdr;

use crate::elf::ehdr::{EHDR32_SIZE, Ehdr32, EhdrParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elf32 {
    pub ehdr: Ehdr32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfParseError {
    EhdrParseError(EhdrParseError),
}

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
