# markdown-table-rs

Creating markdown tables with Rust!


## Example

Code:
```rs
use markdown_table::MarkdownTable;

let table = MarkdownTable::new(
    vec![
        vec!["test".to_string()],
        vec!["1".to_string()],
        vec!["2".to_string()]
    ]
);

println!("{}", table);
```

String output:
```markdown
<table><tr><td>Testing<tr><td>1<tr><td>2</table>
```

Rendered:
<table><tr><td>Testing<tr><td>1<tr><td>2</table>
