mod asm_file;
mod create_asm;
mod line;
mod read_asm;

pub use asm_file::AsmFile;
pub use line::Line;

fn main() {
    let res = create_asm::init();

    if res.is_ok() {
        let file1 = res.get_first().unwrap();
        let file2 = res.get_second().unwrap();

        read_asm::read_asm(file1);
        read_asm::read_asm(file2);
    }

    res.cleanup();
}
