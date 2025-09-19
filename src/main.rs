fn main() {
    let file_path = "assets/a.out";
    let bin = std::fs::read(file_path).expect("file read error");

    let res = elf_parser::elf::Elf32::try_from(&bin[..]);

    eprintln!("{res:#?}");
}
