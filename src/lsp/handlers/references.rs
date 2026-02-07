use anyhow::Result;
use lsp_types::*;
use url::Url;

pub fn references(
    blocks: &[crate::core::types::SpecBlock],
    refs: &[crate::core::types::RefUse],
    params: ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let line = position.line as usize + 1;
    let col = position.character as usize + 1;

    if let Ok(path) = uri.to_file_path() {
        let path = std::fs::canonicalize(&path).unwrap_or(path);
        // Delegate to Core Logic
        if let Some(target_id) =
            crate::core::locate::locate_id_at_position(blocks, refs, &path, line, col)
        {
            let results = crate::core::locate::find_references_msg(blocks, refs, &target_id);
            let mut locations = Vec::new();

            for res in results {
                if let Ok(u) = Url::from_file_path(&res.file_path) {
                    locations.push(Location {
                        uri: u,
                        range: Range {
                            start: Position {
                                line: res.range_start_line as u32 - 1,
                                character: res.range_start_col as u32 - 1,
                            },
                            end: Position {
                                line: res.range_end_line as u32 - 1,
                                character: res.range_end_col as u32 - 1,
                            },
                        },
                    });
                }
            }
            return Ok(Some(locations));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{EdgeUse, RefUse, SpecBlock};
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_references_from_definition() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Line 1: UC-01
        // Line 10: FR-01
        writeln!(temp_file, "<a id=\"UC-01\"></a>").unwrap();
        for _ in 0..8 {
            writeln!(temp_file).unwrap();
        } // filler
        writeln!(temp_file, "<a id=\"FR-01\"></a>").unwrap();
        writeln!(temp_file, "<a id=\"FR-01\"></a>").unwrap();
        let path = std::fs::canonicalize(temp_file.path()).unwrap();

        let blocks = vec![
            SpecBlock {
                id: "FR-01".to_string(),
                file_path: path.clone(),
                line_start: 10,
                line_end: 12,
                ..Default::default()
            },
            SpecBlock {
                id: "UC-01".to_string(),
                file_path: path.clone(),
                line_start: 1,
                line_end: 5,
                edges: vec![EdgeUse {
                    id: "FR-01".to_string(),
                    line: 2,
                    col_start: 5,
                    col_end: 10,
                    ..Default::default()
                }],
                ..Default::default()
            },
        ];
        let refs = vec![];

        let uri = Url::from_file_path(&path).unwrap();
        // FR-01 is at line 10. <a id="FR-01"></a>. col 8 starts 'F'.
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 9,      // 10 - 1
                    character: 7, // 8 - 1
                },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: ReferenceContext {
                include_declaration: true,
            },
        };

        let result = references(&blocks, &refs, params).unwrap();
        if let Some(locs) = result {
            // Should find:
            // 1. Edge in UC-01 (line 2)
            // 2. Definition itself (FR-01, line 10) - strictly speaking find_references_msg includes definition now

            // `find_references_msg` includes Definition, Edges, Refs.
            // Edge is at line 2.
            // Definition is at line 10.

            // Wait, my mock Edge says line 2. But the file content I wrote doesn't have an edge there.
            // core::locate logic for *referencing* (finding location) doesn't verify edge text on disk, it trusts SpecBlock edge locations.
            // BUT `locate_id_at_position` (the input) DOES verify disk content to find "what ID am I on?".

            // So provided I click on FR-01 definition correctly, it should find references.
            assert!(!locs.is_empty());
            // Check for the edge
            assert!(locs.iter().any(|l| l.range.start.line == 1));
        } else {
            panic!("Expected references locations");
        }
    }

    #[test]
    fn test_references_from_edge() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Line 1: <a id="UC-01"></a>
        // Line 2: Link to [FR-01]
        writeln!(temp_file, "<a id=\"UC-01\"></a>").unwrap();
        writeln!(temp_file, "Link to [FR-01]").unwrap();
        // 0123456789
        // FR-01 starts at col 9 (index 9, 1-based 10).
        // Let's adjust mock edge data to match.

        // ... filler ...
        for _ in 0..7 {
            writeln!(temp_file).unwrap();
        }
        writeln!(temp_file, "<a id=\"FR-01\"></a>").unwrap();
        let path = std::fs::canonicalize(temp_file.path()).unwrap();

        let blocks = vec![
            SpecBlock {
                id: "FR-01".to_string(),
                file_path: path.clone(),
                line_start: 10,
                line_end: 12,
                ..Default::default()
            },
            SpecBlock {
                id: "UC-01".to_string(),
                file_path: path.clone(),
                line_start: 1,
                line_end: 5,
                edges: vec![EdgeUse {
                    id: "FR-01".to_string(),
                    line: 2,
                    col_start: 10, // Adjusted to match file content above
                    col_end: 15,
                    ..Default::default()
                }],
                ..Default::default()
            },
        ];
        let refs = vec![];

        let uri = Url::from_file_path(&path).unwrap();
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 1,       // Line 2
                    character: 10, // Inside [FR-01]
                },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: ReferenceContext {
                include_declaration: true,
            },
        };

        // locate_id_at_position connects Edge location logic using "block.edges" iteration.
        // It DOES NOT verify disk content for Edges (Step 2 in locate_id_at_position), only Block Definitions (Step 1).
        // It trusts the EdgeUse struct for step 2.

        // Step 2 code:
        // for edge in &block.edges {
        //    if edge.line == line && col >= edge.col_start && col <= edge.col_end {

        let result = references(&blocks, &refs, params).unwrap();
        assert!(result.is_some());
        let locs = result.unwrap();
        assert!(!locs.is_empty());
    }

    #[test]
    fn test_references_with_standalone_refs() {
        let mut temp_file1 = NamedTempFile::new().unwrap();
        writeln!(temp_file1, "<a id=\"FR-01\"></a>").unwrap();
        let path = std::fs::canonicalize(temp_file1.path()).unwrap();

        let mut temp_file2 = NamedTempFile::new().unwrap();
        writeln!(temp_file2, "Ref to [FR-01]").unwrap();
        writeln!(temp_file2, "Ref to [FR-01]").unwrap();
        let path2 = std::fs::canonicalize(temp_file2.path()).unwrap();

        let blocks = vec![SpecBlock {
            id: "FR-01".to_string(),
            file_path: path.clone(),
            line_start: 1,
            line_end: 1,
            ..Default::default()
        }];

        let refs = vec![RefUse {
            target_id: "FR-01".to_string(),
            file_path: path2.clone(),
            line: 1,
            col_start: 9, // [FR-01] start at 9 if "Ref to [" is 8 chars?
            // "Ref to [" => 8 chars. 0-7.
            // [ is at 7 (1-based 8).
            // F is at 8 (1-based 9).
            col_end: 14,
        }];

        let uri = Url::from_file_path(&path).unwrap();
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 0,
                    character: 7, // On definition FR-01 (col 8)
                },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: ReferenceContext {
                include_declaration: true,
            },
        };

        let result = references(&blocks, &refs, params).unwrap();
        assert!(result.is_some());
        let locs = result.unwrap();
        assert!(!locs.is_empty());
        // Should find the standalone ref in path2
        assert!(
            locs.iter()
                .any(|l| l.uri.to_file_path() == Ok(path2.clone()))
        );
    }

    #[test]
    fn test_references_no_match() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Empty").unwrap();
        let path = temp_file.path().to_path_buf();
        let blocks = vec![];
        let refs = vec![];

        let uri = Url::from_file_path(&path).unwrap();
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 0,
                    character: 0,
                },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: ReferenceContext {
                include_declaration: true,
            },
        };

        let result = references(&blocks, &refs, params).unwrap();
        assert!(result.is_none());
    }
}
