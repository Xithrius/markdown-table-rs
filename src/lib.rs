use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct MarkdownTable {
    cells: Vec<Vec<String>>,
}

impl MarkdownTable {
    pub fn new(cells: Vec<Vec<String>>) -> Self {
        MarkdownTable { cells }
    }

    pub fn as_markdown(&self) -> Result<String> {
        if !self.cells.is_empty() {
            Ok(format!(
                "<table>{}</table>",
                self.cells
                    .iter()
                    .map(|v| {
                        format!(
                            "<tr>{}",
                            v.iter()
                                .map(|v_inner| format!("<td>{}", v_inner))
                                .collect::<Vec<String>>()
                                .join("")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(""),
            ))
        } else {
            bail!("Table must have at least 1 row.".to_string())
        }
    }
}

impl ToString for MarkdownTable {
    fn to_string(&self) -> String {
        self.as_markdown().unwrap()
    }
}
