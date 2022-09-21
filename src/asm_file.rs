use crate::{AsmLine, Line};

#[derive(Debug)]
pub struct AsmFile {
    raw: Vec<Line>,
}

impl AsmFile {
    pub fn new() -> Self {
        Self { raw: Vec::new() }
    }

    pub fn push(&mut self, line: Line) {
        self.raw.push(line);
    }

    pub fn sections(&self) -> Vec<Line> {
        self.raw
            .iter()
            .filter(|line| matches!(line.inner(), AsmLine::SectionHeader(_)))
            .cloned()
            .collect()
    }

    pub fn get_block_lines(&self, section: &str, block_id: usize) -> Vec<Line> {
        #[derive(Copy, Clone)]
        enum State {
            Start,
            FoundSec(usize),
            FoundBlock
        };

        let mut lines = Vec::new();
        let mut state = State::Start;

        for line in self.raw.iter() {
            match (state, line.inner()) {
                (State::Start, AsmLine::SectionHeader(s)) if s.name() == section => {
                    state = State::FoundSec(0)
                }
                (State::Start, _) => continue,
                (State::FoundSec(n), AsmLine::Label(_)) if n == block_id => state = State::FoundBlock,
                (State::FoundSec(n), AsmLine::Label(_)) if n < block_id => state = State::FoundSec(n + 1),
                (State::FoundBlock, AsmLine::Instruction(_)) => lines.push(line.clone()),
                (State::FoundBlock, AsmLine::Label(_)) => break,
                _ => continue,
            }
        }

        lines
    }

    pub fn get_section_blocks(&self, name: &str) -> Vec<Line> {
        let mut blocks = Vec::new();
        let mut started = false;

        for line in self.raw.iter() {
            match (started, line.inner()) {
                (true, AsmLine::Label(_)) => blocks.push(line.clone()),
                (true, AsmLine::SectionHeader(_)) => break,
                (false, AsmLine::SectionHeader(section)) if section.name() == name => {
                    started = true
                }
                _ => continue,
            }
        }

        blocks
    }

    fn get_stats(&self, begin: usize, end: usize) -> (usize, usize, usize, usize, usize) {
        let slice = &self.raw[begin..end];

        slice
            .iter()
            .map(|line| match line.inner() {
                AsmLine::SectionHeader(_) => (1usize, 0usize, 0usize, 0usize, 0usize),
                AsmLine::Label(_) => (0, 1, 0, 0, 0),
                AsmLine::Instruction(_) => (0, 0, 1, 0, 0),
                AsmLine::Blank => (0, 0, 0, 1, 0),
                AsmLine::Other => (0, 0, 0, 0, 1),
            })
            .fold((0, 0, 0, 0, 0), |a, b| {
                (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3, a.4 + b.4)
            })
    }

    pub fn print_section_stats(&self) {
        let section_ixs = self.raw
            .iter()
            .enumerate()
            .filter(|(_, line)| matches!(line.inner(), AsmLine::SectionHeader(_)))
            .map(|(i, line)| i)
            .collect::<Vec<_>>();

        for pair in section_ixs.windows(2) {
            if let &[sec1, sec2] = pair {
                let (_, instructions, blanks, others, labels) = self.get_stats(sec1, sec2);
                let sec_name = match self.raw[sec1].inner() {
                    AsmLine::SectionHeader(sec) => sec.name(),
                    _ => unreachable!()
                };

                println!("section {}:", sec_name);
                println!("  labels: {}", labels);
                println!("  instructions: {}", instructions);
                println!("  blanks: {}", blanks);
                println!("  other: {}", others);
                println!();
            }
        }
    }

    pub fn print_stats(&self) {
        let stats = self.get_stats(0, self.raw.len());
        println!("sections: {}", stats.0);
        println!("labels: {}", stats.1);
        println!("instructions: {}", stats.2);
        println!("blanks: {}", stats.3);
        println!("other: {}", stats.4);
    }
}
