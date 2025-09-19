use crate::elf::ehdr::EhdrParseError;

const ELFDATANONE: u8 = 0; /* Invalid data encoding */
const ELFDATA2LSB: u8 = 1; /* 2's complement, little endian */
const ELFDATA2MSB: u8 = 2; /* 2's complement, big endian */
const ELFDATANUM: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EIData {
    None,
    Lsb,
    Msb,
    Num,
}

impl TryFrom<u8> for EIData {
    type Error = EhdrParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ELFDATANONE => Ok(Self::None),
            ELFDATA2LSB => Ok(Self::Lsb),
            ELFDATA2MSB => Ok(Self::Msb),
            ELFDATANUM => Ok(Self::Num),
            _ => Err(EhdrParseError::InvalidEIdentData),
        }
    }
}
