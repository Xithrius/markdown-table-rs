#[derive(Debug)]
pub struct Table {
    boxes: Vec<Vec<String>>,
}

impl Table {
    pub fn new(boxes: Vec<Vec<String>>) -> Self {
        Table { boxes }
    }

    pub fn as_markdown(&self) -> String {
        wrap_in_table(
            self.boxes
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
}

pub fn wrap_in_table(s: String) -> String {
    format!("<table>{s}</table>")
}
