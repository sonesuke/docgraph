use crate::parse::extract_blocks;
use crate::types::SpecBlock;
use crate::walk::find_markdown_files;
use std::fs;
use std::path::Path;

pub fn collect_workspace(root: &Path) -> Vec<SpecBlock> {
    let files = find_markdown_files(root);
    let mut all_blocks = Vec::new();
    
    for file_path in files {
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                let blocks = extract_blocks(&content, &file_path);
                all_blocks.extend(blocks);
            }
            Err(e) => {
                eprintln!("Failed to read file {:?}: {}", file_path, e);
            }
        }
    }
    all_blocks
}
