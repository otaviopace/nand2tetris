use std::fmt;

pub enum Instr {
    Push(Segment, u16),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

pub enum Segment {
    Const,
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Push(segment, i) => match segment {
                Segment::Const => format!(
                    "\
@{i}
D=A
@SP
A=M
M=D
@SP
M=M+1"
                ),
            },
            // arithmetic
            Self::Add => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1"
                .into(),
            Self::Sub => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1"
                .into(),
            Self::Neg => "\
@SP
M=M-1
A=M
M=-M
@SP
M=M+1"
                .into(),
            // logical
            Self::Eq => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@LOGICAL_EQ_BODY
D;JEQ

// (IF-else) -> no equality found
M=-1
@LOGICAL_EQ_END
0;JMP

// (IF-then) it's equal to zero, which means equality
(LOGICAL_EQ_BODY)
M=0

(LOGICAL_EQ_END)
@SP
M=M+1"
                .into(),
            Self::Gt => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@LOGICAL_GT_BODY
D;JGE

// (IF-else) -> no greater than found
M=-1
@LOGICAL_GT_END
0;JMP

// (IF-then) it's greater than or equal to zero
(LOGICAL_GT_BODY)
M=0

(LOGICAL_GT_END)
@SP
M=M+1"
                .into(),
            Self::Lt => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@LOGICAL_LT_BODY
D;JLE

// (IF-else) -> no less than found
M=-1
@LOGICAL_LT_END
0;JMP

// (IF-then) it's less than or equal to zero
(LOGICAL_LT_BODY)
M=0

(LOGICAL_LT_END)
@SP
M=M+1"
                .into(),
            Self::And => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D&M
@SP
M=M+1"
                .into(),
            Self::Or => "\
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D|M
@SP
M=M+1"
                .into(),
            Self::Not => "\
@SP
M=M-1
A=M
M=!M
@SP
M=M+1"
                .into(),
        };

        write!(f, "{}", s)
    }
}
