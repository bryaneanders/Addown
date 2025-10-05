use crate::models::Mod;
use std::str::SplitWhitespace;

pub enum ModRow {
    Header {
        id: String,
        name: String,
        version: String,
        summary: String,
        download_count: String,
    },
    Data {
        id: u32,
        name: String,
        version: String,
        summary: String,
        download_count: u32,
    },
}

impl ModRow {
    pub fn new_header(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        summary: impl Into<String>,
        download_count: impl Into<String>,
    ) -> Self {
        Self::Header {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            summary: summary.into(),
            download_count: download_count.into(),
        }
    }

    pub fn new_data(
        id: u32,
        name: impl Into<String>,
        version: impl Into<String>,
        summary: impl Into<String>,
        download_count: u32,
    ) -> Self {
        Self::Data {
            id,
            name: name.into(),
            version: version.into(),
            summary: summary.into(),
            download_count,
        }
    }

    pub fn format_row(&self) -> String {
        match self {
            ModRow::Header {
                id,
                name,
                version,
                summary,
                download_count,
            } => {
                format!(
                    "| {:<12} | {:<30} | {:<30} | {:<50} | {:<14} |",
                    id, name, version, summary, download_count
                )
            }
            ModRow::Data {
                id,
                name,
                version,
                summary,
                download_count,
            } => {
                let name_rows_words = name.split_whitespace();
                let summary_rows_words = summary.split_whitespace();
                let version_rows_words = version.split_whitespace();

                let mut name_rows: Vec<String> = Vec::new();
                let mut summary_rows: Vec<String> = Vec::new();
                let mut version_rows: Vec<String> = Vec::new();
                let mut line_rows: Vec<String> = Vec::new();

                name_rows.push(String::new());
                Self::wrap_field_line(name_rows_words, &mut name_rows, 30);

                summary_rows.push(String::new());
                Self::wrap_field_line(summary_rows_words, &mut summary_rows, 50);

                version_rows.push(String::new());
                Self::wrap_field_line(version_rows_words, &mut version_rows, 30);

                let num_rows = std::cmp::max(
                    std::cmp::max(name_rows.len(), summary_rows.len()),
                    version_rows.len(),
                );
                for i in 0..num_rows {
                    let name = if i < name_rows.len() {
                        &name_rows[i]
                    } else {
                        ""
                    };
                    let summary = if i < summary_rows.len() {
                        &summary_rows[i]
                    } else {
                        ""
                    };
                    let version = if i < version_rows.len() {
                        &version_rows[i]
                    } else {
                        ""
                    };
                    if i == 0 {
                        line_rows.push(format!(
                            "| {:<12} | {:<30} | {:<30} | {:<50} | {:<14} |",
                            id, name, version, summary, download_count
                        ));
                    } else {
                        line_rows.push(format!(
                            "| {:<12} | {:<30} | {:<30} | {:<50} | {:<14} |",
                            "", name, version, summary, ""
                        ));
                    }
                }
                line_rows.join("\n")
            }
        }
    }

    fn wrap_field_line(
        version_rows_words: SplitWhitespace,
        summary_rows: &mut Vec<String>,
        length: usize,
    ) {
        for word in version_rows_words {
            if summary_rows.last().unwrap().len() + word.len() + 1 > length {
                summary_rows.push(word.to_string());
            } else {
                if !summary_rows.last().unwrap().is_empty() {
                    summary_rows.last_mut().unwrap().push(' ');
                }
                summary_rows.last_mut().unwrap().push_str(word);
            }
        }
    }
}

pub struct ModTable {
    rows: Vec<ModRow>,
}

impl ModTable {
    pub fn new() -> ModTable {
        Self { rows: Vec::new() }
    }

    pub fn populate_mods_table(
        &mut self,
        mods: Vec<Mod>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.add_row(ModRow::new_header(
            "Mod ID",
            "Name",
            "Version/Display Name",
            "Summary",
            "Download Count",
        ));

        for mod_info in mods {
            self.add_row(ModRow::new_data(
                mod_info.id,
                &mod_info.name,
                &mod_info.latest_files[0].display_name,
                &mod_info.summary,
                mod_info.download_count,
            ));
        }

        Ok(())
    }

    pub fn populate_installed_mods_table(
        &mut self,
        mods: Vec<ModRow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.add_row(ModRow::new_header(
            "Mod ID",
            "Name",
            "Version/Display Name",
            "Summary",
            "Download Count",
        ));

        let mut mods = mods;
        mods.sort_by_key(|mod_row| match mod_row {
            ModRow::Header { name, .. } => name.clone(),
            ModRow::Data { name, .. } => name.clone(),
        });

        for mod_row in mods {
            self.add_row(mod_row);
        }

        Ok(())
    }

    pub fn add_row(&mut self, row: ModRow) {
        self.rows.push(row);
    }

    pub fn print_table(&self) {
        let table = self.format_table();
        for line in table {
            print!("{}", line);
        }
    }

    pub fn format_table(&self) -> Vec<String> {
        let blank_row = "|--------------|--------------------------------|--------------------------------|----------------------------------------------------|----------------|";
        let mut table = Vec::new();

        table.push(format!("{}\n", blank_row));
        for row in &self.rows {
            table.push(format!("{}\n", row.format_row()));
            table.push(format!("{}\n", blank_row));
        }

        table
    }
}
