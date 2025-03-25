use super::{insight::TokenInsight, token::TokenDescriptor};
use crate::fscloc::token::{tokenize, TokenType};
use lazy_regex::*;

#[derive(Debug)]
pub struct Stats {
    pub code: usize,
    pub ignore_lines: Vec<IgnoreLine>,
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
    fn last_token_insight_has_code_in_its_last_line(&self) -> bool {
        self.token_insights.last().is_some_and(|insight| insight.code_lines.last_line_has_code)
    }
}

pub fn get_stats(r_content: &str, skip_cloc: bool) -> Stats {
    if r_content.is_empty() {
        return Stats { code: 0, ignore_lines: vec![] };
    }

    let token_descriptors = tokenize(r_content);
    let mut code_lines = 0;

    if !skip_cloc {
        let mut content = String::new();

        let insights = token_descriptors
            .iter()
            .inspect(|x| {
                content.push_str(&x.content); // will be used to verify if original content is
                                              // preserved
            })
            .map(|tok_dsc| tok_dsc.into())
            .collect::<Vec<TokenInsight>>();

        let mut token_insight_groups = vec![];

        for insight in &insights {
            if token_insight_groups.is_empty() {
                let new_token_insight_group = TokenInsightGroup {
                    token_insights: vec![insight.clone()],
                    start_line: insight.start_line,
                    end_line: insight.end_line,
                    token_type: insight.token_type.clone().into(),
                };
                token_insight_groups.push(new_token_insight_group);
                continue;
            }
            let prev_group = token_insight_groups.last_mut().unwrap();

            if insight.start_line == prev_group.end_line
                && insight.code_lines.actual_first_line == 0
                && prev_group.last_token_insight_has_code_in_its_last_line()
                && prev_group.token_type == insight.token_type.clone().into()
            {
                prev_group.token_insights.push(insight.clone());
                prev_group.end_line = insight.end_line;
            } else {
                let new_token_insight_group = TokenInsightGroup {
                    token_insights: vec![insight.clone()],
                    start_line: insight.start_line,
                    end_line: insight.end_line,
                    token_type: insight.token_type.clone().into(),
                };
                token_insight_groups.push(new_token_insight_group);
            }
        }

        // raw content after tokenizing and joining should give us back the exact same text
        assert!(content == r_content);

        // debug
        // for group in &token_insight_groups {
        //     if group.token_type == HighLevelType::Code {
        //         println!(
        //             "{}-{} Size:{} Contrib:{}",
        //             group.start_line,
        //             group.end_line,
        //             group.token_insights.len(),
        //             group.total_contribution(),
        //         );
        //         if group.total_contribution() == 37 {
        //             for i in &group.token_insights {
        //                 println!(
        //                     "{:?} - {:?} - {} - {} - {}",
        //                     i.token_type,
        //                     i.code_lines.info_lines,
        //                     i.starts_with_newline,
        //                     i.start_line,
        //                     i.end_line
        //                 );
        //             }
        //         }
        //     }
        // }

        let groups = token_insight_groups
            .iter()
            .filter(|g| g.token_type == HighLevelType::Code)
            .collect::<Vec<_>>();

        let len = groups.len();

        if len == 0 {
            return Stats { code: 0, ignore_lines: vec![] };
        }

        let mut prev = &groups[0];
        code_lines += prev.total_contribution();

        // println!("LEN {}", len);

        #[allow(clippy::needless_range_loop)]
        for i in 1..len {
            let curr = &groups[i];
            let grp_contrib = curr.total_contribution();
            code_lines += grp_contrib;

            // what line does the first contributing token start ?
            if curr.start_line == prev.end_line
                && (curr.token_insights[0].code_lines.actual_first_line == 0)
                && grp_contrib >= 1
                && prev.last_token_insight_has_code_in_its_last_line()
            {
                // println!("deducting {} {}", curr.start_line, prev.end_line);
                code_lines -= 1;
            }
            prev = curr;
        }
    }

    let ignore_lines = get_lines_to_ignore(&token_descriptors);

    Stats { code: code_lines, ignore_lines }
}

