#[cfg(test)]
mod tests {
    use markdown_table::*;

    fn setup() -> Table {
        Table::new(vec![
            vec!["Testing".to_string()],
            vec!["1".to_string()],
            vec!["2".to_string()],
        ])
    }

    #[test]
    fn test_hello_world() {
        let md: String = setup().as_markdown();

        assert_eq!(
            md,
            "<table><tr><td>Testing<tr><td>1<tr><td>2</table>".to_string()
        );
    }
}
