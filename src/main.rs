use byteorder::{LittleEndian, WriteBytesExt};
use object::write::{Object, Relocation, StandardSection, Symbol, SymbolScope, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, RelocationEncoding, RelocationFlags, RelocationKind, SymbolFlags, SymbolKind
};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut obj = Object::new(
        BinaryFormat::Elf,
        Architecture::X86_64,
        Endianness::Little,
    );

    // === Machine code ===
    // mov edi, 42         (exit code)
    // call exit           (with relocation)
    let mut code: Vec<u8> = vec![
        0xbf, 0x2a, 0x00, 0x00, 0x00,     // mov edi, 42
        0xe8, 0x00, 0x00, 0x00, 0x00,     // call exit (with relocation)
    ];

    // Get the .text section
    let text_section = obj.section_id(StandardSection::Text);

    // Define `main` symbol
    let main_symbol = obj.add_symbol(Symbol {
        name: b"main".into(),
        value: 0,
        size: code.len() as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(text_section),
        flags: SymbolFlags::None,
    });

    let offset = obj.add_symbol_data(main_symbol, text_section, &code, 1);

    // Define external `exit` symbol (dynamic linker will resolve it)
    let exit_symbol = obj.add_symbol(Symbol {
        name: b"exit".into(),
        value: 0,
        size: 0,
        kind: SymbolKind::Text,
        scope: SymbolScope::Dynamic,
        weak: false,
        section: SymbolSection::Undefined,
        flags: SymbolFlags::None,
    });

    // Add relocation for `call exit`
    obj.add_relocation(
        text_section,
        Relocation {
            offset: offset + 5, // the offset of the call's 4-byte displacement
            symbol: exit_symbol,
            addend: -4,
            flags: RelocationFlags::Generic {
                kind: RelocationKind::PltRelative,
                encoding: RelocationEncoding::X86Branch,
                size: 32,
            },
        },
    )?;

    // Write to file
    let mut file = std::fs::File::create("call_exit.o")?;
    obj.write_stream(&mut file)?;

    println!("Wrote call_exit.o");
    Ok(())
}