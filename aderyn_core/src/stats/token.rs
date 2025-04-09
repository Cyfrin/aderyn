use crate::stats::util::CodeIterator;

#[derive(Debug)]
pub struct TokenDescriptor {
    pub token_type: TokenType,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    CodeSingleQuotes,
    CodeDoubleQuotes,
    CodeOutsideQuotes,
    MultilineComment,
    SinglelineComment,
}

pub fn tokenize(content: &str) -> Vec<TokenDescriptor> {
    let mut iter: CodeIterator = content.parse().unwrap();

    let mut multiline_comment_bytes = vec![];
    let mut singleline_comment_bytes = vec![];
    let mut code_bytes_inside_single_quotes = vec![];
    let mut code_bytes_inside_double_quotes = vec![];
    let mut code_bytes_outside_quotes = vec![];

    let mut token_descriptors = vec![];

    #[allow(clippy::while_let_on_iterator)]
    while let Some(token) = iter.next() {
        // Track on which line no the token ends.
        // (Start by assuming it ends on the same line until we finally hit the end)
        let mut end = token.line_no;

        if token.curr == '/' && token.next.is_some_and(|c| c == '*') {
            multiline_comment_bytes.push('/');
            multiline_comment_bytes.push('*');
            iter.next();
            while let Some(next) = iter.next() {
                if next.curr == '*' && next.next.is_some_and(|c| c == '/') {
                    let multiline_end_line = iter.next().unwrap().line_no;
                    end = std::cmp::max(end, multiline_end_line);
                    multiline_comment_bytes.push('*');
                    multiline_comment_bytes.push('/');
                    break;
                }
                multiline_comment_bytes.push(next.curr);
            }
        } else if token.curr == '"' {
            code_bytes_inside_double_quotes.push('"');
            while let Some(next) = iter.next() {
                if next.curr == '"' {
                    code_bytes_inside_double_quotes.push('"');
                    end = std::cmp::max(end, next.line_no);
                    break;
                }
                code_bytes_inside_double_quotes.push(next.curr);
                if next.curr == '\\' && next.next.is_some() {
                    // escape whatever comes next
                    iter.next();
                    code_bytes_inside_double_quotes.push(next.next.unwrap());
                }
            }
        } else if token.curr == '\'' {
            code_bytes_inside_single_quotes.push('\'');
            while let Some(next) = iter.next() {
                if next.curr == '\'' {
                    code_bytes_inside_single_quotes.push('\'');
                    end = std::cmp::max(end, next.line_no);
                    break;
                }
                code_bytes_inside_single_quotes.push(next.curr);
                if next.curr == '\\' && next.next.is_some() {
                    // escape whatever comes next
                    iter.next();
                    code_bytes_inside_single_quotes.push(next.next.unwrap());
                }
            }
        } else if token.curr == '/' && token.next.is_some_and(|c| c == '/') {
            singleline_comment_bytes.push('/');
            singleline_comment_bytes.push('/');
            iter.next();
            while let Some(next) = iter.next() {
                singleline_comment_bytes.push(next.curr);
                if next.curr == '\n' {
                    break;
                }
            }
        } else {
            // keep going until the next char is one of the above tokens
            code_bytes_outside_quotes.push(token.curr);
            // let staw_away = ['\'', '"', '/'];

            let staw_away = ['\'', '"', '/'];

            let mut curr = token.next;

            while let Some(next) = curr {
                if staw_away.iter().any(|x| x == &next) {
                    break;
                }
                let next_token = iter.next().unwrap();
                end = std::cmp::max(end, next_token.line_no);
                code_bytes_outside_quotes.push(next_token.curr);
                curr = next_token.next;
            }
        }

        if !multiline_comment_bytes.is_empty() {
            token_descriptors.push(TokenDescriptor {
                token_type: TokenType::MultilineComment,
                start_line: token.line_no,
                end_line: end,
                content: multiline_comment_bytes.iter().collect(),
            });
        }

        if !singleline_comment_bytes.is_empty() {
            token_descriptors.push(TokenDescriptor {
                token_type: TokenType::SinglelineComment,
                start_line: token.line_no,
                end_line: end,
                content: singleline_comment_bytes.iter().collect(),
            });
        }

        if !code_bytes_inside_double_quotes.is_empty() {
            token_descriptors.push(TokenDescriptor {
                token_type: TokenType::CodeDoubleQuotes,
                start_line: token.line_no,
                end_line: end,
                content: code_bytes_inside_double_quotes.iter().collect(),
            });
        }

        if !code_bytes_inside_single_quotes.is_empty() {
            token_descriptors.push(TokenDescriptor {
                token_type: TokenType::CodeSingleQuotes,
                start_line: token.line_no,
                end_line: end,
                content: code_bytes_inside_single_quotes.iter().collect(),
            });
        }

        if !code_bytes_outside_quotes.is_empty() {
            token_descriptors.push(TokenDescriptor {
                token_type: TokenType::CodeOutsideQuotes,
                start_line: token.line_no,
                end_line: end,
                content: code_bytes_outside_quotes.iter().collect(),
            });
        }

        multiline_comment_bytes.clear();
        singleline_comment_bytes.clear();
        code_bytes_inside_single_quotes.clear();
        code_bytes_inside_double_quotes.clear();
        code_bytes_outside_quotes.clear();
    }
    token_descriptors
}

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
