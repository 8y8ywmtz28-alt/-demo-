use crate::models::{DiskInfo, DuplicateGroup, LargeFileItem, ScanCategory};
use crate::protect::skip_dir;
use anyhow::Result;
use chrono::{DateTime, Local};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use sysinfo::Disks;
use walkdir::WalkDir;

pub fn list_disks() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    disks
        .iter()
        .map(|d| {
            let total = d.total_space();
            let avail = d.available_space();
            let used = total.saturating_sub(avail);
            let pct = if total == 0 {
                0.0
            } else {
                (used as f64 / total as f64) * 100.0
            };
            DiskInfo {
                mount_point: d.mount_point().display().to_string(),
                total_bytes: total,
                available_bytes: avail,
                used_bytes: used,
                used_percent: pct,
            }
        })
        .collect()
}

fn dir_size(path: &Path) -> (u64, u64, Vec<String>) {
    if !path.exists() {
        return (0, 0, vec![]);
    }
    let mut size = 0u64;
    let mut count = 0u64;
    let mut sample = vec![];
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        if entry.file_type().is_file() {
            if let Ok(m) = entry.metadata() {
                size = size.saturating_add(m.len());
                count += 1;
                if sample.len() < 3 {
                    sample.push(p.display().to_string());
                }
            }
        }
    }
    (size, count, sample)
}

pub fn scan_junk() -> Vec<ScanCategory> {
    let mut targets: Vec<(String, String, PathBuf, String, bool)> = vec![];
    let user = dirs::home_dir().unwrap_or_else(|| PathBuf::from("C:\\Users\\Public"));
    targets.push((
        "win_temp".into(),
        "Windows 临时文件".into(),
        PathBuf::from("C:\\Windows\\Temp"),
        "warning".into(),
        false,
    ));
    targets.push((
        "user_temp".into(),
        "用户临时目录".into(),
        std::env::temp_dir(),
        "safe".into(),
        true,
    ));
    targets.push((
        "thumb".into(),
        "缩略图缓存".into(),
        user.join("AppData\\Local\\Microsoft\\Windows\\Explorer"),
        "safe".into(),
        true,
    ));
    targets.push((
        "logs".into(),
        "日志缓存".into(),
        user.join("AppData\\Local\\Temp\\Logs"),
        "safe".into(),
        true,
    ));
    targets.push((
        "edge_cache".into(),
        "Edge 缓存".into(),
        user.join("AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Cache"),
        "warning".into(),
        false,
    ));
    targets.push((
        "chrome_cache".into(),
        "Chrome 缓存".into(),
        user.join("AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache"),
        "warning".into(),
        false,
    ));
    targets.push((
        "recycle".into(),
        "回收站".into(),
        PathBuf::from("C:\\$Recycle.Bin"),
        "warning".into(),
        false,
    ));

    targets
        .into_iter()
        .map(|(key, name, path, risk, default_selected)| {
            let (size, count, sample) = dir_size(&path);
            ScanCategory {
                key,
                name,
                total_size: size,
                file_count: count,
                risk_level: risk,
                sample_paths: sample,
                default_selected,
            }
        })
        .collect()
}

pub fn scan_large_files(root: &str, min_bytes: u64) -> Vec<LargeFileItem> {
    let mut files = vec![];
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if skip_dir(entry.path()) {
            continue;
        }
        if !entry.file_type().is_file() {
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        if meta.len() < min_bytes {
            continue;
        }
        let modified = meta
            .modified()
            .ok()
            .map(|m| {
                DateTime::<Local>::from(m)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            })
            .unwrap_or_else(|| "-".into());
        files.push(LargeFileItem {
            path: entry.path().display().to_string(),
            name: entry.file_name().to_string_lossy().to_string(),
            size: meta.len(),
            modified,
        });
    }
    files.sort_by(|a, b| b.size.cmp(&a.size));
    files.truncate(5000);
    files
}

fn sha256_file(path: &Path) -> Option<String> {
    let mut f = fs::File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = f.read(&mut buf).ok()?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Some(format!("{:x}", hasher.finalize()))
}

pub fn find_duplicates(root: &str, min_bytes: u64) -> Result<Vec<DuplicateGroup>> {
    let candidates = scan_large_files(root, min_bytes);
    let mut by_size_name: HashMap<(u64, String), Vec<LargeFileItem>> = HashMap::new();
    for file in candidates {
        by_size_name
            .entry((file.size, file.name.clone()))
            .or_default()
            .push(file);
    }

    let mut groups = vec![];
    for ((_size, _name), arr) in by_size_name {
        if arr.len() < 2 {
            continue;
        }
        let mut by_hash: HashMap<String, Vec<LargeFileItem>> = HashMap::new();
        for file in arr {
            if let Some(h) = sha256_file(Path::new(&file.path)) {
                by_hash.entry(h).or_default().push(file);
            }
        }
        for (hash, files) in by_hash {
            if files.len() < 2 {
                continue;
            }
            let total_size = files.iter().map(|x| x.size).sum::<u64>() - files[0].size;
            groups.push(DuplicateGroup {
                signature: hash,
                total_size,
                files,
            });
        }
    }
    groups.sort_by(|a, b| b.total_size.cmp(&a.total_size));
    Ok(groups)
}
