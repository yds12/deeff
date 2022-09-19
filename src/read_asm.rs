use crate::AsmFile;
use crate::Line;

pub fn read_asm_from_memory(output: std::process::Output) -> AsmFile {
    let mut asm = AsmFile::new();
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.split("\n") {
        let line = Line::from_str(&line);
        asm.push(line);
    }

    asm
}
