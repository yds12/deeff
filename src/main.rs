mod asm_file;
mod config;
mod create_asm;
mod diff;
mod line;
mod read_asm;

pub use asm_file::AsmFile;
pub use config::Config;
pub use line::{AsmLine, Line};

fn line_diff(config: Config) {
    let section = config.section.as_ref().expect("need to supply --section");
    let left = config.left_ix.expect("need to supply --left-ix");
    let right = config.right_ix.expect("need to supply --right-ix");
    let right_file = config
        .right_file
        .as_ref()
        .expect("must provide second file name");

    let left_asm = create_asm::create_asm_for_arg(&config.left_file);
    let right_asm = create_asm::create_asm_for_arg(right_file);

    if let (Some(left_asm), Some(right_asm)) = (left_asm, right_asm) {
        let asm1 = read_asm::read_asm_from_memory(left_asm);
        let asm2 = read_asm::read_asm_from_memory(right_asm);

        let lines1 = asm1.get_block_lines(section, left);
        let lines2 = asm2.get_block_lines(section, right);

        let diff = diff::diff(&lines1, &lines2, |a, b| match (a.inner(), b.inner()) {
            (AsmLine::Instruction(i), AsmLine::Instruction(j)) => i.op() == j.op(),
            (a, b) => a == b,
        });

        diff::print_alignment(
            &lines1,
            &lines2,
            diff,
            |line| line.as_str(),
            |l1, l2| match (l1.inner(), l2.inner()) {
                (AsmLine::Instruction(i), AsmLine::Instruction(j)) => i.content() == j.content(),
                (a, b) => a == b,
            },
            config,
        );
    }
}

fn block_diff(config: Config) {
    let section = config.section.as_ref().expect("need to supply --section");
    let right_file = config
        .right_file
        .as_ref()
        .expect("must provide second file name");

    let left_asm = create_asm::create_asm_for_arg(&config.left_file);
    let right_asm = create_asm::create_asm_for_arg(right_file);

    if let (Some(left_asm), Some(right_asm)) = (left_asm, right_asm) {
        let asm1 = read_asm::read_asm_from_memory(left_asm);
        let asm2 = read_asm::read_asm_from_memory(right_asm);

        let lines1 = asm1.get_section_blocks(section);
        let lines2 = asm2.get_section_blocks(section);

        let nohash = config.no_hash;
        let alignment = diff::align(&lines1, &lines2, |bl1, bl2| {
            match (bl1.inner(), bl2.inner()) {
                (AsmLine::Label(a), AsmLine::Label(b)) if nohash => a.clean_name() == b.clean_name(),
                (AsmLine::Label(a), AsmLine::Label(b)) if !nohash => a.demangled_name() == b.demangled_name(),
                (a, b) => a == b
            }
        });

        diff::print_alignment(
            &lines1,
            &lines2,
            alignment,
            |block| match block.inner() {
                AsmLine::Label(l) => l.demangled_name(),
                _ => block.as_str()
            },
            |bl1, bl2| match (bl1.inner(), bl2.inner()) {
                (AsmLine::Label(a), AsmLine::Label(b)) if nohash => a.clean_name() == b.clean_name(),
                (AsmLine::Label(a), AsmLine::Label(b)) if !nohash => a.demangled_name() == b.demangled_name(),
                (a, b) => a == b
            },
            config,
        );
    }
}

fn section_diff(config: Config) {
    let right_file = config
        .right_file
        .as_ref()
        .expect("must provide second file name");

    let left_asm = create_asm::create_asm_for_arg(&config.left_file);
    let right_asm = create_asm::create_asm_for_arg(right_file);

    if let (Some(left_asm), Some(right_asm)) = (left_asm, right_asm) {
        let asm1 = read_asm::read_asm_from_memory(left_asm);
        let asm2 = read_asm::read_asm_from_memory(right_asm);

        let alignment = diff::align(&asm1.sections(), &asm2.sections(),
            |sec_a, sec_b| match (sec_a.inner(), sec_b.inner()) {
            (AsmLine::SectionHeader(a), AsmLine::SectionHeader(b)) => a.name() == b.name(),
            (a, b) => a == b
        });

        diff::print_alignment(
            &asm1.sections(),
            &asm2.sections(),
            alignment,
            |sec| match sec.inner() {
                AsmLine::SectionHeader(h) => h.name(),
                _ => sec.as_str()
            },
            |a, b| match (a.inner(), b.inner()) {
                (AsmLine::SectionHeader(a), AsmLine::SectionHeader(b)) => a.name() == b.name(),
                (a, b) => a == b
            },
            config,
        );
    }
}

fn disassemble(config: Config) {
    let left_asm = create_asm::create_asm_for_arg(&config.left_file);

    if let Some(left_asm) = left_asm {
        println!("{}", String::from_utf8_lossy(&left_asm.stdout));
    }
}

fn summary(config: Config) {
    let typ = config
        .summary_type
        .as_ref()
        .expect("need to supply --summary-type");
    let left_asm = create_asm::create_asm_for_arg(&config.left_file);

    if let Some(left_asm) = left_asm {
        let asm = read_asm::read_asm_from_memory(left_asm);

        match typ.as_ref() {
            "global" => asm.print_stats(),
            "sections" => asm.print_section_stats(),
            "section" => {
                let section_name = config.section.as_ref().expect("must provide --section");
                // TODO
                /*
                let section = asm
                    .get_section(section_name)
                    .expect("no section with provided name");
                 section.print_block_summary();
                */
            }
            _ => panic!("unknown --summary-type"),
        }
    }
}

fn main() {
    let config = Config::get();
    match config.mode.as_str() {
        "summary" => summary(config),
        "disassemble" => disassemble(config),
        "diff" => match config.level.as_deref() {
            Some("section") => section_diff(config),
            Some("block") => block_diff(config),
            Some("line") => line_diff(config),
            None => panic!("must provide --level"),
            _ => panic!("unknown --level"),
        },
        _ => panic!("unknown --mode"),
    }
}
