#[cfg(test)]
mod tests {
    use anyhow::Result;

    use markdown_table::{Heading, MarkdownTable};

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

    #[test]
    fn test_table_with_small_headings() {
        let md: String = MarkdownTable::new(vec![vec!["1", "2", "3", "4"]])
            .with_headings(vec![
                Heading::new("1".to_string(), None),
                Heading::new(
                    "2".to_string(),
                    Some(markdown_table::HeadingAlignment::Left),
                ),
                Heading::new(
                    "3".to_string(),
                    Some(markdown_table::HeadingAlignment::Right),
                ),
                Heading::new(
                    "4".to_string(),
                    Some(markdown_table::HeadingAlignment::Center),
                ),
            ])
            .as_markdown()
            .unwrap();

        let expected: &str = r#"| 1   | 2   | 3   | 4   |
| --- | :-- | --: | :-: |
| 1   | 2   | 3   | 4   |
"#;

        assert_eq!(md, expected.to_string());
    }

    #[test]
    fn test_table_with_headings() {
        let md: String = MarkdownTable::new(vec![vec!["Val 1", "Val 2", "Val 3", "Val 4"]])
            .with_headings(vec![
                Heading::new("Default Header".to_string(), None),
                Heading::new(
                    "Left Align".to_string(),
                    Some(markdown_table::HeadingAlignment::Left),
                ),
                Heading::new(
                    "Right Align".to_string(),
                    Some(markdown_table::HeadingAlignment::Right),
                ),
                Heading::new(
                    "Center Align".to_string(),
                    Some(markdown_table::HeadingAlignment::Center),
                ),
            ])
            .as_markdown()
            .unwrap();

        let expected: &str = r#"| Default Header | Left Align | Right Align | Center Align |
| -------------- | :--------- | ----------: | :----------: |
| Val 1          | Val 2      | Val 3       | Val 4        |
"#;

        assert_eq!(md, expected.to_string());
    }


    #[test]
    fn test_table_with_long_values() {
        let md: String = MarkdownTable::new(vec![vec!["value bigger than header 1", "value bigger than header 2", "value bigger than header 3", "value bigger than header 4"]])
            .with_headings(vec![
                Heading::new("Default Header".to_string(), None),
                Heading::new(
                    "Left Align".to_string(),
                    Some(markdown_table::HeadingAlignment::Left),
                ),
                Heading::new(
                    "Right Align".to_string(),
                    Some(markdown_table::HeadingAlignment::Right),
                ),
                Heading::new(
                    "Center Align".to_string(),
                    Some(markdown_table::HeadingAlignment::Center),
                ),
            ])
            .as_markdown()
            .unwrap();

        let expected: &str = r#"| Default Header             | Left Align                 | Right Align                | Center Align               |
| -------------------------- | :------------------------- | -------------------------: | :------------------------: |
| value bigger than header 1 | value bigger than header 2 | value bigger than header 3 | value bigger than header 4 |
"#;

        assert_eq!(md, expected.to_string());
    }
}
