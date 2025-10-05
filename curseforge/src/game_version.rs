use crate::{CurseForgeConfig, Mod, ModFile};
use once_cell::sync::Lazy;
use roxmltree::Document;
use std::fs;

static GAME_VERSION: Lazy<Result<String, String>> =
    Lazy::new(|| load_game_version().map_err(|e| e.to_string()));

fn load_game_version() -> Result<String, Box<dyn std::error::Error>> {
    let config = CurseForgeConfig::get();
    let version_file_path = if fs::exists(config.wow_path.to_string() + "/_retail_/World of Warcraft.app/Contents/Info.plist").unwrap() {
        config.wow_path.to_string() + "/_retail_/World of Warcraft.app/Contents/Info.plist"
    } else {
        config.wow_path.to_string() + "/.build.info"
    };

    if version_file_path.ends_with("Info.plist") {
        get_version_from_info_plist(&version_file_path)
    } else {
        get_version_from_build_info(&version_file_path)
    }
}

fn get_version_from_info_plist(plist_path: &String) -> Result<String, Box<dyn std::error::Error>> {
    let xml_content = fs::read_to_string(&plist_path)?;
    let doc = Document::parse_with_options(
        &xml_content,
        roxmltree::ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        },
    )?;

    // Find the CFBundleShortVersionString key
    for node in doc.descendants() {
        if node.has_tag_name("key") && node.text() == Some("CFBundleShortVersionString") {
            // The value is in the next sibling <string> element
            let mut next = node.next_sibling();
            while let Some(n) = next {
                if n.is_element() && n.has_tag_name("string") {
                    if let Some(version) = n.text() {
                        return Ok(version.to_string());
                    }
                }
                next = n.next_sibling();
            }
        }
    }

    Err("Version not found in plist".into())
}

fn get_version_from_build_info(
    build_info_path: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&build_info_path)?;
    let mut header_row = true;
    let mut version_header_index = 0;
    for line in content.lines() {
        if header_row {
            header_row = false;
            let headers: Vec<&str> = line.split_whitespace().collect();
            for (i, header) in headers.iter().enumerate() {
                if *header == "Version!STRING:0" {
                    version_header_index = i;
                    break;
                }
            }
            continue;
        } else if version_header_index == 0 {
            return Err("Version not found in .build.info".into());
        }

        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() > version_header_index {
            let version = columns[version_header_index];
            if !version.is_empty() {
                return Ok(version
                    .rfind('.')
                    .map(|pos| &version[..pos])
                    .unwrap_or(version)
                    .parse()
                    .unwrap());
            }
        }
    }

    Err("Version not found in build.info".into())
}

fn get_game_version() -> Result<&'static str, Box<dyn std::error::Error>> {
    GAME_VERSION
        .as_ref()
        .map(|s| s.as_str())
        .map_err(|e| e.clone().into())
}

pub(crate) fn get_mod_file_for_game_version<'a>(
    game_mod: &'a Mod,
) -> Result<&'a ModFile, Box<dyn std::error::Error>> {
    let game_version = get_game_version()?;

    for file in &game_mod.latest_files {
        for file_game_version in &file.game_versions {
            if game_version == *file_game_version {
                return Ok(file);
            }
        }
    }

    Err("File not found for game version".into())
}
