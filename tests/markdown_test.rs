#[cfg(test)]
mod tests {
    use markdown_table::{utils::markdown::wrap_in_table, Table};

    fn setup() -> Table {
        Table::new(vec![
            vec!["Testing".to_string()],
            vec!["1".to_string()],
            vec!["2".to_string()],
        ])
    }

    #[test]
    fn test_simple_table_to_markdown() {
        let md: String = setup().as_markdown();

        assert_eq!(
            md,
            "<table><tr><td>Testing<tr><td>1<tr><td>2</table>".to_string()
        );
    }

    #[test]
    fn test_wrap_in_table_empty_string() {
        assert_eq!(
            wrap_in_table("Testing".to_string()),
            "<table>Testing</table>".to_string()
        );
    }
}
