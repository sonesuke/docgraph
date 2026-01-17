use crate::parse::extract_all;
use crate::types::{RefUse, SpecBlock};
use crate::walk::find_markdown_files;
use std::fs;
use std::path::Path;

/// Collect all SpecBlocks and standalone RefUses from the workspace
pub fn collect_workspace_all(root: &Path) -> (Vec<SpecBlock>, Vec<RefUse>) {
    let files = find_markdown_files(root);
    let mut all_blocks = Vec::new();
    let mut all_refs = Vec::new();

    for file_path in files {
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                let (blocks, refs) = extract_all(&content, &file_path);
                all_blocks.extend(blocks);
                all_refs.extend(refs);
            }
            Err(e) => {
                eprintln!("Failed to read file {:?}: {}", file_path, e);
            }
        }
    }
    (all_blocks, all_refs)
}
