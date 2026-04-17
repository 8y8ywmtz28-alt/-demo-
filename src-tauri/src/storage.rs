use crate::models::StorageNode;
use crate::protect::skip_dir;
use std::fs;
use std::path::Path;

fn size_of(path: &Path, depth: usize) -> StorageNode {
    let name = path
        .file_name()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());
    if path.is_file() {
        let size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        return StorageNode {
            path: path.display().to_string(),
            name,
            size,
            children: vec![],
        };
    }

    let mut children = vec![];
    let mut total = 0u64;
    if let Ok(rd) = fs::read_dir(path) {
        for entry in rd.flatten() {
            let p = entry.path();
            if skip_dir(&p) {
                continue;
            }
            let child = if depth > 0 {
                size_of(&p, depth - 1)
            } else {
                let s = dir_quick_size(&p);
                StorageNode {
                    path: p.display().to_string(),
                    name: p
                        .file_name()
                        .map(|x| x.to_string_lossy().to_string())
                        .unwrap_or_default(),
                    size: s,
                    children: vec![],
                }
            };
            total = total.saturating_add(child.size);
            children.push(child);
        }
    }
    children.sort_by(|a, b| b.size.cmp(&a.size));
    children.truncate(30);
    StorageNode {
        path: path.display().to_string(),
        name,
        size: total,
        children,
    }
}

fn dir_quick_size(path: &Path) -> u64 {
    if path.is_file() {
        return fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    }
    let mut total = 0u64;
    if let Ok(rd) = fs::read_dir(path) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_file() {
                total = total.saturating_add(fs::metadata(&p).map(|m| m.len()).unwrap_or(0));
            }
        }
    }
    total
}

pub fn analyze_storage(root: &str, depth: usize) -> StorageNode {
    size_of(Path::new(root), depth)
}
