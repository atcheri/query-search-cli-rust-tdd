use core::str;

use domain::{ FileReader, QuerySearcher };

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

struct FileReaderMock {
    lines: String,
}

impl FileReader for FileReaderMock {
    fn read_to_string(&self, _file_path: String) -> Result<String, String> {
        Ok(self.lines.clone())
    }
}

fn search_query(query: &str, content: &str) -> Vec<(usize, String)> {
    let file_reader_mock = FileReaderMock {
        lines: content.to_string(),
    };

    QuerySearcher::new(file_reader_mock).search(&query, &content).unwrap()
}
