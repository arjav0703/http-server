pub fn directory_to_html(directory_name: &String) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("<title>Directory Structure</title>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");

    let directory = scan_directory(directory_name);
    generate_html_for_directory(&mut html, &directory, 0);

    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}

struct Directory {
    entries: Vec<DirectoryEntry>,
}
struct DirectoryEntry {
    name: String,
    file: Option<FileEntry>,
}
struct FileEntry {
    name: String,
}

fn generate_html_for_directory(html: &mut String, directory: &Directory, depth: usize) {
    let indent = "  ".repeat(depth);
    html.push_str(&format!("{}<ul>\n", indent));

    for entry in &directory.entries {
        html.push_str(&format!("{}<li>\n", indent));
        html.push_str(&format!("{}<strong>{}</strong>\n", indent, entry.name));

        html.push_str(&format!("{}</li>\n", indent));
    }

    html.push_str(&format!("{}</ul>\n", indent));
}

fn scan_directory(path: &str) -> Directory {
    let mut entries = Vec::new();

    if let Ok(dir_entries) = std::fs::read_dir(path) {
        for entry in dir_entries.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            let full_path = entry.path();

            if full_path.is_dir() {
                entries.push(DirectoryEntry { name, file: None });
            } else if full_path.is_file() {
                entries.push(DirectoryEntry {
                    name: name.clone(),
                    file: Some(FileEntry { name }),
                });
            }
        }
    }

    Directory { entries }
}
