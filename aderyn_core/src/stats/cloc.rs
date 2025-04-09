use super::token::*;

pub fn count_code_lines(token_descriptors: &[TokenDescriptor]) -> usize {
    let mut code_lines = 0;

    let insights =
        token_descriptors.iter().map(|tok_dsc| tok_dsc.into()).collect::<Vec<TokenInsight>>();

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

    let groups = token_insight_groups
        .iter()
        .filter(|g| g.token_type == HighLevelType::Code)
        .collect::<Vec<_>>();

    if groups.is_empty() {
        return 0;
    }

    let len = groups.len();
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
            && prev.last_token_insight_has_code_in_its_last_line()
        {
            // println!("deducting {} {}", curr.start_line, prev.end_line);
            code_lines -= 1;
        }
        prev = curr;
    }

    code_lines
}

#[cfg(test)]
mod cloc_tests {
    use crate::stats::token::tokenize;

    use super::*;

    #[test]
    fn test_print_cloc_heavily_commented_contract() {
        let content = tokenize(include_str!(
            "../../../tests/contract-playground/src/cloc/HeavilyCommentedContract.sol"
        ));
        let stats = count_code_lines(&content);
        assert_eq!(stats, 21);
    }

    #[test]
    fn test_print_cloc_another_heavily_commented_contract() {
        let content = tokenize(include_str!(
            "../../../tests/contract-playground/src/cloc/AnotherHeavilyCommentedContract.sol"
        ));
        let stats = count_code_lines(&content);
        assert_eq!(stats, 32);
    }
}
