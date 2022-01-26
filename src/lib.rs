pub mod utils;

use crate::utils::markdown::wrap_in_table;

#[derive(Debug)]
pub struct Table {
    cells: Vec<Vec<String>>,
}

impl Table {
    pub fn new(cells: Vec<Vec<String>>) -> Self {
        Table { cells }
    }

    pub fn as_markdown(&self) -> String {
        wrap_in_table(
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
        )
    }

    pub fn append_column(&mut self) {
        todo!()
    }

    pub fn prepend_column() {
        todo!()
    }

    pub fn insert_column() {
        todo!()
    }

    pub fn pop_column() {
        todo!()
    }

    pub fn append_row() {
        todo!()
    }

    pub fn prepend_row() {
        todo!()
    }

    pub fn insert_row() {
        todo!()
    }

    pub fn pop_row() {
        todo!()
    }

    pub fn remove_cell() {
        todo!()
    }
}
