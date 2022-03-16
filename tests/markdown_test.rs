#[cfg(test)]
mod tests {
    use anyhow::Result;

    use markdown_table::MarkdownTable;

    const TEST_TABLE: &str = "<table><tr><td>Testing<tr><td>1<tr><td>2</table>";

    fn setup() -> MarkdownTable {
        MarkdownTable::new(vec![
            vec!["Testing".to_string()],
            vec!["1".to_string()],
            vec!["2".to_string()],
        ])
    }

    fn setup_empty() -> MarkdownTable {
        MarkdownTable::new(vec![])
    }

    #[test]
    fn test_simple_table_to_markdown() {
        let md: String = setup().as_markdown().unwrap();

        assert_eq!(md, TEST_TABLE.to_string());
    }

    #[test]
    fn test_empty_constructor_error_out() {
        let md: Result<String> = setup_empty().as_markdown();

        if let Err(e) = md {
            assert_eq!(e.to_string(), "Table must have at least 1 row.".to_string());
        } else {
            panic!("Test should not reach this branch.")
        }
    }

    #[test]
    #[should_panic]
    fn test_empty_constructor_string_conversion() {
        setup_empty().to_string();
    }
}
