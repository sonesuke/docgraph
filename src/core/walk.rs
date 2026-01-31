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

    let walker = WalkBuilder::new(root)
        .hidden(false) // Look into hidden folders if needed
        .git_ignore(true)
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();

                // transform path to relative for checking against ignore_matcher if needed,
                // but ignore_matcher.matched(path, is_dir) usually handles absolute if root matches?
                // Actually Gitignore::matched checks relative to the builder root.
                // Let's rely on standard ignore crate behavior or just simple check.

                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

                // Check custom ignore config
                if ignore_matcher
                    .as_ref()
                    .is_some_and(|m| m.matched(path, is_dir).is_ignore())
                {
                    continue;
                }

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
