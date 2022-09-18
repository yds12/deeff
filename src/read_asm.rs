use crate::AsmFile;
use crate::Line;

pub fn read_asm(filename: &str) {
    use std::io::BufRead;
    let file = std::fs::File::open(filename).expect(&format!("could not read {}", filename));
    let mut asm = AsmFile::new();

    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let line = Line::from_str(&line);
            asm.push(line);
        }
    }

    println!("Stats for {}", filename);
    asm.print_stats();
}
