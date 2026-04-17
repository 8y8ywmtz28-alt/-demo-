use crate::models::StartupItem;
use anyhow::{anyhow, Result};
#[cfg(windows)]
use winreg::{enums::*, RegKey};

#[cfg(windows)]
fn read_run_items(hive: HKEY, hive_name: &str, vec: &mut Vec<StartupItem>) {
    let h = RegKey::predef(hive);
    if let Ok(run) = h.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
        for item in run.enum_values().flatten() {
            let name = item.0;
            let cmd: String = run.get_value(&name).unwrap_or_default();
            let id = format!("reg:{}:{}", hive_name, name);
            vec.push(StartupItem {
                id,
                name,
                source: format!("注册表 {}", hive_name),
                command: cmd,
                enabled: true,
                recommendation: if name.to_lowercase().contains("security") {
                    "keep".into()
                } else {
                    "review".into()
                },
                mutable: hive == HKEY_CURRENT_USER,
            });
        }
    }
}

pub fn list_startup_items() -> Vec<StartupItem> {
    let mut vec = vec![];
    #[cfg(windows)]
    {
        read_run_items(HKEY_CURRENT_USER, "HKCU", &mut vec);
        read_run_items(HKEY_LOCAL_MACHINE, "HKLM", &mut vec);
        if let Some(dir) = dirs::data_dir() {
            let startup = dir.join("Microsoft\\Windows\\Start Menu\\Programs\\Startup");
            if startup.exists() {
                if let Ok(rd) = std::fs::read_dir(&startup) {
                    for e in rd.flatten() {
                        let p = e.path();
                        let Some(name) = p.file_name().map(|x| x.to_string_lossy().to_string())
                        else {
                            continue;
                        };
                        vec.push(StartupItem {
                            id: format!("startup:{}", p.display()),
                            name: name.clone(),
                            source: "启动文件夹".into(),
                            command: p.display().to_string(),
                            enabled: !name.ends_with(".disabled"),
                            recommendation: "review".into(),
                            mutable: true,
                        });
                    }
                }
            }
        }
    }
    vec
}

pub fn set_startup_status(id: &str, enable: bool) -> Result<()> {
    #[cfg(windows)]
    {
        if let Some(remain) = id.strip_prefix("reg:HKCU:") {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let run = hkcu.open_subkey_with_flags(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                KEY_READ | KEY_WRITE,
            )?;
            let approved = hkcu
                .create_subkey(
                    "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run",
                )?
                .0;
            let val: Vec<u8> = if enable {
                vec![0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            } else {
                vec![0x03, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            };
            let _cmd: String = run.get_value(remain)?;
            approved.set_value(remain, &val)?;
            return Ok(());
        }
        if let Some(path) = id.strip_prefix("startup:") {
            let p = std::path::PathBuf::from(path);
            if enable {
                if let Some(name) = p.file_name().map(|x| x.to_string_lossy().to_string()) {
                    if name.ends_with(".disabled") {
                        let new = p.with_file_name(name.trim_end_matches(".disabled"));
                        std::fs::rename(p, new)?;
                    }
                }
            } else {
                let new = std::path::PathBuf::from(format!("{}.disabled", path));
                std::fs::rename(p, new)?;
            }
            return Ok(());
        }
    }
    Err(anyhow!("不支持或无权限修改该启动项"))
}
