use core::str;

#[test]
fn should_return_a_single_line_when_matched_with_single_line() {
    let query = "TDD";
    let content = "TDD rocks";

    let lines = QuerySearcher::search(&query, &content);
}

pub struct QuerySearcher {}

impl QuerySearcher {
    fn search(query: &str, content: &str) -> String {
        String::from("")
    }
}
