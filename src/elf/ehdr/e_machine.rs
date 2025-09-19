use crate::elf::ehdr::EhdrParseError;

/* Legal values for e_machine (architecture).  */

const EM_NONE: u16 = 0; /* No machine */
const EM_M32: u16 = 1; /* AT&T WE 32100 */
const EM_SPARC: u16 = 2; /* SUN SPARC */
const EM_386: u16 = 3; /* Intel 80386 */
const EM_68K: u16 = 4; /* Motorola m68k family */
const EM_88K: u16 = 5; /* Motorola m88k family */
const EM_IAMCU: u16 = 6; /* Intel MCU */
const EM_860: u16 = 7; /* Intel 80860 */
const EM_MIPS: u16 = 8; /* MIPS R3000 big-endian */
const EM_S370: u16 = 9; /* IBM System/370 */
const EM_MIPS_RS3_LE: u16 = 10; /* MIPS R3000 little-endian */
const EM_PARISC: u16 = 15; /* HPPA */
const EM_VPP500: u16 = 17; /* Fujitsu VPP500 */
const EM_SPARC32PLUS: u16 = 18; /* Sun's "v8plus" */
const EM_960: u16 = 19; /* Intel 80960 */
const EM_PPC: u16 = 20; /* PowerPC */
const EM_PPC64: u16 = 21; /* PowerPC 64-bit */
const EM_S390: u16 = 22; /* IBM S390 */
const EM_SPU: u16 = 23; /* IBM SPU/SPC */
const EM_V800: u16 = 36; /* NEC V800 series */
const EM_FR20: u16 = 37; /* Fujitsu FR20 */
const EM_RH32: u16 = 38; /* TRW RH-32 */
const EM_RCE: u16 = 39; /* Motorola RCE */
const EM_ARM: u16 = 40; /* ARM */
const EM_FAKE_ALPHA: u16 = 41; /* Digital Alpha */
const EM_SH: u16 = 42; /* Hitachi SH */
const EM_SPARCV9: u16 = 43; /* SPARC v9 64-bit */
const EM_TRICORE: u16 = 44; /* Siemens Tricore */
const EM_ARC: u16 = 45; /* Argonaut RISC Core */
const EM_H8_300: u16 = 46; /* Hitachi H8/300 */
const EM_H8_300H: u16 = 47; /* Hitachi H8/300H */
const EM_H8S: u16 = 48; /* Hitachi H8S */
const EM_H8_500: u16 = 49; /* Hitachi H8/500 */
const EM_IA_64: u16 = 50; /* Intel Merced */
const EM_MIPS_X: u16 = 51; /* Stanford MIPS-X */
const EM_COLDFIRE: u16 = 52; /* Motorola Coldfire */
const EM_68HC12: u16 = 53; /* Motorola M68HC12 */
const EM_MMA: u16 = 54; /* Fujitsu MMA Multimedia Accelerator */
const EM_PCP: u16 = 55; /* Siemens PCP */
const EM_NCPU: u16 = 56; /* Sony nCPU embedded RISC */
const EM_NDR1: u16 = 57; /* Denso NDR1 microprocessor */
const EM_STARCORE: u16 = 58; /* Motorola Start*Core processor */
const EM_ME16: u16 = 59; /* Toyota ME16 processor */
const EM_ST100: u16 = 60; /* STMicroelectronic ST100 processor */
const EM_TINYJ: u16 = 61; /* Advanced Logic Corp. Tinyj emb.fam */
const EM_X86_64: u16 = 62; /* AMD x86-64 architecture */
const EM_PDSP: u16 = 63; /* Sony DSP Processor */
const EM_PDP10: u16 = 64; /* Digital PDP-10 */
const EM_PDP11: u16 = 65; /* Digital PDP-11 */
const EM_FX66: u16 = 66; /* Siemens FX66 microcontroller */
const EM_ST9PLUS: u16 = 67; /* STMicroelectronics ST9+ 8/16 mc */
const EM_ST7: u16 = 68; /* STmicroelectronics ST7 8 bit mc */
const EM_68HC16: u16 = 69; /* Motorola MC68HC16 microcontroller */
const EM_68HC11: u16 = 70; /* Motorola MC68HC11 microcontroller */
const EM_68HC08: u16 = 71; /* Motorola MC68HC08 microcontroller */
const EM_68HC05: u16 = 72; /* Motorola MC68HC05 microcontroller */
const EM_SVX: u16 = 73; /* Silicon Graphics SVx */
const EM_ST19: u16 = 74; /* STMicroelectronics ST19 8 bit mc */
const EM_VAX: u16 = 75; /* Digital VAX */
const EM_CRIS: u16 = 76; /* Axis Communications 32-bit emb.proc */
const EM_JAVELIN: u16 = 77; /* Infineon Technologies 32-bit emb.proc */
const EM_FIREPATH: u16 = 78; /* Element 14 64-bit DSP Processor */
const EM_ZSP: u16 = 79; /* LSI Logic 16-bit DSP Processor */
const EM_MMIX: u16 = 80; /* Donald Knuth's educational 64-bit proc */
const EM_HUANY: u16 = 81; /* Harvard University machine-independent object files */
const EM_PRISM: u16 = 82; /* SiTera Prism */
const EM_AVR: u16 = 83; /* Atmel AVR 8-bit microcontroller */
const EM_FR30: u16 = 84; /* Fujitsu FR30 */
const EM_D10V: u16 = 85; /* Mitsubishi D10V */
const EM_D30V: u16 = 86; /* Mitsubishi D30V */
const EM_V850: u16 = 87; /* NEC v850 */
const EM_M32R: u16 = 88; /* Mitsubishi M32R */
const EM_MN10300: u16 = 89; /* Matsushita MN10300 */
const EM_MN10200: u16 = 90; /* Matsushita MN10200 */
const EM_PJ: u16 = 91; /* picoJava */
const EM_OPENRISC: u16 = 92; /* OpenRISC 32-bit embedded processor */
const EM_ARC_COMPACT: u16 = 93; /* ARC International ARCompact */
const EM_XTENSA: u16 = 94; /* Tensilica Xtensa Architecture */
const EM_VIDEOCORE: u16 = 95; /* Alphamosaic VideoCore */
const EM_TMM_GPP: u16 = 96; /* Thompson Multimedia General Purpose Proc */
const EM_NS32K: u16 = 97; /* National Semi. 32000 */
const EM_TPC: u16 = 98; /* Tenor Network TPC */
const EM_SNP1K: u16 = 99; /* Trebia SNP 1000 */
const EM_ST200: u16 = 100; /* STMicroelectronics ST200 */
const EM_IP2K: u16 = 101; /* Ubicom IP2xxx */
const EM_MAX: u16 = 102; /* MAX processor */
const EM_CR: u16 = 103; /* National Semi. CompactRISC */
const EM_F2MC16: u16 = 104; /* Fujitsu F2MC16 */
const EM_MSP430: u16 = 105; /* Texas Instruments msp430 */
const EM_BLACKFIN: u16 = 106; /* Analog Devices Blackfin DSP */
const EM_SE_C33: u16 = 107; /* Seiko Epson S1C33 family */
const EM_SEP: u16 = 108; /* Sharp embedded microprocessor */
const EM_ARCA: u16 = 109; /* Arca RISC */
const EM_UNICORE: u16 = 110; /* PKU-Unity & MPRC Peking Uni. mc series */
const EM_EXCESS: u16 = 111; /* eXcess configurable cpu */
const EM_DXP: u16 = 112; /* Icera Semi. Deep Execution Processor */
const EM_ALTERA_NIOS2: u16 = 113; /* Altera Nios II */
const EM_CRX: u16 = 114; /* National Semi. CompactRISC CRX */
const EM_XGATE: u16 = 115; /* Motorola XGATE */
const EM_C166: u16 = 116; /* Infineon C16x/XC16x */
const EM_M16C: u16 = 117; /* Renesas M16C */
const EM_DSPIC30F: u16 = 118; /* Microchip Technology dsPIC30F */
const EM_CE: u16 = 119; /* Freescale Communication Engine RISC */
const EM_M32C: u16 = 120; /* Renesas M32C */
const EM_TSK3000: u16 = 131; /* Altium TSK3000 */
const EM_RS08: u16 = 132; /* Freescale RS08 */
const EM_SHARC: u16 = 133; /* Analog Devices SHARC family */
const EM_ECOG2: u16 = 134; /* Cyan Technology eCOG2 */
const EM_SCORE7: u16 = 135; /* Sunplus S+core7 RISC */
const EM_DSP24: u16 = 136; /* New Japan Radio (NJR) 24-bit DSP */
const EM_VIDEOCORE3: u16 = 137; /* Broadcom VideoCore III */
const EM_LATTICEMICO32: u16 = 138; /* RISC for Lattice FPGA */
const EM_SE_C17: u16 = 139; /* Seiko Epson C17 */
const EM_TI_C6000: u16 = 140; /* Texas Instruments TMS320C6000 DSP */
const EM_TI_C2000: u16 = 141; /* Texas Instruments TMS320C2000 DSP */
const EM_TI_C5500: u16 = 142; /* Texas Instruments TMS320C55x DSP */
const EM_TI_ARP32: u16 = 143; /* Texas Instruments App. Specific RISC */
const EM_TI_PRU: u16 = 144; /* Texas Instruments Prog. Realtime Unit */
const EM_MMDSP_PLUS: u16 = 160; /* STMicroelectronics 64bit VLIW DSP */
const EM_CYPRESS_M8C: u16 = 161; /* Cypress M8C */
const EM_R32C: u16 = 162; /* Renesas R32C */
const EM_TRIMEDIA: u16 = 163; /* NXP Semi. TriMedia */
const EM_QDSP6: u16 = 164; /* QUALCOMM DSP6 */
const EM_8051: u16 = 165; /* Intel 8051 and variants */
const EM_STXP7X: u16 = 166; /* STMicroelectronics STxP7x */
const EM_NDS32: u16 = 167; /* Andes Tech. compact code emb. RISC */
const EM_ECOG1X: u16 = 168; /* Cyan Technology eCOG1X */
const EM_MAXQ30: u16 = 169; /* Dallas Semi. MAXQ30 mc */
const EM_XIMO16: u16 = 170; /* New Japan Radio (NJR) 16-bit DSP */
const EM_MANIK: u16 = 171; /* M2000 Reconfigurable RISC */
const EM_CRAYNV2: u16 = 172; /* Cray NV2 vector architecture */
const EM_RX: u16 = 173; /* Renesas RX */
const EM_METAG: u16 = 174; /* Imagination Tech. META */
const EM_MCST_ELBRUS: u16 = 175; /* MCST Elbrus */
const EM_ECOG16: u16 = 176; /* Cyan Technology eCOG16 */
const EM_CR16: u16 = 177; /* National Semi. CompactRISC CR16 */
const EM_ETPU: u16 = 178; /* Freescale Extended Time Processing Unit */
const EM_SLE9X: u16 = 179; /* Infineon Tech. SLE9X */
const EM_L10M: u16 = 180; /* Intel L10M */
const EM_K10M: u16 = 181; /* Intel K10M */
const EM_AARCH64: u16 = 183; /* ARM AARCH64 */
const EM_AVR32: u16 = 185; /* Amtel 32-bit microprocessor */
const EM_STM8: u16 = 186; /* STMicroelectronics STM8 */
const EM_TILE64: u16 = 187; /* Tilera TILE64 */
const EM_TILEPRO: u16 = 188; /* Tilera TILEPro */
const EM_MICROBLAZE: u16 = 189; /* Xilinx MicroBlaze */
const EM_CUDA: u16 = 190; /* NVIDIA CUDA */
const EM_TILEGX: u16 = 191; /* Tilera TILE-Gx */
const EM_CLOUDSHIELD: u16 = 192; /* CloudShield */
const EM_COREA_1ST: u16 = 193; /* KIPO-KAIST Core-A 1st gen. */
const EM_COREA_2ND: u16 = 194; /* KIPO-KAIST Core-A 2nd gen. */
const EM_ARCV2: u16 = 195; /* Synopsys ARCv2 ISA.  */
const EM_OPEN8: u16 = 196; /* Open8 RISC */
const EM_RL78: u16 = 197; /* Renesas RL78 */
const EM_VIDEOCORE5: u16 = 198; /* Broadcom VideoCore V */
const EM_78KOR: u16 = 199; /* Renesas 78KOR */
const EM_56800EX: u16 = 200; /* Freescale 56800EX DSC */
const EM_BA1: u16 = 201; /* Beyond BA1 */
const EM_BA2: u16 = 202; /* Beyond BA2 */
const EM_XCORE: u16 = 203; /* XMOS xCORE */
const EM_MCHP_PIC: u16 = 204; /* Microchip 8-bit PIC(r) */
const EM_INTELGT: u16 = 205; /* Intel Graphics Technology */
const EM_KM32: u16 = 210; /* KM211 KM32 */
const EM_KMX32: u16 = 211; /* KM211 KMX32 */
const EM_EMX16: u16 = 212; /* KM211 KMX16 */
const EM_EMX8: u16 = 213; /* KM211 KMX8 */
const EM_KVARC: u16 = 214; /* KM211 KVARC */
const EM_CDP: u16 = 215; /* Paneve CDP */
const EM_COGE: u16 = 216; /* Cognitive Smart Memory Processor */
const EM_COOL: u16 = 217; /* Bluechip CoolEngine */
const EM_NORC: u16 = 218; /* Nanoradio Optimized RISC */
const EM_CSR_KALIMBA: u16 = 219; /* CSR Kalimba */
const EM_Z80: u16 = 220; /* Zilog Z80 */
const EM_VISIUM: u16 = 221; /* Controls and Data Services VISIUMcore */
const EM_FT32: u16 = 222; /* FTDI Chip FT32 */
const EM_MOXIE: u16 = 223; /* Moxie processor */
const EM_AMDGPU: u16 = 224; /* AMD GPU */
const EM_RISCV: u16 = 243; /* RISC-V */
const EM_BPF: u16 = 247; /* Linux BPF -- in-kernel virtual machine */
const EM_CSKY: u16 = 252; /* C-SKY */
const EM_LOONGARCH: u16 = 258; /* LoongArch */
const EM_NUM: u16 = 259;

