use crate::{CurseForgeConfig, Mod, ModFile};
use once_cell::sync::Lazy;
use roxmltree::Document;
use std::fs;

static GAME_VERSION: Lazy<Result<String, String>> =
    Lazy::new(|| load_game_version().map_err(|e| e.to_string()));

fn load_game_version() -> Result<String, Box<dyn std::error::Error>> {
    let config = CurseForgeConfig::get();
    let plist_path =
        config.wow_path.to_string() + "/_retail_/World of Warcraft.app/Contents/Info.plist";
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
