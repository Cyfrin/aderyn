use crate::fscloc::token::{tokenize, TokenType};

use super::insight::TokenInsight;

#[derive(Debug)]
pub struct Stats {
    pub code: usize,
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

pub fn get_stats(r_content: &str) -> Stats {
    if r_content.is_empty() {
        return Stats { code: 0 };
    }

    let token_descriptors = tokenize(r_content);
    let mut content = String::new();

    let insights = token_descriptors
        .iter()
        .map(|x| {
            content.push_str(&x.content); // will be used to verify if original content is preserved
            x
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
    // println!(
    //     "{}-{} Size:{} Contrib:{}",
    //     group.start_line,
    //     group.end_line,
    //     group.token_insights.len(),
    //     group.total_contribution(),
    // );
    // if group.token_insights.len() == 7 {
    //     for i in &group.token_insights {
    //         println!(
    //             "{:?} - {:?} - {} - {} - {}",
    //             i.token_type,
    //             i.code_lines.info_lines,
    //             i.starts_with_newline,
    //             i.start_line,
    //             i.end_line
    //         );
    //     }
    // }
    //     }
    // }

    let mut code_lines = 0;

    let groups = token_insight_groups
        .iter()
        .filter(|g| g.token_type == HighLevelType::Code)
        .collect::<Vec<_>>();

    let len = groups.len();

    if len == 0 {
        return Stats { code: 0 };
    }

    let mut prev = &groups[0];
    code_lines += prev.total_contribution();

    #[allow(clippy::needless_range_loop)]
    for i in 1..len {
        let curr = &groups[i];
        let grp_contrib = curr.total_contribution();
        code_lines += grp_contrib;

        // what line does the first contributing token start ?
        if curr.start_line == prev.end_line
            && (curr.token_insights[0].code_lines.actual_first_line == 0)
            && grp_contrib >= 1
        {
            // println!("deducting {} {}", curr.start_line, prev.end_line);
            code_lines -= 1;
        }
        prev = curr;
    }

    Stats { code: code_lines }
}
