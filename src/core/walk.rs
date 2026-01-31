use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub fn find_markdown_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(root)
        .hidden(false) // Look into hidden folders if needed? Default ignore rules apply usually.
        .git_ignore(true)
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file()
                    && let Some(ext) = path.extension()
                    && ext == "md"
                {
                    files.push(path.to_path_buf());
                }
            }
            Err(_err) => {} // Silence traversal errors in core
        }
    }
    files
}
