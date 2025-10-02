use crate::config::CurseForgeConfig;
use crate::curseforge_api;
use crate::mod_table::*;
use regex::Regex;
use std::fs;

pub async fn get_installed_mods() -> Result<(), Box<dyn std::error::Error>> {
    let config = CurseForgeConfig::get();
    let addon_path = config.wow_path.to_owned() + config.path_suffix.as_str();
    let re = Regex::new(r"\|c[fF].{7}").unwrap();

    let mut installed_mods: Vec<ModRow> = Vec::new();
    println!("{}", addon_path);
    // list directories in addon_path
    let entries = std::fs::read_dir(addon_path)?;
    for entry in entries {
        let entry = entry?;
        if entry.path().is_dir() {
            let mut dependency_dir = false;
            let files = fs::read_dir(entry.path())?;
            for file in files {
                let file = file?;
                if file.file_name().into_string().unwrap().as_str()
                    == entry.file_name().into_string().unwrap() + ".toc"
                    || file.file_name().into_string().unwrap().as_str()
                        == entry.file_name().into_string().unwrap() + "_Mainline.toc"
                {
                    let mut project_id = 0;
                    let download_count = 0;
                    let mut title = String::new();
                    let mut toc_version = String::new();
                    let mut changelog_version = String::new();
                    let mut notes = String::new();

                    //println!("  - {}", file.file_name().to_string_lossy());
                    // read the file and print the title and version
                    let content = std::fs::read_to_string(file.path())?;
                    for line in content.lines() {
                        if line.starts_with("## RequiredDeps:")
                            || line.starts_with("## Dependencies:")
                        {
                            //println!("Dep directory: {}", file.path().to_string_lossy());
                            dependency_dir = true;
                            break;
                        }

                        if line.starts_with("## X-Curse-Project-ID:") {
                            let project_id_str = line.replace("## X-Curse-Project-ID:", "");
                            let project_id_result = project_id_str.trim().parse::<u32>();
                            match project_id_result {
                                Ok(id) => project_id = id,
                                Err(_) => {
                                    project_id = 0;
                                }
                            }
                            //println!("    - Curse Project ID: {}", project_id);
                        }
                        if line.starts_with("## Title:") {
                            title = line.replace("## Title:", "").trim().to_string();
                            title = title.replace("[", "").replace("|r", "").replace("]", "");
                            title = re.replace_all(&title, "").to_string();
                            //println!("    - Title: {}", title);
                        }
                        if line.starts_with("## Version:") {
                            toc_version = line.replace("## Version:", "").trim().to_string();
                            //println!("    - Version: {}", version);
                        }
                        if line.starts_with("## Notes:") {
                            notes = line.replace("## Notes:", "").trim().to_string();
                            //println!("    - Notes: {}", notes);
                        }
                    }

                    if dependency_dir && project_id == 0 {
                        //println!("Filtering out directory: {}", file.path().to_string_lossy());
                        continue;
                    }
                    let already_added_mod = installed_mods.iter().any(|row| {
                        if let ModRow::Data { name, .. } = row {
                            *name == title
                        } else {
                            false
                        }
                    });

                    if already_added_mod {
                        //println!("Skipping duplicate mod: {}", title);
                        continue;
                    }

                    // handle getting version differently. Preferring the changelog version because that is more likely to match the archive name
                    if file.file_name().into_string().unwrap().as_str() == "CHANGELOG.md" {
                        changelog_version =
                            get_changelog_md_version(file.path().to_string_lossy().to_string());
                    } else if file.file_name().into_string().unwrap().as_str() == "CHANGELOG.txt" {
                        changelog_version =
                            get_changelog_txt_version(file.path().to_string_lossy().to_string());
                    } else if file.file_name().into_string().unwrap().as_str() == "Changelog.lua" {
                        changelog_version =
                            get_changelog_lua_version(file.path().to_string_lossy().to_string());
                    }

                    if project_id == 0 {
                        project_id = get_id_by_search(&title).await;
                    }

                    let version = if !changelog_version.is_empty() {
                        changelog_version
                    } else {
                        toc_version
                    };
                    let mod_row =
                        ModRow::new_data(project_id, title, version, notes, download_count);
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

fn get_changelog_md_version(path: String) -> String {
    let content = fs::read_to_string(path).unwrap();
    let lines = content.lines();
    for line in lines {
        if line.starts_with("## [") {
            return line
                .split("[")
                .nth(1)
                .unwrap()
                .split("]")
                .nth(0)
                .unwrap()
                .to_string();
        }
    }
    String::new()
}

fn get_changelog_txt_version(path: String) -> String {
    let content = fs::read_to_string(path).unwrap();
    let lines = content.lines();
    for line in lines {
        if line.starts_with("##") {
            return line.split(" ").nth(1).unwrap().to_string();
        }
    }
    String::new()
}

fn get_changelog_lua_version(path: String) -> String {
    let content = fs::read_to_string(path).unwrap();
    let lines = content.lines();
    for line in lines {
        if line.starts_with("v.") {
            return line.split("v.").nth(1).unwrap().to_string();
        }
    }
    String::new()
}

async fn get_id_by_search(mod_name: &String) -> u32 {
    let mods = curseforge_api::search_mods(1, &mod_name).await.unwrap();
    for game_mod in mods {
        if game_mod.name.to_lowercase() == mod_name.to_lowercase() {
            //println!("Found mod: {}, ({})", game_mod.name, game_mod.id);
            return game_mod.id;
        }
    }
    0
}
