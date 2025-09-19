pub(crate) mod abiversion;
pub(crate) mod class;
pub(crate) mod data;
pub(crate) mod osabi;
pub(crate) mod version;

use crate::elf::ehdr::{
    EhdrParseError,
    e_ident::{
        abiversion::ABIVersion, class::EIClass, data::EIData, osabi::EIOsABI, version::EIVersion,
    },
};

const EI_MAG0: usize = 0; /* File identification byte 0 index */
const ELFMAG0: u8 = 0x7f; /* Magic number byte 0 */

const EI_MAG1: usize = 1; /* File identification byte 1 index */
const ELFMAG1: u8 = 0x45; /* Magic number byte 1 'E' */

const EI_MAG2: usize = 2; /* File identification byte 2 index */
const ELFMAG2: u8 = 0x4c; /* Magic number byte 2 'L' */

const EI_MAG3: usize = 3; /* File identification byte 3 index */
const ELFMAG3: u8 = 0x46; /* Magic number byte 3 'F' */

const EI_CLASS: usize = 4; /* File class byte index */
const EI_DATA: usize = 5; /* Data encoding byte index */
const EI_VERSION: usize = 6; /* File version byte index */
const EI_OSABI: usize = 7; /* OS ABI identification */

const EI_ABIVERSION: usize = 8; /* ABI version */
// const EI_PAD: usize = 9; /* Byte index of padding bytes */
pub(crate) const EI_NIDENT: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EIdent {
    pub class: EIClass,
    pub data: EIData,
    pub version: EIVersion,
    pub osabi: EIOsABI,
    pub abiversion: ABIVersion,
}

impl TryFrom<&[u8]> for EIdent {
    type Error = EhdrParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != EI_NIDENT {
            Err(EhdrParseError::InvalidEhdrSize(value.len()))
        } else if value[EI_MAG0] != ELFMAG0
            || value[EI_MAG1] != ELFMAG1
            || value[EI_MAG2] != ELFMAG2
            || value[EI_MAG3] != ELFMAG3
        {
            Err(EhdrParseError::InvalidElfMagic)
        } else {
            let class = EIClass::try_from(value[EI_CLASS])?;
            let data = EIData::try_from(value[EI_DATA])?;
            let version = EIVersion::try_from(value[EI_VERSION])?;
            let osabi = EIOsABI::try_from(value[EI_OSABI])?;
            let abiversion = ABIVersion(value[EI_ABIVERSION]);

            Ok(Self {
                class,
                data,
                version,
                osabi,
                abiversion,
            })
        }
    }
}
