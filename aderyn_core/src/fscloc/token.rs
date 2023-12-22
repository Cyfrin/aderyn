use crate::fscloc::util::CodeIterator;

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
