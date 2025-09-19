use crate::elf::ehdr::EhdrParseError;

const ELFOSABI_NONE: u8 = 0; /* UNIX System V ABI */
// const ELFOSABI_SYSV: u8 = 0; /* Alias.  */
const ELFOSABI_HPUX: u8 = 1; /* HP-UX */
const ELFOSABI_NETBSD: u8 = 2; /* NetBSD.  */
const ELFOSABI_GNU: u8 = 3; /* Object uses GNU ELF extensions.  */
// const ELFOSABI_LINUX: u8 = ELFOSABI_GNU; /* Compatibility alias.  */
const ELFOSABI_SOLARIS: u8 = 6; /* Sun Solaris.  */
const ELFOSABI_AIX: u8 = 7; /* IBM AIX.  */
const ELFOSABI_IRIX: u8 = 8; /* SGI Irix.  */
const ELFOSABI_FREEBSD: u8 = 9; /* FreeBSD.  */
const ELFOSABI_TRU64: u8 = 10; /* Compaq TRU64 UNIX.  */
const ELFOSABI_MODESTO: u8 = 11; /* Novell Modesto.  */
const ELFOSABI_OPENBSD: u8 = 12; /* OpenBSD.  */
const ELFOSABI_ARM_AEABI: u8 = 64; /* ARM EABI */
const ELFOSABI_ARM: u8 = 97; /* ARM */
const ELFOSABI_STANDALONE: u8 = 255; /* Standalone (embedded) application */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EIOsABI {
    None,
    HpUx,
    NetBSD,
    Gnu,
    Solaris,
    Aix,
    Irix,
    FreeBSD,
    TRU64,
    Modesto,
    OpenBSD,
    ArmAEABI,
    Arm,
    Standalone,
}

impl TryFrom<u8> for EIOsABI {
    type Error = EhdrParseError;

    fn try_from(value: u8) -> Result<Self, EhdrParseError> {
        match value {
            ELFOSABI_NONE => Ok(Self::None),
            ELFOSABI_HPUX => Ok(Self::HpUx),
            ELFOSABI_NETBSD => Ok(Self::NetBSD),
            ELFOSABI_GNU => Ok(Self::Gnu),
            // ELFOSABI_LINUX => Ok(Self::GNU),
            ELFOSABI_AIX => Ok(Self::Aix),
            ELFOSABI_IRIX => Ok(Self::Irix),
            ELFOSABI_SOLARIS => Ok(Self::Solaris),
            ELFOSABI_FREEBSD => Ok(Self::FreeBSD),
            ELFOSABI_TRU64 => Ok(Self::TRU64),
            ELFOSABI_MODESTO => Ok(Self::Modesto),
            ELFOSABI_OPENBSD => Ok(Self::OpenBSD),
            ELFOSABI_ARM_AEABI => Ok(Self::ArmAEABI),
            ELFOSABI_ARM => Ok(Self::Arm),
            ELFOSABI_STANDALONE => Ok(Self::Standalone),
            _ => Err(EhdrParseError::InvalidEIdentClass),
        }
    }
}
