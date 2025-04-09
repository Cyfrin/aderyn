use super::{
    token::{TokenDescriptor, TokenType},
    IgnoreLine, When,
};
use lazy_regex::*;

pub fn get_lines_to_ignore(token_descriptors: &Vec<TokenDescriptor>) -> Vec<IgnoreLine> {
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

        let mut found = false;

        // Check if we have a specific set of detectors only, for which we want to ignore.
        for capture in ADERYN_IGNORE_REGEX.captures_iter(&token.content) {
            let line_number = {
                if capture.get(1).is_none() {
                    token.end_line
                } else {
                    token.end_line + 1
                }
            };
            let detector_names = capture
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
            ignore_lines.push(IgnoreLine {
                when: When::ForDetectorsWithNames(detector_names.clone()),
                which: line_number,
            });
            found = true;
        }
        if !found {
            if should_ignore_next_line(&token.content) {
                ignore_lines.push(IgnoreLine { when: When::Always, which: token.end_line + 1 });
            } else if should_ignore_line(&token.content) {
                ignore_lines.push(IgnoreLine { when: When::Always, which: token.end_line });
            }
        }
    }

    ignore_lines
}

static ADERYN_IGNORE_REGEX: Lazy<Regex> =
    lazy_regex!(r"aderyn-(?:ignore|fp)(\-next\-line)?\s*\((\s*[a-zA-Z\-\s,]*)\)");

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
