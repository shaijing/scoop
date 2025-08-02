use std::{fs, path::Path};

use crate::util::errors::ScoopResult;
use anyhow::Context;

// pub fn installed_apps<P: AsRef<Path>>(apps_dir: P) -> io::Result<Vec<String>> {
//     let mut apps = Vec::new();

//     // 遍历 apps 目录
//     for entry in fs::read_dir(apps_dir)? {
//         let entry = entry?;
//         let path = entry.path();

//         // 判断是否为目录，且不是 scoop 本身
//         if path.is_dir() {
//             if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
//                 if name != "scoop" {
//                     apps.push(name.to_string());
//                 }
//             }
//         }
//     }

//     Ok(apps)
// }

pub fn installed_apps<P: AsRef<Path>>(apps_dir: P, _global: bool) -> ScoopResult<Vec<String>> {
    let mut apps = Vec::new();

    // 读取 apps 目录（带上下文）
    for entry in fs::read_dir(&apps_dir)
        .with_context(|| format!("failed to read apps directory: {:?}", apps_dir.as_ref()))?
    {
        let entry = entry.with_context(|| "failed to read a directory entry")?;
        let path = entry.path();

        // 判断是否为目录，且不是 scoop 本身
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name != "scoop" {
                    apps.push(name.to_string());
                }
            }
        }
    }

    Ok(apps)
}

pub mod manifest;