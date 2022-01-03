use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub struct LexPosition {
    line: u32,
    column: u32,
}

impl LexPosition {
    pub fn new(line: u32, column: u32) -> Self {
        LexPosition { line, column }
    }

    pub fn zero() -> Self {
        LexPosition::new(0, 0)
    }
}

use std::cmp::Ordering;
impl PartialOrd for LexPosition {
    fn lt(&self, other: &Self) -> bool {
        if self.line == other.line {
            self.column < other.column
        } else {
            self.line < other.line
        }
    }

    fn le(&self, other: &Self) -> bool {
        self.eq(other) || self.lt(other)
    }

    fn gt(&self, other: &Self) -> bool {
        if self.line == other.line {
            self.column > other.column
        } else {
            self.line > other.line
        }
    }

    fn ge(&self, other: &Self) -> bool {
        self.eq(other) || self.gt(other)
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.lt(other) {
            Some(Ordering::Less)
        } else if self.gt(other) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl Display for LexPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(line: {}, column: {})", self.line, self.column)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct LexLocation {
    begin: LexPosition,
    end: LexPosition,
}

impl LexLocation {
    pub fn new(begin: LexPosition, end: LexPosition) -> Self {
        LexLocation { begin, end }
    }

    pub fn line_offset(begin: LexPosition, length: u32) -> Self {
        LexLocation::new(begin, LexPosition::new(begin.line, begin.column + length))
    }

    pub fn line_zero(begin: LexPosition) -> Self {
        LexLocation::line_offset(begin, 0u32)
    }

    pub fn zero() -> Self {
        LexLocation::new(LexPosition::zero(), LexPosition::zero())
    }

    pub fn enclose(&self, other: &Self) -> bool {
        self.begin.le(&other.begin) && self.end.ge(&other.end)
    }

    pub fn contains(&self, position: &LexPosition) -> bool {
        self.begin.le(position) && self.end.gt(position)
    }

    pub fn contains_closed(&self, position: &LexPosition) -> bool {
        self.begin.le(position) && self.end.ge(position)
    }

    pub fn get_begin(&self) -> LexPosition {
        self.begin
    }

    pub fn get_end(&self) -> LexPosition {
        self.end
    }
}

impl Display for LexLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(begin: {}, end: {})", self.begin, self.end)
    }
}
