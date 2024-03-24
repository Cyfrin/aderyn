use crate::fscloc::{
    cloc::TokenInsightGroup,
    token::{TokenDescriptor, TokenType},
    CodeLines,
};

#[derive(Clone, Debug)]
pub struct TokenInsight {
    pub token_type: TokenType,
    pub start_line: usize,
    pub end_line: usize,
    pub code_lines: CodeLines,
    pub starts_with_newline: bool,
    pub ends_with_newline: bool,
}

impl From<&TokenDescriptor> for TokenInsight {
    #[allow(clippy::let_and_return)]
    fn from(value: &TokenDescriptor) -> Self {
        let insight = TokenInsight {
            code_lines: match value.token_type.clone() {
                TokenType::CodeDoubleQuotes | TokenType::CodeSingleQuotes => CodeLines {
                    info_lines: value.end_line - value.start_line + 1,
                    actual_first_line: 0,
                },
                _ => value.token_type.number_of_lines(&value.content),
            },
            start_line: value.start_line,
            end_line: value.end_line,
            token_type: value.token_type.clone(),
            starts_with_newline: value.content.starts_with('\n'),
            ends_with_newline: value.content.ends_with('\n'),
        };

        // if value.token_type != TokenType::MultilineComment
        //     && value.token_type != TokenType::SinglelineComment
        // && insight.code_lines.actual_first_line != -1
        //{
        // print!(
        //     "{:?}, {}-{}\n",
        //     value.token_type.clone(),
        //     value.start_line,
        //     value.end_line
        // );
        // println!("{}", value.content.clone());

        // println!(
        //     "Lines {}/{}\n----------------------",
        //     insight.code_lines.info_lines, insight.code_lines.actual_first_line
        // );
        //}

        insight
    }
}

impl TokenType {
    pub fn number_of_lines(&self, content: &str) -> CodeLines {
        match self {
            TokenType::CodeOutsideQuotes => {
                let mut non_blank_lines = 0;
                let mut actual_first_line = -1;
                let mut count = 0;
                #[allow(clippy::explicit_counter_loop)]
                for curr in content.split('\n') {
                    if curr.trim() != "" {
                        non_blank_lines += 1;
                        if actual_first_line == -1 {
                            actual_first_line = count;
                        }
                    }
                    count += 1;
                }

                CodeLines {
                    info_lines: non_blank_lines,
                    actual_first_line,
                }
            }
            _ => CodeLines {
                info_lines: 0, // we don't care about these values
                actual_first_line: -1,
            },
        }
    }
}

impl TokenInsightGroup {
    pub fn total_contribution(&self) -> usize {
        let len = self.token_insights.len();
        let mut total = 0;

        for i in 0..len {
            let curr = &self.token_insights[i];
            if curr.code_lines.actual_first_line == -1 {
                continue;
            }
            total += curr.code_lines.info_lines;
            //todo! do not over count
            if i > 0 {
                let prev = &self.token_insights[i - 1];
                if curr.code_lines.actual_first_line == 0
                    && curr.start_line == prev.end_line
                    && prev.code_lines.actual_first_line != -1
                {
                    // starts in the same line, overcounted !
                    // println!(
                    //     "cutting for {} {} {} {} {}",
                    //     curr.start_line,
                    //     prev.end_line,
                    //     curr.code_lines.info_lines,
                    //     prev.code_lines.info_lines,
                    //     prev.code_lines.actual_first_line,
                    // );
                    total -= 1;
                }
            }
        }

        total
    }
}
