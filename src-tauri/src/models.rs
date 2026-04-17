use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DiskInfo {
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub used_percent: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanCategory {
    pub key: String,
    pub name: String,
    pub total_size: u64,
    pub file_count: u64,
    pub risk_level: String,
    pub sample_paths: Vec<String>,
    pub default_selected: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct LargeFileItem {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub modified: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DuplicateGroup {
    pub signature: String,
    pub total_size: u64,
    pub files: Vec<LargeFileItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StartupItem {
    pub id: String,
    pub name: String,
    pub source: String,
    pub command: String,
    pub enabled: bool,
    pub recommendation: String,
    pub mutable: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct StorageNode {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub children: Vec<StorageNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecentFile {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub modified: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DesktopSuggestion {
    pub category: String,
    pub count: u64,
    pub total_size: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatCacheItem {
    pub name: String,
    pub path: String,
    pub size: u64,
}
