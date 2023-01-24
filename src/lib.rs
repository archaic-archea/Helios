pub struct Operation {
    opcode: Op,
    operands: Vec<u64>
}

impl Operation {
    pub fn new(opcode: Op, operands: Vec<u64>) -> Operation {
        Operation { opcode, operands }
    }

    pub fn perform(&self) -> u64 {
        use Op::*;

        match &self.opcode {
            Nop => return 0,
            AddI => {
                let mut result = 0;

                for operand in self.operands.iter() {
                    result += operand;
                }

                return result;
            },
            SubI => {
                let mut result = 0;

                for operand in self.operands.iter() {
                    result -= operand;
                }

                return result;
            },
            _ => return 0
        }
    }
}

pub enum Op {
    Nop = 0,
    AddI,
    SubI,
    MulI,
    DivI,
    AddF,
    SubF,
    MulF,
    DivF,
}