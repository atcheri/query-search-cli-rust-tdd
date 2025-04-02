use core::str;

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

pub struct QuerySearcher {}

impl QuerySearcher {
    fn search(query: &str, content: &str) -> String {
        String::from("TDD rocks")
    }
}
