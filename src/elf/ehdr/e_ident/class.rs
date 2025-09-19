use crate::elf::ehdr::EhdrParseError;

pub const ELFCLASSNONE: u8 = 0; /* Invalid class */
pub const ELFCLASS32: u8 = 1; /* 32-bit objects */
pub const ELFCLASS64: u8 = 2; /* 64-bit objects */
pub const ELFCLASSNUM: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EIClass {
    None,
    Class32,
    Class64,
    Num,
}

impl TryFrom<u8> for EIClass {
    type Error = EhdrParseError;

    fn try_from(value: u8) -> Result<Self, EhdrParseError> {
        match value {
            ELFCLASSNONE => Ok(Self::None),
            ELFCLASS32 => Ok(Self::Class32),
            ELFCLASS64 => Ok(Self::Class64),
            ELFCLASSNUM => Ok(Self::Num),
            _ => Err(EhdrParseError::InvalidEIdentClass),
        }
    }
}
