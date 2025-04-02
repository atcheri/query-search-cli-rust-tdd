use core::str;

#[test]
fn should_return_a_single_line_when_matched_with_single_line() {
    // arrange
    let query = "TDD";
    let content = "TDD rocks";

    // act
    let lines = QuerySearcher::search(&query, &content);

    // assert
    assert_eq!(lines, vec![(1, content)])
}

#[test]
fn should_return_a_single_line_when_matched_with_another_line() {
    // arrange
    let query = "TDD";
    let content = "Programming gets better with TDD";

    // act
    let lines = QuerySearcher::search(&query, &content);

    // assert
    assert_eq!(lines, vec![(1, content)]);
}

#[test]
fn should_return_no_line_when_matched_with_none() {
    // arrange
    let query = "pizza";
    let content = "TDD rocks";

    // act
    let lines = QuerySearcher::search(&query, &content);

    // assert
    assert!(lines.is_empty());
}

#[test]
fn should_return_a_single_line_when_matched_with_second_line() {
    // arrange
    let query = "TDD";
    let content = "First line without the word\nTDD rocks";

    // act
    let lines = QuerySearcher::search(&query, &content);

    // assert
    assert_eq!(lines, vec![(2, "TDD rocks")]);
}

#[test]
fn should_return_multiple_lines_when_matched_multiple_times() {
    // arrange
    let query = "TDD";
    let content =
        "First line without the word\nTDD rocks\nClean code is a must\nIn TDD, one must starts writing a failing test";

    // act
    let lines = QuerySearcher::search(&query, &content);

    // assert
    assert_eq!(
        lines,
        vec![(2, "TDD rocks"), (4, "In TDD, one must starts writing a failing test")]
    );
}

#[test]
fn should_return_a_single_line_when_matched_with_single_line_and_case_insensitive() {
    // arrange
    let query = "tdd";
    let content = "TDD rocks";

    // act
    let lines = QuerySearcher::search(&query, &content);

    // assert
    assert_eq!(lines, vec![(1, content)])
}

pub struct QuerySearcher {}

impl QuerySearcher {
    fn search(query: &str, content: &str) -> Vec<String> {
        let mut matches = vec![];
        for line in content.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                matches.push(line.to_string());
            }
        }

        return matches;
    }
}
