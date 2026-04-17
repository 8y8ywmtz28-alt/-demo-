use crate::protect::is_protected_path;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn junk_map() -> HashMap<String, PathBuf> {
    let user = dirs::home_dir().unwrap_or_else(|| PathBuf::from("C:\\Users\\Public"));
    HashMap::from([
        ("win_temp".into(), PathBuf::from("C:\\Windows\\Temp")),
        ("user_temp".into(), std::env::temp_dir()),
        (
            "thumb".into(),
            user.join("AppData\\Local\\Microsoft\\Windows\\Explorer"),
        ),
        ("logs".into(), user.join("AppData\\Local\\Temp\\Logs")),
        (
            "edge_cache".into(),
            user.join("AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Cache"),
        ),
        (
            "chrome_cache".into(),
            user.join("AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache"),
        ),
    ])
}

fn clear_children(path: &Path) {
    if !path.exists() {
        return;
    }
    for entry in WalkDir::new(path)
        .min_depth(1)
        .contents_first(true)
        .into_iter()
        .filter_map(Result::ok)
    {
        let p = entry.path();
        let _ = if p.is_file() {
            fs::remove_file(p)
        } else {
            fs::remove_dir(p)
        };
    }
}

pub fn cleanup_selected(keys: Vec<String>) -> Vec<String> {
    let map = junk_map();
    let mut done = vec![];
    for key in keys {
        if key == "recycle" {
            if empty_recycle_bin().is_ok() {
                done.push(key);
            }
            continue;
        }

        if let Some(path) = map.get(&key) {
            let is_allowed_risky = key == "win_temp";
            if is_protected_path(path) && !is_allowed_risky {
                continue;
            }
            clear_children(path);
            done.push(key);
        }
    }
    done
}

pub fn empty_recycle_bin() -> Result<()> {
    #[cfg(windows)]
    {
        use std::process::Command;
        let out = Command::new("powershell")
            .arg("-Command")
            .arg("Clear-RecycleBin -Force -ErrorAction Stop")
            .output()?;
        if out.status.success() {
            return Ok(());
        }
        return Err(anyhow!(String::from_utf8_lossy(&out.stderr).to_string()));
    }
    #[cfg(not(windows))]
    {
        Err(anyhow!("仅支持 Windows"))
    }
}

fn recovery_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("无法定位用户目录"))?;
    let p = home.join(".wincleaner_recovery");
    fs::create_dir_all(&p)?;
    Ok(p)
}

pub fn safe_delete_file(path: &str) -> Result<String> {
    let src = Path::new(path);
    if is_protected_path(src) {
        return Err(anyhow!("系统保护目录，禁止删除"));
    }
    if !src.exists() {
        return Err(anyhow!("路径不存在"));
    }

    let stamp = chrono::Local::now().timestamp();
    let target_folder = recovery_dir()?.join(format!("{}", stamp));
    fs::create_dir_all(&target_folder)?;

    let file_name = src
        .file_name()
        .ok_or_else(|| anyhow!("非法文件名"))?
        .to_string_lossy()
        .to_string();
    let backup_file = target_folder.join(file_name);
    fs::rename(src, &backup_file)?;

    let meta = target_folder.join("meta.txt");
    fs::write(meta, path)?;

    Ok(backup_file.display().to_string())
}

pub fn restore_file(path: &str) -> Result<()> {
    let src = Path::new(path);
    if !src.exists() {
        return Err(anyhow!("恢复文件不存在"));
    }
    let Some(parent) = src.parent() else {
        return Err(anyhow!("恢复路径无效"));
    };
    let meta_file = parent.join("meta.txt");

    let fallback = dirs::desktop_dir().ok_or_else(|| anyhow!("桌面目录不可用"))?;
    let original_path = if meta_file.exists() {
        fs::read_to_string(meta_file).unwrap_or_default()
    } else {
        String::new()
    };

    let original = PathBuf::from(original_path.trim());
    let dest = if original.as_os_str().is_empty() {
        fallback.join(src.file_name().ok_or_else(|| anyhow!("恢复文件名无效"))?)
    } else {
        if let Some(p) = original.parent() {
            let _ = fs::create_dir_all(p);
        }
        original
    };

    fs::rename(src, dest)?;
    Ok(())
}
