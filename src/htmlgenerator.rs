pub fn directory_to_html(directory: &Directory) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("<title>Directory Structure</title>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");

    generate_html_for_directory(&mut html, directory, 0);

    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}

struct Directory {
    entries: Vec<DirectoryEntry>,
}
struct DirectoryEntry {
    name: String,
    subdirectory: Option<Box<Directory>>,
    file: Option<File>,
}
struct File {
    name: String,
}

fn generate_html_for_directory(html: &mut String, directory: &Directory, depth: usize) {
    let indent = "  ".repeat(depth);
    html.push_str(&format!("{}<ul>\n", indent));

    for entry in &directory.entries {
        html.push_str(&format!("{}<li>\n", indent));
        html.push_str(&format!("{}<strong>{}</strong>\n", indent, entry.name));

        if let Some(subdir) = &entry.subdirectory {
            generate_html_for_directory(html, subdir, depth + 1);
        } else if let Some(file) = &entry.file {
            html.push_str(&format!("{}<p>File: {}</p>\n", indent, file.name));
        }

        html.push_str(&format!("{}</li>\n", indent));
    }

    html.push_str(&format!("{}</ul>\n", indent));
}

