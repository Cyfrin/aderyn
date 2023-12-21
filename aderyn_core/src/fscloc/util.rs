use std::str::FromStr;

pub struct CodeIterator {
    pub content: Vec<char>,
    pub curr_pos: usize,
    pub line_no: usize,
}

pub struct StatefulToken {
    pub curr: char,
    pub next: Option<char>,
    pub line_no: usize,
}

impl Iterator for CodeIterator {
    type Item = StatefulToken;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_pos = self.curr_pos;
        self.curr_pos += 1;
        let result = {
            if curr_pos < self.content.len() - 1 {
                Some(StatefulToken {
                    curr: self.content[curr_pos],
                    next: Some(self.content[curr_pos + 1]),
                    line_no: self.line_no,
                })
            } else if curr_pos < self.content.len() {
                Some(StatefulToken {
                    curr: self.content[curr_pos],
                    next: None,
                    line_no: self.line_no,
                })
            } else {
                None
            }
        };

        if let Some(ref c) = result {
            if c.curr == '\n' {
                self.line_no += 1;
            }
        }

        result
    }
}

impl FromStr for CodeIterator {
    type Err = usize;

    fn from_str(code: &str) -> Result<Self, <Self as FromStr>::Err> {
        return Ok(CodeIterator {
            content: code.chars().collect::<Vec<_>>(),
            curr_pos: 0,
            line_no: 1,
        });
    }
}
