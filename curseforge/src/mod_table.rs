use crate::models::Mod;

pub enum ModRow {
    Header {
        id: String,
        name: String,
        summary: String,
        download_count: String,
    },
    Data {
        id: u32,
        name: String,
        summary: String,
        download_count: u32,
    },
}

impl ModRow {
    pub fn new_header(id: impl Into<String>, name: impl Into<String>, summary: impl Into<String>, download_count: impl Into<String>) -> Self {
        Self::Header {
            id: id.into(),
            name: name.into(),
            summary: summary.into(),
            download_count: download_count.into(),
        }
    }

    pub fn new_data(id: u32, name: impl Into<String>, summary: impl Into<String>, download_count: u32) -> Self {
        Self::Data {
            id,
            name: name.into(),
            summary: summary.into(),
            download_count,
        }
    }

    pub fn format_row(&self) -> String {
        match self {
            ModRow::Header { id, name, summary, download_count } => {
                format!("| {:<12} | {:<30} | {:<50} | {:<14} |", id, name, summary, download_count)
            }
            ModRow::Data { id, name, summary, download_count } => {
                let name_rows_words = name.split_whitespace();
                let summary_rows_words = summary.split_whitespace();

                let mut name_rows : Vec<String> = Vec::new();
                let mut summary_rows : Vec<String> = Vec::new();
                let mut line_rows : Vec<String> = Vec::new();

                name_rows.push(String::new());
                for word in name_rows_words {
                    if name_rows.last().unwrap().len() + word.len() + 1 > 30 {
                        name_rows.push(word.to_string());
                    } else {
                        if !name_rows.last().unwrap().is_empty() {
                            name_rows.last_mut().unwrap().push(' ');
                        }
                        name_rows.last_mut().unwrap().push_str(word);
                    }
                }

                summary_rows.push(String::new());
                for word in summary_rows_words {
                    if summary_rows.last().unwrap().len() + word.len() + 1 > 50 {
                        summary_rows.push(word.to_string());
                    } else {
                        if !summary_rows.last().unwrap().is_empty() {
                            summary_rows.last_mut().unwrap().push(' ');
                        }
                        summary_rows.last_mut().unwrap().push_str(word);
                    }
                }

                let num_rows = std::cmp::max(name_rows.len(), summary_rows.len());
                for i in 0..num_rows {
                    let name = if i < name_rows.len() { &name_rows[i] } else { "" };
                    let summary = if i < summary_rows.len() { &summary_rows[i] } else { "" };
                    if i == 0 {
                        line_rows.push(format!("| {:<12} | {:<30} | {:<50} | {:<14} |", id, name, summary, download_count));
                    } else {
                        line_rows.push(format!("| {:<12} | {:<30} | {:<50} | {:<14} |", "", name, summary, ""));
                    }
                }
                line_rows.join("\n")
            }
        }
    }
}

pub struct ModTable {
    rows: Vec<ModRow>,
}

impl ModTable {
    pub fn new() -> ModTable  {
        Self {
            rows: Vec::new(),
        }
    }

    pub fn populate_mod_table(
        &mut self,
        mods: Vec<Mod>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.add_row(ModRow::new_header("Mod ID", "Name", "Summary", "Download Count"));

        for mod_info in mods {
            self.add_row(ModRow::new_data(
                mod_info.id,
                mod_info.name,
                mod_info.summary,
                mod_info.download_count
            ));
        }

        Ok(())
    }

    pub fn add_row(&mut self, row: ModRow) {
        self.rows.push(row);
    }

    pub fn printstd(&self) {
        let blank_row = "|--------------|--------------------------------|----------------------------------------------------|----------------|";

        println!("{}", blank_row);
        for row in &self.rows {
            println!("{}", row.format_row());
            println!("{}", blank_row);
        }
    }
}