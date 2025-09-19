pub mod elf;

#[cfg(test)]
mod tests {
    use crate::elf::{
        Elf32,
        ehdr::{
            Ehdr32,
            e_ident::{
                EIdent, abiversion::ABIVersion, class::EIClass, data::EIData, osabi::EIOsABI,
                version::EIVersion,
            },
            e_machine::EMachine,
            e_type::EType,
        },
    };

    #[test]
    fn read_file() {
        let file_path = "assets/a.out";
        let contents = std::fs::read(file_path).expect("file read error");

        let res = Elf32::try_from(&contents[..]);

        assert_eq!(
            res,
            Ok(Elf32 {
                ehdr: Ehdr32 {
                    e_ident: EIdent {
                        class: EIClass::Class64,
                        data: EIData::Lsb,
                        version: EIVersion::Current,
                        osabi: EIOsABI::None,
                        abiversion: ABIVersion(0)
                    },
                    e_type: EType::Dyn,
                    e_machine: EMachine::X86_64
                }
            })
        );
    }
}
