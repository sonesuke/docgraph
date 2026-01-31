use ignore::WalkBuilder;
use ignore::gitignore::GitignoreBuilder;
use std::path::{Path, PathBuf};

pub fn find_markdown_files(root: &Path, ignore_patterns: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();

    // Build gitignore matcher from config patterns
    let mut builder = GitignoreBuilder::new(root);
    for pattern in ignore_patterns {
        builder.add_line(None, pattern).ok();
    }
    let ignore_matcher = builder.build().ok();

    let root_owned = root.to_path_buf();
    let walker = WalkBuilder::new(root)
        .hidden(false) // Look into hidden folders if needed
        .git_ignore(true)
        .filter_entry(move |entry| {
            let path = entry.path();
            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            let rel_path = path.strip_prefix(&root_owned).unwrap_or(path);

            if let Some(m) = &ignore_matcher
                && m.matched(rel_path, is_dir).is_ignore()
            {
                return false;
            }
            true
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_find_markdown_files() {
        let dir = tempdir().unwrap();
        let f1 = dir.path().join("a.md");
        let f2 = dir.path().join("b.txt");
        let subdir = dir.path().join("sub");
        std::fs::create_dir(&subdir).unwrap();
        let f3 = subdir.join("c.md");

        File::create(&f1).unwrap();
        File::create(&f2).unwrap();
        File::create(&f3).unwrap();

        let files = find_markdown_files(dir.path(), &[]);
        assert_eq!(files.len(), 2);
        let filenames: Vec<_> = files.iter().map(|p| p.file_name().unwrap()).collect();
        assert!(filenames.contains(&std::ffi::OsStr::new("a.md")));
        assert!(filenames.contains(&std::ffi::OsStr::new("c.md")));
    }

    #[test]
    fn test_find_markdown_files_ignore() {
        let dir = tempdir().unwrap();
        let f1 = dir.path().join("a.md");
        let f2 = dir.path().join("ignored.md");

        File::create(&f1).unwrap();
        File::create(&f2).unwrap();

        let files = find_markdown_files(dir.path(), &["ignored.md".to_string()]);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_name().unwrap(), "a.md");
    }

    #[test]
    fn test_find_markdown_files_ignore_folder() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("ignored_dir");
        std::fs::create_dir(&subdir).unwrap();
        let f1 = subdir.join("a.md");
        let f2 = dir.path().join("b.md");

        File::create(&f1).unwrap();
        File::create(&f2).unwrap();

        let files = find_markdown_files(dir.path(), &["ignored_dir".to_string()]);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_name().unwrap(), "b.md");
    }

    #[test]
    fn test_find_markdown_files_ignore_folder_trailing_slash() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("ignored_dir");
        std::fs::create_dir(&subdir).unwrap();
        let f1 = subdir.join("a.md");
        let f2 = dir.path().join("b.md");

        File::create(&f1).unwrap();
        File::create(&f2).unwrap();

        let files = find_markdown_files(dir.path(), &["ignored_dir/".to_string()]);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_name().unwrap(), "b.md");
    }
}
