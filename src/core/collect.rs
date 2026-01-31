use crate::core::parse::extract_all;
use crate::core::types::{RefUse, SpecBlock};
use crate::core::walk::find_markdown_files;
use std::fs;
use std::path::Path;

/// Collect all SpecBlocks and standalone RefUses from the workspace
pub fn collect_workspace_all(
    root: &Path,
    ignore_patterns: &[String],
) -> (Vec<SpecBlock>, Vec<RefUse>) {
    let files = find_markdown_files(root, ignore_patterns);
    let mut all_blocks = Vec::new();
    let mut all_refs = Vec::new();

    for file_path in files {
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                let (blocks, refs) = extract_all(&content, &file_path);
                all_blocks.extend(blocks);
                all_refs.extend(refs);
            }
            Err(_e) => {} // Silence read errors in core
        }
    }
    (all_blocks, all_refs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_collect_workspace_all() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.md");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "<a id=\"ID-1\"></a>\n# Heading\n[Ref](#ID-2)").unwrap();

        let (blocks, refs) = collect_workspace_all(dir.path(), &[]);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].id, "ID-1");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].target_id, "ID-2");
    }
}
