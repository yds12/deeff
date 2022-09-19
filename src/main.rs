use clap::Parser as _;
use once_cell::sync::Lazy;

mod align;
mod asm_file;
mod create_asm;
mod diff;
mod line;
mod read_asm;

pub use asm_file::AsmFile;
pub use line::Line;

pub static CFG: Lazy<Config> = Lazy::new(|| {
    Config::parse()
});

#[derive(Debug, clap::Parser)]
#[clap(author, version, about)]
pub struct Config {
    #[clap(value_parser)]
    left_file: String,
    #[clap(value_parser)]
    right_file: String,
    #[clap(long, value_parser)]
    mode: String,
    #[clap(long)]
    remove_hashes: bool,
    #[clap(long, value_parser)]
    section_name: Option<String>,
    #[clap(long, value_parser)]
    left_block_index: Option<usize>,
    #[clap(long, value_parser)]
    right_block_index: Option<usize>,
}

fn do_diff() {
    let section = CFG.section_name.as_ref().expect("need to supply --section-name");
    let left = CFG
        .left_block_index
        .expect("need to supply --left-block-index");
    let right = CFG
        .right_block_index
        .expect("need to supply --right-block-index");

    let res = create_asm::init(&CFG.left_file, &CFG.right_file);

    if res.is_ok() {
        let file1 = res.get_first().unwrap();
        let file2 = res.get_second().unwrap();

        let asm1 = read_asm::read_asm(file1);
        let asm2 = read_asm::read_asm(file2);

        let alignment = align::align(&asm1.sections(), &asm2.sections(), |sec_a, sec_b| {
            sec_a.name() == sec_b.name()
        });

        let text1 = asm1.get_section(section).unwrap();
        let text2 = asm2.get_section(section).unwrap();

        let lines1 = text1.blocks()[left].lines();
        let lines2 = text2.blocks()[right].lines();
        let diff = diff::diff(lines1, lines2, |a, b| match (a, b) {
            (Line::Instruction(i), Line::Instruction(j)) => i.op() == j.op(),
            (a, b) => a == b,
        });
        align::print_alignment(
            &lines1,
            &lines2,
            diff,
            false,
            true,
            |line| line.as_str(),
            |l1, l2| match (l1, l2) {
                (Line::Instruction(i), Line::Instruction(j)) => i.content() == j.content(),
                (a, b) => a == b,
            },
        );
    }

    res.cleanup();
}

fn do_section_diff() {
    let section = CFG.section_name.as_ref().expect("need to supply --section-name");

    let res = create_asm::init(&CFG.left_file, &CFG.right_file);

    if res.is_ok() {
        let file1 = res.get_first().unwrap();
        let file2 = res.get_second().unwrap();

        let asm1 = read_asm::read_asm(file1);
        let asm2 = read_asm::read_asm(file2);

        let alignment = align::align(&asm1.sections(), &asm2.sections(), |sec_a, sec_b| {
            sec_a.name() == sec_b.name()
        });

        let text1 = asm1.get_section(section).unwrap();
        let text2 = asm2.get_section(section).unwrap();

        let alignment = align::align(&text1.blocks(), &text2.blocks(), |bl1, bl2| {
            bl1.demangled_label() == bl2.demangled_label()
        });

        align::print_alignment(
            &text1.blocks(),
            &text2.blocks(),
            alignment,
            false,
            true,
            |block| block.demangled_label(),
            |bl1, bl2| bl1.label() == bl2.label(),
        );
    }

    res.cleanup();
}

fn main() {
    match CFG.mode.as_str() {
        "section" => do_section_diff(),
        "block" => do_diff(),
        _ => panic!("unknown mode"),
    }
}