/* If it is necessary to assign new unofficial EM_* values, please
pick large random numbers (0x8523, 0xa7f2, etc.) to minimize the
chances of collision with official or non-GNU unofficial values.  */

// const EM_ALPHA: u16 = 0x9026;

// pub const ET_HIPROC: u16 = 0xffff; /* Processor-specific range end */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EMachine {
    None,
    M32,
    SPARC,
    EM386,
    EM68K,
    EM88K,
    IAMCU,
    EM860,
    MIPS,
    S370,
    MipsRS3LE,
    PARISC,
    VPP500,
    SPARC32PLUS,
    EM960,
    PPC,
    PPC64,
    S390,
    SPU,
    V800,
    FR20,
    RH32,
    RCE,
    ARM,
    FakeAlpha,
    SH,
    SPARCV9,
    TRICORE,
    ARC,
    H8_300,
    H8_300H,
    H8S,
    H8_500,
    IA64,
    MipsX,
    COLDFIRE,
    EM68HC12,
    MMA,
    PCP,
    NCPU,
    NDR1,
    STARCORE,
    ME16,
    ST100,
    TINYJ,
    X86_64,
    PDSP,
    PDP10,
    PDP11,
    FX66,
    ST9PLUS,
    ST7,
    EM8HC16,
    EM8HC11,
    EM8HC08,
    EM8HC05,
    SVX,
    ST19,
    VAX,
    CRIS,
    JAVELIN,
    FIREPATH,
    ZSP,
    MMIX,
    HUANY,
    PRISM,
    AVR,
    FR30,
    D10V,
    D30V,
    V850,
    M32R,
    MN10300,
    MN10200,
    PJ,
    OPENRISC,
    ArcCompact,
    XTENSA,
    VIDEOCORE,
    TmmGpp,
    NS32K,
    TPC,
    SNP1K,
    ST200,
    IP2K,
    MAX,
    CR,
    F2MC16,
    MSP430,
    BLACKFIN,
    SeC33,
    SEP,
    ARCA,
    UNICORE,
    EXCESS,
    DXP,
    AlteraNios2,
    CRX,
    XGATE,
    C166,
    M16C,
    DSPIC30F,
    CE,
    M32C,
    TSK3000,
    RS08,
    SHARC,
    ECOG2,
    SCORE7,
    DSP24,
    VIDEOCORE3,
    LATTICEMICO32,
    SeC17,
    TiC6000,
    TiC2000,
    TiC5500,
    TiARP32,
    TiPRU,
    MmdspPlus,
    CypressM8C,
    R32C,
    TRIMEDIA,
    QDSP6,
    EM8051,
    STXP7X,
    NDS32,
    ECOG1X,
    MAXQ30,
    XIMO16,
    MANIK,
    CRAYNV2,
    RX,
    METAG,
    McstElbrus,
    ECOG16,
    CR16,
    ETPU,
    SLE9X,
    L10M,
    K10M,
    AARCH64,
    AVR32,
    STM8,
    TILE64,
    TILEPRO,
    MICROBLAZE,
    CUDA,
    TILEGX,
    CLOUDSHIELD,
    Corea1ST,
    Corea2ND,
    ARCV2,
    OPEN8,
    RL78,
    VIDEOCORE5,
    EM78KOR,
    EM56800EX,
    BA1,
    BA2,
    XCORE,
    MchpPic,
    INTELGT,
    KM32,
    KMX32,
    EMX16,
    EMX8,
    KVARC,
    CDP,
    COGE,
    COOL,
    NORC,
    CsrKalimba,
    Z80,
    VISIUM,
    FT32,
    MOXIE,
    AMDGPU,
    RISCV,
    BPF,
    CSKY,
    LOONGARCH,
    NUM,
}

