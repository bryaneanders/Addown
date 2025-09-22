use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponse {
    pub data: Game,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameArrayResponse {
    pub data: Vec<Game>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: u32,
    pub name: String,
    pub slug: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    pub assets: GameAssets,
    pub status: u32,
    #[serde(rename = "apiStatus")]
    pub api_status: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAssets {
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "tileUrl")]
    pub tile_url: Option<String>,
    #[serde(rename = "coverUrl")]
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModResponse {
    pub data: Mod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModArrayResponse {
    pub data: Vec<Mod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    pub screenshots: Vec<String>,
    pub id: u32,
    #[serde(rename = "gameId")]
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub links: ModLinks,
    pub summary: String,
    pub status: u32,
    #[serde(rename = "downloadCount")]
    pub download_count: u32,
    #[serde(rename = "isFeatured")]
    pub is_featured: bool,
    #[serde(rename = "primaryCategoryId")]
    pub primary_category_id: u32,
    pub categories: Vec<Category>,
    #[serde(rename = "classId")]
    pub class_id: u32,
    pub authors: Vec<Author>,
    pub logo: ModLogo,
    #[serde(rename = "mainFileId")]
    pub main_file_id: u32,
    #[serde(rename = "latestFiles")]
    pub latest_files: Vec<ModFile>,
    #[serde(rename = "latestFilesIndexes")]
    pub latest_files_indexes: Vec<FileIndex>,
    #[serde(rename = "latestEarlyAccessFilesIndexes")]
    pub latest_early_access_files_indexes: Vec<FileIndex>,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "dateReleased")]
    pub date_released: String,
    #[serde(rename = "allowModDistribution")]
    pub allow_mod_distribution: bool,
    #[serde(rename = "gamePopularityRank")]
    pub game_popularity_rank: u32,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "hasCommentsEnabled")]
    pub has_comments_enabled: bool,
    #[serde(rename = "thumbsUpCount")]
    pub thumbs_up_count: u32,
    #[serde(rename = "featuredProjectTag")]
    pub featured_project_tag: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModLinks {
    #[serde(rename = "websiteUrl")]
    pub website_url: String,
    #[serde(rename = "wikiUrl")]
    pub wiki_url: Option<String>,
    #[serde(rename = "issuesUrl")]
    pub issues_url: Option<String>,
    #[serde(rename = "sourceUrl")]
    pub source_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: u32,
    #[serde(rename = "gameId")]
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub url: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "isClass")]
    pub is_class: bool,
    #[serde(rename = "classId")]
    pub class_id: u32,
    #[serde(rename = "parentCategoryId")]
    pub parent_category_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: u32,
    pub name: String,
    pub url: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModLogo {
    pub id: u32,
    #[serde(rename = "modId")]
    pub mod_id: u32,
    pub title: String,
    pub description: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModFile {
    pub id: u32,
    #[serde(rename = "gameId")]
    pub game_id: u32,
    #[serde(rename = "modId")]
    pub mod_id: u32,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "releaseType")]
    pub release_type: u32,
    #[serde(rename = "fileStatus")]
    pub file_status: u32,
    pub hashes: Vec<FileHash>,
    #[serde(rename = "fileDate")]
    pub file_date: String,
    #[serde(rename = "fileLength")]
    pub file_length: u64,
    #[serde(rename = "downloadCount")]
    pub download_count: u32,
    #[serde(rename = "fileSizeOnDisk")]
    pub file_size_on_disk: u64,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<String>,
    #[serde(rename = "sortableGameVersions")]
    pub sortable_game_versions: Vec<GameVersion>,
    pub dependencies: Vec<String>,
    #[serde(rename = "alternateFileId")]
    pub alternate_file_id: u32,
    #[serde(rename = "isServerPack")]
    pub is_server_pack: bool,
    #[serde(rename = "fileFingerprint")]
    pub file_fingerprint: u64,
    pub modules: Vec<FileModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHash {
    pub value: String,
    pub algo: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVersion {
    #[serde(rename = "gameVersionName")]
    pub game_version_name: String,
    #[serde(rename = "gameVersionPadded")]
    pub game_version_padded: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "gameVersionReleaseDate")]
    pub game_version_release_date: String,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileModule {
    pub name: String,
    pub fingerprint: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileIndex {
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "fileId")]
    pub file_id: u32,
    pub filename: String,
    #[serde(rename = "releaseType")]
    pub release_type: u32,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: u32,
}
