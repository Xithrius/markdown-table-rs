use std::{cmp, fmt::Display};

use anyhow::{bail, Result};

use pad::{Alignment, PadStr};

#[derive(Debug, Clone)]
pub enum HeadingAlignment {
    Left,
    Center,
    Right,
    Default,
}

impl HeadingAlignment {
    fn to_padded_string(&self, length: usize) -> String {
        match self {
            HeadingAlignment::Default => "---".pad(length, '-', Alignment::Left, false),
            HeadingAlignment::Left => ":--".pad(length, '-', Alignment::Left, false),
            HeadingAlignment::Center => {
                format!("{}:", ":-".pad(length - 1, '-', Alignment::Left, false))
            }
            HeadingAlignment::Right => "--:".pad(length, '-', Alignment::Right, false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Heading {
    label: String,
    alignment: HeadingAlignment,
}

impl Heading {
    pub fn new(label: String, alignment: Option<HeadingAlignment>) -> Self {
        Self {
            label,
            alignment: alignment.unwrap_or(HeadingAlignment::Default),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarkdownTable<T>
where
    T: ToString + Display,
{
    headings: Option<Vec<Heading>>,
    cells: Vec<Vec<T>>,
}

impl<T> MarkdownTable<T>
where
    T: ToString + Display,
{
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        Self {
            cells,
            headings: None,
        }
    }

    pub fn with_headings(&mut self, headings: Vec<Heading>) -> &Self {
        self.headings = Some(headings);
        self
    }

    pub fn as_markdown(&self) -> Result<String> {
        match &self.headings {
            Some(headings) => {
                let mut col_width: Vec<usize> = headings
                    .iter()
                    .map(|h| cmp::max(5, h.label.len() + 2))
                    .collect();

                for row in &self.cells {
                    for (col_index, value) in row.iter().enumerate() {
                        // TODO using tostring,  may be not good as mutliline string will break it
                        let value_len = value.to_string().len() + 2;
                        if value_len > col_width[col_index] {
                            col_width[col_index] = value_len;
                        }
                    }
                }
                let header_row = format!(
                    "|{}|",
                    headings
                        .iter()
                        .enumerate()
                        .map(|(i, h)| format!(" {} ", h.label).pad_to_width(col_width[i]))
                        .collect::<Vec<String>>()
                        .join("|")
                );
                let header_split_row = format!(
                    "|{}|",
                    headings
                        .iter()
                        .enumerate()
                        .map(|(i, h)| format!(
                            " {} ",
                            h.alignment.to_padded_string(col_width[i] - 2)
                        ))
                        .collect::<Vec<String>>()
                        .join("|")
                );
                let cells_row = self
                    .cells
                    .iter()
                    .map(|v| {
                        format!(
                            "|{}|",
                            v.iter()
                                .enumerate()
                                .map(|(i, v_inner)| format!(" {} ", v_inner)
                                    .pad_to_width(col_width[i]))
                                .collect::<Vec<String>>()
                                .join("|")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                Ok(format!(
                    "{}\n{}\n{}\n",
                    header_row, header_split_row, cells_row
                ))
            }
            None => {
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
    }
}

impl<T> ToString for MarkdownTable<T>
where
    T: ToString + Display,
{
    fn to_string(&self) -> String {
        self.as_markdown().unwrap()
    }
}
