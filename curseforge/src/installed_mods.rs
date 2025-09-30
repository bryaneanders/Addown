use crate::config::CurseForgeConfig;
use crate::mod_table::*;

pub async fn get_installed_mods() -> Result<(), Box<dyn std::error::Error>> {
    let config = CurseForgeConfig::get();
    let addon_path  = config.wow_path.to_owned() + config.path_suffix.as_str();

    let mut installed_mods : Vec<ModRow> = Vec::new();
    println!("{}", addon_path);
    // list directories in addon_path
    let entries = std::fs::read_dir(addon_path)?;
    for entry in entries {
        let entry = entry?;
        if entry.path().is_dir() {
            //println!("{}", entry.file_name().to_string_lossy());
            let files = std::fs::read_dir(entry.path())?;
            for file in files {
                let file = file?;
                if file.file_name().into_string().unwrap().as_str() == entry.file_name().into_string().unwrap() + ".toc" ||
                    file.file_name().into_string().unwrap().as_str() == entry.file_name().into_string().unwrap() + "_Mainline.toc"{


                    let mut project_id= 0;
                    let download_count = 0;
                    let mut title = String::new();
                    let mut version = String::new();
                    let mut notes = String::new();

                    //println!("  - {}", file.file_name().to_string_lossy());
                    // read the file and print the title and version
                    let content = std::fs::read_to_string(file.path())?;
                    for line in content.lines() {
                        if line.starts_with("## X-Curse-Project-ID:") {
                            let project_id_str = line.replace("## X-Curse-Project-ID:", "");
                            let project_id_result = project_id_str.trim().parse::<u32>();
                            match project_id_result {
                                Ok(id) => { project_id = id },
                                Err(_) => { project_id = 0; },
                            }
                            //println!("    - Curse Project ID: {}", project_id);
                        }
                        if line.starts_with("## Title:") {
                            title = line.replace("## Title:", "").trim().to_string();
                            title = title.replace("[|cffeda55f", "").replace("|cffFFFFFFM", "").replace("|r", "").replace("]", "");
                            //println!("    - Title: {}", title);
                        }
                        if line.starts_with("## Version:") {
                            version = line.replace("## Version:", "").trim().to_string();
                            //println!("    - Version: {}", version);
                        }
                        if line.starts_with("## Notes:") {
                            notes = line.replace("## Notes:", "").trim().to_string();
                            //println!("    - Notes: {}", notes);
                        }
                    }

                    let mod_row = ModRow::new_data(project_id, title, version, notes, download_count);
                    installed_mods.push(mod_row);
                }
            }
        }
    }

    let mut mod_table = ModTable::new();
    mod_table.populate_installed_mods_table(installed_mods)?;
    mod_table.printstd();

    Ok(())
}

