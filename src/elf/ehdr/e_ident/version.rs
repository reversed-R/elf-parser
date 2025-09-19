use crate::elf::ehdr::EhdrParseError;

const EV_NONE: u8 = 0; /* Invalid ELF version */
const EV_CURRENT: u8 = 1; /* Current version */
const EV_NUM: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EIVersion {
    None,
    Current,
    Num,
}

impl TryFrom<u8> for EIVersion {
    type Error = EhdrParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            EV_NONE => Ok(Self::None),
            EV_CURRENT => Ok(Self::Current),
            EV_NUM => Ok(Self::Num),
            _ => Err(EhdrParseError::InvalidEIdentVersion),
        }
    }
}
