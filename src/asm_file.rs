use crate::Line;

#[derive(Debug)]
pub struct Block(Line, Vec<Line>);

impl Block {
    fn new(line: Line) -> Self {
        let mut blocks = Vec::new();
        Self(line, blocks)
    }

    fn push(&mut self, line: Line) {
        self.1.push(line);
    }

    fn get_stats(&self) -> (usize, usize, usize) {
        let instructions = self
            .1
            .iter()
            .filter(|line| matches!(line, Line::Instruction(_)))
            .count();
        let blanks = self
            .1
            .iter()
            .filter(|line| matches!(line, Line::Blank))
            .count();
        let others = self
            .1
            .iter()
            .filter(|line| matches!(line, Line::Other(_)))
            .count();

        (instructions, blanks, others)
    }
}

#[derive(Debug)]
pub struct Section(Line, Vec<Block>, Block);

impl Section {
    fn new(line: Line) -> Self {
        Self(line, Vec::new(), Block::new(Line::Blank))
    }

    fn push(&mut self, line: Line) {
        if let Line::Label(_) = &line {
            self.new_block(line);
        } else {
            self.push_to_last_block(line);
        }
    }

    fn new_block(&mut self, line: Line) {
        self.1.push(Block::new(line));
    }

    fn push_to_last_block(&mut self, line: Line) {
        match self.1.len() {
            l if l == 0 => self.2.push(line),
            l => self.1[l - 1].push(line),
        }
    }

    fn get_stats(&self) -> (usize, usize, usize, usize) {
        let sum = self
            .1
            .iter()
            .map(|block| block.get_stats())
            .fold(self.2.get_stats(), |acc, bl_stat| {
                (acc.0 + bl_stat.0, acc.1 + bl_stat.1, acc.2 + bl_stat.2)
            });

        (sum.0, sum.1, sum.2, self.1.len())
    }
}

#[derive(Debug)]
pub struct AsmFile(Vec<Section>, Section);

impl AsmFile {
    pub fn new() -> Self {
        let mut sections = Vec::new();
        Self(sections, Section::new(Line::Blank))
    }

    pub fn push(&mut self, line: Line) {
        if let Line::SectionHeader(_) = &line {
            self.new_section(line);
        } else {
            self.push_to_last_section(line);
        }
    }

    fn new_section(&mut self, line: Line) {
        self.0.push(Section::new(line));
    }

    fn push_to_last_section(&mut self, line: Line) {
        match self.0.len() {
            l if l == 0 => self.1.push(line),
            l => self.0[l - 1].push(line),
        }
    }

    fn get_stats(&self) -> (usize, usize, usize, usize, usize) {
        let sum = self.0.iter().map(|section| section.get_stats()).fold(
            self.1.get_stats(),
            |acc, sec_stat| {
                (
                    acc.0 + sec_stat.0,
                    acc.1 + sec_stat.1,
                    acc.2 + sec_stat.2,
                    acc.3 + sec_stat.3,
                )
            },
        );

        (sum.0, sum.1, sum.2, sum.3, self.0.len())
    }

    pub fn print_stats(&self) {
        let (instructions, blanks, others, labels, sections) = self.get_stats();

        println!("sections: {}", sections);
        println!("labels: {}", labels);
        println!("instructions: {}", instructions);
        println!("blanks: {}", blanks);
        println!("other: {}", others);
    }
}
