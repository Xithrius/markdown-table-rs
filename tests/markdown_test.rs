#[cfg(test)]
mod tests {
    use anyhow::Result;

    use markdown_table::MarkdownTable;

    const TEST_TABLE: &str = "<table><tr><td>0<tr><td>1<tr><td>2</table>";

    #[test]
    fn test_simple_table_to_markdown_with_strings() {
        let md: String = MarkdownTable::new(vec![vec!["0"], vec!["1"], vec!["2"]])
            .as_markdown()
            .unwrap();

        assert_eq!(md, TEST_TABLE.to_string());
    }

    #[test]
    fn test_simple_table_to_markdown_with_integers() {
        let md: String = MarkdownTable::new(vec![vec![0], vec![1], vec![2]])
            .as_markdown()
            .unwrap();

        assert_eq!(md, TEST_TABLE.to_string());
    }

    #[test]
    fn test_empty_constructor_error_out() {
        let md: Result<String> = MarkdownTable::<String>::new(vec![]).as_markdown();

        if let Err(e) = md {
            assert_eq!(e.to_string(), "Table must have at least 1 row.".to_string());
        } else {
            panic!("Test should not reach this branch.")
        }
    }
}
