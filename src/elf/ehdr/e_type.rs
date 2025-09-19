use crate::elf::ehdr::EhdrParseError;

/* Legal values for e_type (object file type).  */
pub const ET_NONE: u16 = 0; /* No file type */
pub const ET_REL: u16 = 1; /* Relocatable file */
pub const ET_EXEC: u16 = 2; /* Executable file */
pub const ET_DYN: u16 = 3; /* Shared object file */
pub const ET_CORE: u16 = 4; /* Core file */
pub const ET_NUM: u16 = 5; /* Number of defined types */
pub const ET_LOOS: u16 = 0xfe00; /* OS-specific range start */
pub const ET_HIOS: u16 = 0xfeff; /* OS-specific range end */
pub const ET_LOPROC: u16 = 0xff00; /* Processor-specific range start */

// pub const ET_HIPROC: u16 = 0xffff; /* Processor-specific range end */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Num,
    Os(u16),
    Proc(u16),
}

impl TryFrom<u16> for EType {
    type Error = EhdrParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            ET_NONE => Ok(Self::None),
            ET_NUM => Ok(Self::Num),
            ET_REL => Ok(Self::Rel),
            ET_EXEC => Ok(Self::Exec),
            ET_DYN => Ok(Self::Dyn),
            ET_CORE => Ok(Self::Core),
            _ => {
                if (ET_LOOS..ET_HIOS + 1).contains(&value) {
                    Ok(Self::Os(value))
                } else if ET_LOPROC <= value {
                    Ok(Self::Proc(value))
                } else {
                    Err(EhdrParseError::InvalidEType(value))
                }
            }
        }
    }
}
