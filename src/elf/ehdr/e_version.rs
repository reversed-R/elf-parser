use crate::elf::ehdr::EhdrParseError;

const EV_NONE: u32 = 0; /* Invalid ELF version */
const EV_CURRENT: u32 = 1; /* Current version */
const EV_NUM: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVersion {
    None,
    Current,
    Num,
}

// impl TryFrom<u16> for EVersion {
//     type Error = EhdrParseError;
//
//     fn try_from(value: u16) -> Result<Self, Self::Error> {
//         match value {
//             EV_NONE => Ok(Self::None),
//             EV_CURRENT => Ok(Self::Current),
//             EV_NUM => Ok(Self::Num),
//             _ => Err(EhdrParseError::InvalidEIdentVersion),
//         }
//     }
// }

impl TryFrom<u32> for EVersion {
    type Error = EhdrParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            EV_NONE => Ok(Self::None),
            EV_CURRENT => Ok(Self::Current),
            EV_NUM => Ok(Self::Num),
            _ => Err(EhdrParseError::InvalidEIdentVersion),
        }
    }
}
