use crate::config::CurseForgeConfig;

pub async fn get_installed_mods() -> Result<(), Box<dyn std::error::Error>> {
    let config = CurseForgeConfig::get();
    let addon_path  = config.wow_path.to_owned() + config.path_suffix.as_str();

    println!("{}", addon_path);
    // list directories in addon_path
    let entries = std::fs::read_dir(addon_path)?;
    for entry in entries {
        let entry = entry?;
        if entry.path().is_dir() {
            println!("{}", entry.file_name().to_string_lossy());
            let files = std::fs::read_dir(entry.path())?;
            for file in files {
                let file = file?;
                if file.file_name().into_string().unwrap().as_str() == entry.file_name().into_string().unwrap() + ".toc" ||
                    file.file_name().into_string().unwrap().as_str() == entry.file_name().into_string().unwrap() + "_Mainline.toc"{
                    println!("  - {}", file.file_name().to_string_lossy());
                    // read the file and print the title and version
                    let content = std::fs::read_to_string(file.path())?;
                    for line in content.lines() {
                        if line.starts_with("## X-Curse-Project-ID:") {
                            println!("    - Curse Project ID: {}", line.replace("## X-Curse-Project-ID:", "").trim());
                        }
                        if line.starts_with("## Title:") {
                            println!("    - Title: {}", line.replace("## Title:", "").trim());
                        }
                        if line.starts_with("## Version:") {
                            println!("    - Version: {}", line.replace("## Version:", "").trim());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

