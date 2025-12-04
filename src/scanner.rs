use std::fs;
use std::path::Path;

use anyhow::Result;

fn should_skip_directory(dir_name: &str) -> bool {
    matches!(
        dir_name,
        "mocks" | "__mocks__" | "mocks_stubs" | "tests" | "environments" | "i18n"
    )
}

fn should_skip_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        if let Some(name_str) = file_name.to_str() {
            return name_str.ends_with(".spec.ts")
                || name_str.ends_with(".d.ts")
                || name_str.ends_with(".stories.ts")
                || name_str.ends_with("-stub.ts")
                || name_str.ends_with("mocks.ts")
                || name_str.ends_with("mock.ts");
        }
    }
    false
}

pub(crate) fn list_typescript_files(dir: &Path) -> Result<Vec<String>> {
    let mut ts_files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(dir_name) = path.file_name() {
                    if let Some(name_str) = dir_name.to_str() {
                        if should_skip_directory(name_str) {
                            continue;
                        }
                    }
                }

                match list_typescript_files(&path) {
                    Ok(mut nested_files) => ts_files.append(&mut nested_files),
                    Err(e) => eprintln!("Warning: Could not read directory {:?}: {}", path, e),
                }
            } else if path.is_file() {
                if should_skip_file(&path) {
                    continue;
                }

                if let Some(extension) = path.extension() {
                    if extension == "ts" || extension == "tsx" {
                        if let Some(path_str) = path.to_str() {
                            ts_files.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(ts_files)
}