impl TryFrom<u16> for EMachine {
    type Error = EhdrParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            EM_NONE => Ok(Self::None),
            EM_M32 => Ok(Self::M32),
            EM_SPARC => Ok(Self::SPARC),
            EM_386 => Ok(Self::EM386),
            EM_68K => Ok(Self::EM68K),
            EM_88K => Ok(Self::EM88K),
            EM_IAMCU => Ok(Self::IAMCU),
            EM_860 => Ok(Self::EM860),
            EM_MIPS => Ok(Self::MIPS),
            EM_S370 => Ok(Self::S370),
            EM_MIPS_RS3_LE => Ok(Self::MipsRS3LE),
            EM_PARISC => Ok(Self::PARISC),
            EM_VPP500 => Ok(Self::VPP500),
            EM_SPARC32PLUS => Ok(Self::SPARC32PLUS),
            EM_960 => Ok(Self::EM960),
            EM_PPC => Ok(Self::PPC),
            EM_PPC64 => Ok(Self::PPC64),
            EM_S390 => Ok(Self::S390),
            EM_SPU => Ok(Self::SPU),
            EM_V800 => Ok(Self::V800),
            EM_FR20 => Ok(Self::FR20),
            EM_RH32 => Ok(Self::RH32),
            EM_RCE => Ok(Self::RCE),
            EM_ARM => Ok(Self::ARM),
            EM_FAKE_ALPHA => Ok(Self::FakeAlpha),
            EM_SH => Ok(Self::SH),
            EM_SPARCV9 => Ok(Self::SPARCV9),
            EM_TRICORE => Ok(Self::TRICORE),
            EM_ARC => Ok(Self::ARC),
            EM_H8_300 => Ok(Self::H8_300),
            EM_H8_300H => Ok(Self::H8_300H),
            EM_H8S => Ok(Self::H8S),
            EM_H8_500 => Ok(Self::H8_500),
            EM_IA_64 => Ok(Self::IA64),
            EM_MIPS_X => Ok(Self::MipsX),
            EM_COLDFIRE => Ok(Self::COLDFIRE),
            EM_68HC12 => Ok(Self::EM68HC12),
            EM_MMA => Ok(Self::MMA),
            EM_PCP => Ok(Self::PCP),
            EM_NCPU => Ok(Self::NCPU),
            EM_NDR1 => Ok(Self::NDR1),
            EM_STARCORE => Ok(Self::STARCORE),
            EM_ME16 => Ok(Self::ME16),
            EM_ST100 => Ok(Self::ST100),
            EM_TINYJ => Ok(Self::TINYJ),
            EM_X86_64 => Ok(Self::X86_64),
            EM_PDSP => Ok(Self::PDSP),
            EM_PDP10 => Ok(Self::PDP10),
            EM_PDP11 => Ok(Self::PDP11),
            EM_FX66 => Ok(Self::FX66),
            EM_ST9PLUS => Ok(Self::ST9PLUS),
            EM_ST7 => Ok(Self::ST7),
            EM_68HC16 => Ok(Self::EM8HC16),
            EM_68HC11 => Ok(Self::EM8HC11),
            EM_68HC08 => Ok(Self::EM8HC08),
            EM_68HC05 => Ok(Self::EM8HC05),
            EM_SVX => Ok(Self::SVX),
            EM_ST19 => Ok(Self::ST19),
            EM_VAX => Ok(Self::VAX),
            EM_CRIS => Ok(Self::CRIS),
            EM_JAVELIN => Ok(Self::JAVELIN),
            EM_FIREPATH => Ok(Self::FIREPATH),
            EM_ZSP => Ok(Self::ZSP),
            EM_MMIX => Ok(Self::MMIX),
            EM_HUANY => Ok(Self::HUANY),
            EM_PRISM => Ok(Self::PRISM),
            EM_AVR => Ok(Self::AVR),
            EM_FR30 => Ok(Self::FR30),
            EM_D10V => Ok(Self::D10V),
            EM_D30V => Ok(Self::D30V),
            EM_V850 => Ok(Self::V850),
            EM_M32R => Ok(Self::M32R),
            EM_MN10300 => Ok(Self::MN10300),
            EM_MN10200 => Ok(Self::MN10200),
            EM_PJ => Ok(Self::PJ),
            EM_OPENRISC => Ok(Self::OPENRISC),
            EM_ARC_COMPACT => Ok(Self::ArcCompact),
            EM_XTENSA => Ok(Self::XTENSA),
            EM_VIDEOCORE => Ok(Self::VIDEOCORE),
            EM_TMM_GPP => Ok(Self::TmmGpp),
            EM_NS32K => Ok(Self::NS32K),
            EM_TPC => Ok(Self::TPC),
            EM_SNP1K => Ok(Self::SNP1K),
            EM_ST200 => Ok(Self::ST200),
            EM_IP2K => Ok(Self::IP2K),
            EM_MAX => Ok(Self::MAX),
            EM_CR => Ok(Self::CR),
            EM_F2MC16 => Ok(Self::F2MC16),
            EM_MSP430 => Ok(Self::MSP430),
            EM_BLACKFIN => Ok(Self::BLACKFIN),
            EM_SE_C33 => Ok(Self::SeC33),
            EM_SEP => Ok(Self::SEP),
            EM_ARCA => Ok(Self::ARCA),
            EM_UNICORE => Ok(Self::UNICORE),
            EM_EXCESS => Ok(Self::EXCESS),
            EM_DXP => Ok(Self::DXP),
            EM_ALTERA_NIOS2 => Ok(Self::AlteraNios2),
            EM_CRX => Ok(Self::CRX),
            EM_XGATE => Ok(Self::XGATE),
            EM_C166 => Ok(Self::C166),
            EM_M16C => Ok(Self::M16C),
            EM_DSPIC30F => Ok(Self::DSPIC30F),
            EM_CE => Ok(Self::CE),
            EM_M32C => Ok(Self::M32C),
            EM_TSK3000 => Ok(Self::TSK3000),
            EM_RS08 => Ok(Self::RS08),
            EM_SHARC => Ok(Self::SHARC),
            EM_ECOG2 => Ok(Self::ECOG2),
            EM_SCORE7 => Ok(Self::SCORE7),
            EM_DSP24 => Ok(Self::DSP24),
            EM_VIDEOCORE3 => Ok(Self::VIDEOCORE3),
            EM_LATTICEMICO32 => Ok(Self::LATTICEMICO32),
            EM_SE_C17 => Ok(Self::SeC17),
            EM_TI_C6000 => Ok(Self::TiC6000),
            EM_TI_C2000 => Ok(Self::TiC2000),
            EM_TI_C5500 => Ok(Self::TiC5500),
            EM_TI_ARP32 => Ok(Self::TiARP32),
            EM_TI_PRU => Ok(Self::TiPRU),
            EM_MMDSP_PLUS => Ok(Self::MmdspPlus),
            EM_CYPRESS_M8C => Ok(Self::CypressM8C),
            EM_R32C => Ok(Self::R32C),
            EM_TRIMEDIA => Ok(Self::TRIMEDIA),
            EM_QDSP6 => Ok(Self::QDSP6),
            EM_8051 => Ok(Self::EM8051),
            EM_STXP7X => Ok(Self::STXP7X),
            EM_NDS32 => Ok(Self::NDS32),
            EM_ECOG1X => Ok(Self::ECOG1X),
            EM_MAXQ30 => Ok(Self::MAXQ30),
            EM_XIMO16 => Ok(Self::XIMO16),
            EM_MANIK => Ok(Self::MANIK),
            EM_CRAYNV2 => Ok(Self::CRAYNV2),
            EM_RX => Ok(Self::RX),
            EM_METAG => Ok(Self::METAG),
            EM_MCST_ELBRUS => Ok(Self::McstElbrus),
            EM_ECOG16 => Ok(Self::ECOG16),
            EM_CR16 => Ok(Self::CR16),
            EM_ETPU => Ok(Self::ETPU),
            EM_SLE9X => Ok(Self::SLE9X),
            EM_L10M => Ok(Self::L10M),
            EM_K10M => Ok(Self::K10M),
            EM_AARCH64 => Ok(Self::AARCH64),
            EM_AVR32 => Ok(Self::AVR32),
            EM_STM8 => Ok(Self::STM8),
            EM_TILE64 => Ok(Self::TILE64),
            EM_TILEPRO => Ok(Self::TILEPRO),
            EM_MICROBLAZE => Ok(Self::MICROBLAZE),
            EM_CUDA => Ok(Self::CUDA),
            EM_TILEGX => Ok(Self::TILEGX),
            EM_CLOUDSHIELD => Ok(Self::CLOUDSHIELD),
            EM_COREA_1ST => Ok(Self::Corea1ST),
            EM_COREA_2ND => Ok(Self::Corea2ND),
            EM_ARCV2 => Ok(Self::ARCV2),
            EM_OPEN8 => Ok(Self::OPEN8),
            EM_RL78 => Ok(Self::RL78),
            EM_VIDEOCORE5 => Ok(Self::VIDEOCORE5),
            EM_78KOR => Ok(Self::EM78KOR),
            EM_56800EX => Ok(Self::EM56800EX),
            EM_BA1 => Ok(Self::BA1),
            EM_BA2 => Ok(Self::BA2),
            EM_XCORE => Ok(Self::XCORE),
            EM_MCHP_PIC => Ok(Self::MchpPic),
            EM_INTELGT => Ok(Self::INTELGT),
            EM_KM32 => Ok(Self::KM32),
            EM_KMX32 => Ok(Self::KMX32),
            EM_EMX16 => Ok(Self::EMX16),
            EM_EMX8 => Ok(Self::EMX8),
            EM_KVARC => Ok(Self::KVARC),
            EM_CDP => Ok(Self::CDP),
            EM_COGE => Ok(Self::COGE),
            EM_COOL => Ok(Self::COOL),
            EM_NORC => Ok(Self::NORC),
            EM_CSR_KALIMBA => Ok(Self::CsrKalimba),
            EM_Z80 => Ok(Self::Z80),
            EM_VISIUM => Ok(Self::VISIUM),
            EM_FT32 => Ok(Self::FT32),
            EM_MOXIE => Ok(Self::MOXIE),
            EM_AMDGPU => Ok(Self::AMDGPU),
            EM_RISCV => Ok(Self::RISCV),
            EM_BPF => Ok(Self::BPF),
            EM_CSKY => Ok(Self::CSKY),
            EM_LOONGARCH => Ok(Self::LOONGARCH),
            EM_NUM => Ok(Self::NUM),
            _ => Err(EhdrParseError::InvalidEMachine(value)),
        }
    }
}
