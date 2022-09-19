mod asm_file;
mod config;
mod create_asm;
mod diff;
mod line;
mod read_asm;

pub use asm_file::AsmFile;
pub use config::CFG;
pub use line::Line;

fn do_diff() {
    let section = CFG
        .section_name
        .as_ref()
        .expect("need to supply --section-name");
    let left = CFG
        .left_block_index
        .expect("need to supply --left-block-index");
    let right = CFG
        .right_block_index
        .expect("need to supply --right-block-index");

    let left_asm = create_asm::create_asm_for_arg(&CFG.left_file);
    let right_asm = create_asm::create_asm_for_arg(&CFG.right_file);

    if let (Some(left_asm), Some(right_asm)) = (left_asm, right_asm) {
        let asm1 = read_asm::read_asm_from_memory(left_asm);
        let asm2 = read_asm::read_asm_from_memory(right_asm);

        let text1 = asm1.get_section(section).unwrap();
        let text2 = asm2.get_section(section).unwrap();

        let lines1 = text1.blocks()[left].lines();
        let lines2 = text2.blocks()[right].lines();
        let diff = diff::diff(lines1, lines2, |a, b| match (a, b) {
            (Line::Instruction(i), Line::Instruction(j)) => i.op() == j.op(),
            (a, b) => a == b,
        });
        diff::print_alignment(
            &lines1,
            &lines2,
            diff,
            |line| line.as_str(),
            |l1, l2| match (l1, l2) {
                (Line::Instruction(i), Line::Instruction(j)) => i.content() == j.content(),
                (a, b) => a == b,
            },
        );
    }
}

fn do_section_diff() {
    let section = CFG
        .section_name
        .as_ref()
        .expect("need to supply --section-name");

    let left_asm = create_asm::create_asm_for_arg(&CFG.left_file);
    let right_asm = create_asm::create_asm_for_arg(&CFG.right_file);

    if let (Some(left_asm), Some(right_asm)) = (left_asm, right_asm) {
        let asm1 = read_asm::read_asm_from_memory(left_asm);
        let asm2 = read_asm::read_asm_from_memory(right_asm);

        /*
        let alignment = align::align(&asm1.sections(), &asm2.sections(), |sec_a, sec_b| {
            sec_a.name() == sec_b.name()
        });
        */

        let text1 = asm1.get_section(section).unwrap();
        let text2 = asm2.get_section(section).unwrap();

        let alignment = diff::align(&text1.blocks(), &text2.blocks(), |bl1, bl2| {
            bl1.demangled_label() == bl2.demangled_label()
        });

        diff::print_alignment(
            &text1.blocks(),
            &text2.blocks(),
            alignment,
            |block| block.demangled_label(),
            |bl1, bl2| bl1.label() == bl2.label(),
        );
    }
}

fn do_summary() {
    let typ = CFG
        .summary_type
        .as_ref()
        .expect("need to supply --summary-type");
    let left_asm = create_asm::create_asm_for_arg(&CFG.left_file);

    if let Some(left_asm) = left_asm {
        let asm1 = read_asm::read_asm_from_memory(left_asm);
    }
}

fn main() {
    match CFG.mode.as_str() {
        "section" => do_section_diff(),
        "block" => do_diff(),
        "summary" => do_summary(),
        _ => panic!("unknown mode"),
    }
}
