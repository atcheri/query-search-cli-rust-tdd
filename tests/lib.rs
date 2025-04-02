use core::str;

use assert_cmd::assert;

#[test]
fn should_return_a_single_line_when_matched_with_single_line() {
    // arrange
    let query = "TDD";
    let content = "TDD rocks";

    // act
    let lines = QuerySearcher::search(&query, &content);

    // assert
    assert_eq!(lines, content)
}

#[test]
fn should_return_a_single_line_when_matched_with_another_line() {
    // arrange
    let query = "TDD";
    let content = "Programming gets better with TDD";

    // act
    let lines = QuerySearcher::search(&query, &content);

    // assert
    assert_eq!(lines, content);
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

pub struct QuerySearcher {}

impl QuerySearcher {
    fn search(query: &str, content: &str) -> String {
        String::from(content)
    }
}
