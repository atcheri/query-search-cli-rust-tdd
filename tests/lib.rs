use core::str;

#[test]
fn should_return_a_single_line_when_matched_with_single_line() {
    // arrange
    let query = "TDD";
    let content = "TDD rocks";

    // act
    let lines = search_query(query, content);

    // assert
    assert_eq!(lines, vec![(1, content.to_string())])
}

#[test]
fn should_return_a_single_line_when_matched_with_another_line() {
    // arrange
    let query = "TDD";
    let content = "Programming gets better with TDD";

    // act
    let lines = search_query(query, content);

    // assert
    assert_eq!(lines, vec![(1, content.to_string())]);
}

#[test]
fn should_return_no_line_when_matched_with_none() {
    // arrange
    let query = "pizza";
    let content = "TDD rocks";

    // act
    let lines = search_query(query, content);

    // assert
    assert!(lines.is_empty());
}

#[test]
fn should_return_a_single_line_when_matched_with_second_line() {
    // arrange
    let query = "TDD";
    let content = "First line without the word\nTDD rocks";

    // act
    let lines = search_query(query, content);

    // assert
    assert_eq!(lines, vec![(2, "TDD rocks".to_string())]);
}

#[test]
fn should_return_multiple_lines_when_matched_multiple_times() {
    // arrange
    let query = "TDD";
    let content =
        "First line without the word\nTDD rocks\nClean code is a must\nIn TDD, one must starts writing a failing test";

    // act
    let lines = search_query(query, content);

    // assert
    assert_eq!(
        lines,
        vec![
            (2, "TDD rocks".to_string()),
            (4, "In TDD, one must starts writing a failing test".to_string())
        ]
    );
}

#[test]
fn should_return_a_single_line_when_matched_with_single_line_and_case_insensitive() {
    // arrange
    let query = "tdd";
    let content = "TDD rocks";

    // act
    let lines = search_query(query, content);

    // assert
    assert_eq!(lines, vec![(1, content.to_string())])
}

pub struct QuerySearcher {}

impl QuerySearcher {
    fn search(query: &str, content: &str) -> Vec<(usize, String)> {
        let mut matches: Vec<(usize, String)> = vec![];
        for (index, line) in content.lines().enumerate() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                matches.push((index + 1, line.to_string()));
            }
        }

        return matches;
    }
}

fn search_query(query: &str, content: &str) -> Vec<(usize, String)> {
    QuerySearcher::search(&query, &content)
}
