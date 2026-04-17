#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cleanup;
mod models;
mod protect;
mod scanner;
mod startup;
mod storage;

use anyhow::Result;
use chrono::{DateTime, Duration, Local};
use models::{ChatCacheItem, DesktopSuggestion, RecentFile, StartupItem, StorageNode};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[tauri::command]
fn list_disks() -> Vec<models::DiskInfo> {
    scanner::list_disks()
}

#[tauri::command]
fn scan_junk(_target: Option<String>) -> Vec<models::ScanCategory> {
    scanner::scan_junk()
}

#[tauri::command]
fn cleanup_selected(keys: Vec<String>) -> Vec<String> {
    cleanup::cleanup_selected(keys)
}

#[tauri::command]
fn scan_large_files(root: String, min_bytes: u64) -> Vec<models::LargeFileItem> {
    scanner::scan_large_files(&root, min_bytes)
}

#[tauri::command]
fn find_duplicates(root: String, min_bytes: u64) -> Result<Vec<models::DuplicateGroup>, String> {
    scanner::find_duplicates(&root, min_bytes).map_err(|e| e.to_string())
}

#[tauri::command]
fn safe_delete_file(path: String) -> Result<String, String> {
    cleanup::safe_delete_file(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn restore_file(path: String) -> Result<(), String> {
    cleanup::restore_file(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_startup_items() -> Vec<StartupItem> {
    startup::list_startup_items()
}

#[tauri::command]
fn set_startup_status(id: String, enable: bool) -> Result<(), String> {
    startup::set_startup_status(&id, enable).map_err(|e| e.to_string())
}

#[tauri::command]
fn analyze_storage(root: String, depth: usize) -> StorageNode {
    storage::analyze_storage(&root, depth)
}

#[tauri::command]
fn open_in_explorer(path: String) -> Result<(), String> {
    #[cfg(windows)]
    {
        use std::process::Command;
        let target = if Path::new(&path).is_file() {
            Path::new(&path)
                .parent()
                .map(|p| p.display().to_string())
                .unwrap_or(path)
        } else {
            path
        };
        Command::new("explorer")
            .arg(target)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(windows))]
    {
        let _ = path;
        Err("仅支持 Windows".into())
    }
}

#[tauri::command]
fn empty_recycle_bin() -> Result<String, String> {
    cleanup::empty_recycle_bin()
        .map(|_| "回收站清空完成".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_recent_downloads() -> Vec<RecentFile> {
    let Some(dir) = dirs::download_dir() else {
        return vec![];
    };
    let threshold = Local::now() - Duration::days(14);
    let mut out = vec![];
    for e in WalkDir::new(dir)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !e.file_type().is_file() {
            continue;
        }
        let Ok(meta) = e.metadata() else { continue };
        let modified = meta
            .modified()
            .ok()
            .map(DateTime::<Local>::from)
            .unwrap_or_else(Local::now);
        if modified < threshold {
            continue;
        }
        out.push(RecentFile {
            path: e.path().display().to_string(),
            name: e.file_name().to_string_lossy().to_string(),
            size: meta.len(),
            modified: modified.format("%Y-%m-%d %H:%M:%S").to_string(),
        });
    }
    out.sort_by(|a, b| b.modified.cmp(&a.modified));
    out.truncate(200);
    out
}

#[tauri::command]
fn get_desktop_suggestions() -> Vec<DesktopSuggestion> {
    let Some(dir) = dirs::desktop_dir() else {
        return vec![];
    };
    let mut map: HashMap<String, (u64, u64)> = HashMap::new();
    let classify = |ext: &str| match ext {
        "zip" | "rar" | "7z" => "压缩包",
        "mp4" | "mkv" | "mov" => "视频",
        "jpg" | "png" | "jpeg" | "gif" => "图片",
        "exe" | "msi" => "安装包",
        "doc" | "docx" | "pdf" | "xls" => "文档",
        _ => "其他",
    };
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if !p.is_file() {
                continue;
            }
            let ext = p
                .extension()
                .map(|x| x.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            let c = classify(&ext).to_string();
            let size = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
            let entry = map.entry(c).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += size;
        }
    }
    map.into_iter()
        .map(|(k, (count, size))| DesktopSuggestion {
            category: k,
            count,
            total_size: size,
        })
        .collect()
}

#[tauri::command]
fn detect_chat_caches() -> Vec<ChatCacheItem> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("C:\\Users\\Public"));
    let candidates = vec![
        ("微信", home.join("Documents\\WeChat Files")),
        ("QQ", home.join("Documents\\Tencent Files")),
    ];
    candidates
        .into_iter()
        .map(|(name, path)| ChatCacheItem {
            name: name.into(),
            size: dir_size_quick(&path),
            path: path.display().to_string(),
        })
        .filter(|x| x.size > 0)
        .collect()
}

fn dir_size_quick(path: &Path) -> u64 {
    if !path.exists() {
        return 0;
    }
    WalkDir::new(path)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

#[tauri::command]
fn quick_dirs() -> HashMap<String, String> {
    let mut out = HashMap::new();
    if let Some(v) = dirs::download_dir() {
        out.insert("下载".into(), v.display().to_string());
    }
    if let Some(v) = dirs::desktop_dir() {
        out.insert("桌面".into(), v.display().to_string());
    }
    if let Some(v) = dirs::document_dir() {
        out.insert("文档".into(), v.display().to_string());
    }
    if let Some(v) = dirs::picture_dir() {
        out.insert("图片".into(), v.display().to_string());
    }
    if let Some(v) = dirs::video_dir() {
        out.insert("视频".into(), v.display().to_string());
    }
    out
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_disks,
            scan_junk,
            cleanup_selected,
            scan_large_files,
            find_duplicates,
            safe_delete_file,
            restore_file,
            list_startup_items,
            set_startup_status,
            analyze_storage,
            open_in_explorer,
            empty_recycle_bin,
            get_recent_downloads,
            get_desktop_suggestions,
            detect_chat_caches,
            quick_dirs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
