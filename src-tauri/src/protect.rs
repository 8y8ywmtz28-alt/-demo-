use std::path::Path;

pub fn is_protected_path(path: &Path) -> bool {
    let s = path.to_string_lossy().to_lowercase();
    [
        "\\windows",
        "\\program files",
        "\\program files (x86)",
        "\\programdata",
        "\\system volume information",
        "\\$recycle.bin",
    ]
    .iter()
    .any(|k| s.contains(k))
}

pub fn skip_dir(path: &Path) -> bool {
    is_protected_path(path)
}
