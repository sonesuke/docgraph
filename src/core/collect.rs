use crate::core::parse::extract_all;
use crate::core::types::{RefUse, SpecBlock};
use crate::core::walk::find_markdown_files;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Collect all SpecBlocks and standalone RefUses from the workspace
pub fn collect_workspace_all(
    root: &Path,
    ignore_patterns: &[String],
    overrides: Option<&HashMap<PathBuf, String>>,
) -> (Vec<SpecBlock>, Vec<RefUse>) {
    let files = find_markdown_files(root, ignore_patterns);
    let mut all_blocks = Vec::new();
    let mut all_refs = Vec::new();

    for file_path in files {
        // Canonicalize the path for lookup to match the keys in overrides
        let lookup_path = fs::canonicalize(&file_path).unwrap_or_else(|_| file_path.clone());

        let content_result = if let Some(map) = overrides
            && let Some(content) = map.get(lookup_path.as_path())
        {
            Ok(content.clone())
        } else {
            fs::read_to_string(&file_path)
        };

        match content_result {
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

        let (blocks, refs) = collect_workspace_all(dir.path(), &[], None);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].id, "ID-1");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].target_id, "ID-2");
    }
}
