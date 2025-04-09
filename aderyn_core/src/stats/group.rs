use super::token::*;

#[derive(Debug, Clone)]
pub struct CodeLines {
    pub info_lines: usize,
    pub actual_first_line: isize,
    pub last_line_has_code: bool,
}

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
                    last_line_has_code: true,
                },
                _ => value.token_type.number_of_lines(&value.content),
            },
            start_line: value.start_line,
            end_line: value.end_line,
            token_type: value.token_type.clone(),
            starts_with_newline: value.content.starts_with('\n'),
            ends_with_newline: value.content.ends_with('\n'),
        };

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
                let mut last_line_has_code = false;
                #[allow(clippy::explicit_counter_loop)]
                for curr in content.split('\n') {
                    if curr.trim() != "" {
                        non_blank_lines += 1;
                        if actual_first_line == -1 {
                            actual_first_line = count;
                        }
                        last_line_has_code = true;
                    } else {
                        last_line_has_code = false;
                    }
                    count += 1;
                }

                CodeLines { info_lines: non_blank_lines, actual_first_line, last_line_has_code }
            }
            _ => CodeLines {
                info_lines: 0, // we don't care about these values
                actual_first_line: -1,
                last_line_has_code: true,
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
                    total -= 1;
                }
            }
        }

        total
    }
}

#[derive(Debug, PartialEq)]
pub enum HighLevelType {
    Code,
    NotCode,
}

impl From<TokenType> for HighLevelType {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::CodeDoubleQuotes
            | TokenType::CodeOutsideQuotes
            | TokenType::CodeSingleQuotes => Self::Code,
            TokenType::MultilineComment | TokenType::SinglelineComment => Self::NotCode,
        }
    }
}

#[derive(Debug)]
pub struct TokenInsightGroup {
    pub token_insights: Vec<TokenInsight>,
    pub start_line: usize,
    pub end_line: usize,
    pub token_type: HighLevelType,
}

impl TokenInsightGroup {
    pub fn last_token_insight_has_code_in_its_last_line(&self) -> bool {
        self.token_insights.last().is_some_and(|insight| insight.code_lines.last_line_has_code)
    }
}