fn get_lines_to_ignore(token_descriptors: &Vec<TokenDescriptor>) -> Vec<IgnoreLine> {
    let mut ignore_lines = vec![];
    for token in token_descriptors {
        if matches!(
            token.token_type,
            TokenType::CodeSingleQuotes
                | TokenType::CodeDoubleQuotes
                | TokenType::CodeOutsideQuotes
        ) {
            continue;
        }

        let should_ignore_next_line = |content: &str| -> bool {
            content.contains("aderyn-ignore-next-line") || content.contains("aderyn-fp-next-line")
        };

        let should_ignore_line = |content: &str| -> bool {
            content.contains("aderyn-ignore") || content.contains("aderyn-fp")
        };

        #[allow(clippy::never_loop)]
        if let Some(ignore_line) = loop {
            // Check if we have a specific set of detectors only, for which we want to ignore.
            if let Some(captures) = ADERYN_IGNORE_REGEX.captures(&token.content) {
                let line_number = {
                    if captures.get(1).is_none() {
                        token.end_line
                    } else {
                        token.end_line + 1
                    }
                };
                let detector_names = captures
                    .get(2)
                    .map(|m| m.as_str())
                    .map(|names| {
                        names
                            .split(',')
                            .map(|name| name.trim().to_string())
                            .filter(|name| !name.is_empty())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                break Some(IgnoreLine {
                    when: When::ForDetectorsWithNames(detector_names),
                    which: line_number,
                });
            } else if should_ignore_next_line(&token.content) {
                break Some(IgnoreLine { when: When::Always, which: token.end_line + 1 });
            } else if should_ignore_line(&token.content) {
                break Some(IgnoreLine { when: When::Always, which: token.end_line });
            }

            break None;
        } {
            ignore_lines.push(ignore_line);
        }
    }
    ignore_lines
}

static ADERYN_IGNORE_REGEX: Lazy<Regex> =
    lazy_regex!(r"aderyn-(?:ignore|fp)(\-next\-line)?\s*\((\s*[a-zA-Z\-\s,]*)\)");

#[derive(Debug, Clone)]
pub enum When {
    Always,
    ForDetectorsWithNames(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct IgnoreLine {
    /// When to consider this ignore
    pub when: When,

    /// Which line number to ignore
    pub which: usize,
}

#[cfg(test)]
mod parse_comments_to_rightfully_ignore_lines {

    use super::*;

    #[test]
    fn test_aderyn_ignore_specific_detectors() {
        let negative_examples = [
            r#"""
                // aderyn-ignore solhint-disable
            """#,
            r#"""
                // solhint-disable aderyn-ignore
            """#,
            r#""" aderyn-ignore solhint-disable
            """#,
            r#""" solhint-disable aderyn-ignore
            """#,
            "aderyn-ignore",
            "aderyn-ignore ",
        ];

        let positive_examples = [
            r#"""
                aderyn-ignore ( name-a, name-b,name-c) solhint-disable 
            """#,
            r#"""
                aderyn-ignore ( name-a, name-b,name-c ) 
            """#,
            r#"""
                aderyn-ignore(name-c)
            """#,
            r#"""
                aderyn-ignore()
            """#,
            r#"""
                aderyn-ignore(  )
            """#,
            r#"""
                aderyn-ignore ( name-a, name-b,
                name-c ) 
            """#,
            r#"""
                aderyn-ignore-next-line ( name-a, name-b,
                name-c ) 
            """#,
        ];

        let positive_example_1 = ADERYN_IGNORE_REGEX.captures(positive_examples[0]).unwrap();
        assert!(positive_example_1.get(1).is_none());
        assert!(positive_example_1.get(2).unwrap().as_str() == " name-a, name-b,name-c");

        let positive_example_2 = ADERYN_IGNORE_REGEX.captures(positive_examples[1]).unwrap();
        assert!(positive_example_2.get(1).is_none());
        assert!(positive_example_2.get(2).unwrap().as_str() == " name-a, name-b,name-c ");

        let positive_example_3 = ADERYN_IGNORE_REGEX.captures(positive_examples[2]).unwrap();
        assert!(positive_example_3.get(1).is_none());
        assert!(positive_example_3.get(2).unwrap().as_str() == "name-c");

        let positive_example_4 = ADERYN_IGNORE_REGEX.captures(positive_examples[3]).unwrap();
        assert!(positive_example_4.get(1).is_none());
        assert!(positive_example_4.get(2).unwrap().as_str() == "");

        let positive_example_5 = ADERYN_IGNORE_REGEX.captures(positive_examples[4]).unwrap();
        assert!(positive_example_5.get(1).is_none());
        assert!(positive_example_5.get(2).unwrap().as_str() == "  ");

        let positive_example_6 = ADERYN_IGNORE_REGEX.captures(positive_examples[5]).unwrap();
        assert!(positive_example_6.get(1).is_none());
        assert!(
            positive_example_6.get(2).unwrap().as_str()
                == " name-a, name-b,
                name-c "
        );

        let positive_example_7 = ADERYN_IGNORE_REGEX.captures(positive_examples[6]).unwrap();
        assert!(positive_example_7.get(1).is_some());
        assert!(
            positive_example_6.get(2).unwrap().as_str()
                == " name-a, name-b,
                name-c "
        );

        let false_positive_examples = [
            r#"""
                aderyn-fp ( name-a, name-b,name-c) solhint-disable 
            """#,
            r#"""
                aderyn-fp ( name-a, name-b,name-c ) 
            """#,
            r#"""
                aderyn-fp(name-c)
            """#,
            r#"""
                aderyn-fp()
            """#,
            r#"""
                aderyn-fp(  )
            """#,
            r#"""
                aderyn-fp ( name-a, name-b,
                name-c ) 
            """#,
            r#"""
                aderyn-fp-next-line ( name-a, name-b,
                name-c ) 
            """#,
        ];

        let positive_example_1 = ADERYN_IGNORE_REGEX.captures(false_positive_examples[0]).unwrap();
        assert!(positive_example_1.get(1).is_none());
        assert!(positive_example_1.get(2).unwrap().as_str() == " name-a, name-b,name-c");

        let positive_example_2 = ADERYN_IGNORE_REGEX.captures(false_positive_examples[1]).unwrap();
        assert!(positive_example_2.get(1).is_none());
        assert!(positive_example_2.get(2).unwrap().as_str() == " name-a, name-b,name-c ");

        let positive_example_3 = ADERYN_IGNORE_REGEX.captures(false_positive_examples[2]).unwrap();
        assert!(positive_example_3.get(1).is_none());
        assert!(positive_example_3.get(2).unwrap().as_str() == "name-c");

        let positive_example_4 = ADERYN_IGNORE_REGEX.captures(false_positive_examples[3]).unwrap();
        assert!(positive_example_4.get(1).is_none());
        assert!(positive_example_4.get(2).unwrap().as_str() == "");

        let positive_example_5 = ADERYN_IGNORE_REGEX.captures(false_positive_examples[4]).unwrap();
        assert!(positive_example_5.get(1).is_none());
        assert!(positive_example_5.get(2).unwrap().as_str() == "  ");

        let positive_example_6 = ADERYN_IGNORE_REGEX.captures(false_positive_examples[5]).unwrap();
        assert!(positive_example_6.get(1).is_none());
        assert!(
            positive_example_6.get(2).unwrap().as_str()
                == " name-a, name-b,
                name-c "
        );

        let positive_example_7 = ADERYN_IGNORE_REGEX.captures(false_positive_examples[6]).unwrap();
        assert!(positive_example_7.get(1).is_some());
        assert!(
            positive_example_6.get(2).unwrap().as_str()
                == " name-a, name-b,
                name-c "
        );

        for e in negative_examples {
            assert!(ADERYN_IGNORE_REGEX.captures(e).is_none());
        }
    }
}
