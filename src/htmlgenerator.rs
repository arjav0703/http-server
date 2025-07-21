use std::path::Path;

pub fn directory_to_html(directory_name: &str) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("<style>\n");
    html.push_str("  body { font-family: Arial, sans-serif; margin: 20px; }\n");
    html.push_str("  h1 { color: #333; }\n");
    html.push_str("  ul { list-style-type: none; padding-left: 20px; }\n");
    html.push_str("  li { margin: 5px 0; }\n");
    html.push_str("  .directory { font-weight: bold; }\n");
    html.push_str("  a { text-decoration: none; color: #0066cc; }\n");
    html.push_str("  a:hover { text-decoration: underline; }\n");
    html.push_str("</style>\n");
    html.push_str(format!("<title>Directory: {}</title>\n", directory_name).as_str());
    html.push_str("</head>\n");
    html.push_str("<body>\n");

    html.push_str(format!("<h1>Index of {}</h1>\n", directory_name).as_str());

    // append parent directory link if not at root
    let path = Path::new(directory_name);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            html.push_str("<p><a href=\"../\">Parent Directory</a></p>\n");
        }
    }

    let directory = scan_directory(directory_name);
    generate_html_for_directory(&mut html, &directory);

    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}

struct Directory {
    entries: Vec<DirectoryEntry>,
}

#[derive(Clone)]
struct DirectoryEntry {
    name: String,
    is_directory: bool,
    path: String,
}

fn generate_html_for_directory(html: &mut String, directory: &Directory) {
    html.push_str("<ul>\n");

    // Sort entries - directories first, then files, both alphabetically
    let mut sorted_entries = directory.entries.clone();
    sorted_entries.sort_by(|a, b| match (a.is_directory, b.is_directory) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    for entry in &sorted_entries {
        let url_path = format!("{}/", entry.path);

        html.push_str("  <li>");
        if entry.is_directory {
            html.push_str(
                format!(
                    "<span class=\"directory\">📁 </span><a href=\"{}\">{}/</a>",
                    url_path, entry.name
                )
                .as_str(),
            );
        } else {
            html.push_str(
                format!(
                    "<span>📄 </span><a href=\"{}\">{}</a>",
                    entry.path, entry.name
                )
                .as_str(),
            );
        }
        html.push_str("</li>\n");
    }

    html.push_str("</ul>\n");
}

fn scan_directory(path: &str) -> Directory {
    let mut entries = Vec::new();

    if let Ok(dir_entries) = std::fs::read_dir(path) {
        for entry_result in dir_entries {
            if let Ok(entry) = entry_result {
                let name = entry.file_name().into_string().unwrap_or_default();
                let full_path = entry.path();

                if name.starts_with('.') {
                    continue;
                }

                // relative path for URLs
                let rel_path = if let Ok(rel) = full_path.strip_prefix(path) {
                    rel.to_string_lossy().to_string()
                } else {
                    name.clone()
                };

                // URL encode the path for special characters
                let encoded_path = url_encode(&rel_path);

                entries.push(DirectoryEntry {
                    name,
                    is_directory: full_path.is_dir(),
                    path: encoded_path,
                });
            }
        }
    }

    Directory { entries }
}

// Simple URL encoding function for path components
fn url_encode(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);
    for c in s.chars() {
        match c {
            ' ' => result.push_str("%20"),
            '<' => result.push_str("%3C"),
            '>' => result.push_str("%3E"),
            '#' => result.push_str("%23"),
            '%' => result.push_str("%25"),
            '"' => result.push_str("%22"),
            '\'' => result.push_str("%27"),
            '&' => result.push_str("%26"),
            '+' => result.push_str("%2B"),
            '?' => result.push_str("%3F"),
            // Safe characters
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '/' => result.push(c),
            // Everything else
            _ => {
                for b in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", b));
                }
            }
        }
    }
    result
}
