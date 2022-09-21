use super::{demangle, demangle_no_hash, Offset, RE_INSTR};

#[derive(Debug, PartialEq)]
pub struct Instruction {
    op: String,
    operands: Vec<String>,
    operands_demangled: Vec<String>,
    operands_no_hash: Vec<String>,
    offset: Offset,
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        let caps = RE_INSTR.captures(line).unwrap();
        let offset = caps.get(1).unwrap().as_str();
        let op = caps.get(2).unwrap().as_str().to_owned();

        let mut operands = Vec::new();
        let mut operands_demangled = Vec::new();
        let mut operands_no_hash = Vec::new();
        for i in 3..caps.len() {
            if let Some(operand) = caps.get(i) {
                operands.push(operand.as_str().to_owned());
                operands_demangled.push(demangle(operand.as_str()));
                operands_no_hash.push(demangle_no_hash(operand.as_str()));
            }
        }

        Self {
            op,
            operands,
            operands_demangled,
            operands_no_hash,
            offset: Offset(offset.to_owned()),
        }
    }

    pub fn op(&self) -> &str {
        &self.op
    }

    pub fn content(&self) -> String {
        format!("{} {}", self.op, self.operands.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn do_not_panic() {
        let lines = [
            "  465f00:	lea    rax,[rip+0x211581]        # 677488 <__dso_handle>",
            "  465f07:	test   rax,rax",
            "  465f0a:	je     465f20 <atexit+0x20>",
            "  465f0c:	mov    rdx,QWORD PTR [rax]",
            "  465f0f:	xor    esi,esi",
            "  465f11:	jmp    40d510 <__cxa_atexit@plt>",
            "  465f16:	nop    WORD PTR cs:[rax+rax*1+0x0]",
            "  465f20:	xor    edx,edx",
            "  465f22:	xor    esi,esi",
            "  465f24:	jmp    40d510 <__cxa_atexit@plt>",
            "  465f29:	nop    DWORD PTR [rax+0x0]",
        ];

        for line in lines {
            let _ = Instruction::new(line);
        }
    }
}
