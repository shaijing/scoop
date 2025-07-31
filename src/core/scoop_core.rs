use std::{fs, io, path::Path};

pub fn installed_apps<P: AsRef<Path>>(apps_dir: P) -> io::Result<Vec<String>> {
    let mut apps = Vec::new();

    // 遍历 apps 目录
    for entry in fs::read_dir(apps_dir)? {
        let entry = entry?;
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
