
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{EdgeUse, RefUse, SpecBlock};

    #[test]
    fn test_references_from_definition() {
        let path = std::env::current_dir().unwrap().join("test.md");
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
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 9,
                    character: 0,
                }, // At definition of FR-01
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: ReferenceContext {
                include_declaration: true,
            },
        };

        let result = references(&blocks, &refs, params).unwrap();
        if let Some(locs) = result {
            assert_eq!(locs.len(), 1);
            assert_eq!(locs[0].range.start.line, 1); // Edge at line 2 (index 1)
        } else {
            panic!("Expected references locations");
        }
    }

    #[test]
    fn test_references_from_edge() {
        let path = std::env::current_dir().unwrap().join("test.md");
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
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 1,
                    character: 7,
                }, // Inside edge reference
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
        assert_eq!(locs.len(), 1);
    }

    #[test]
    fn test_references_with_standalone_refs() {
        let path = std::env::current_dir().unwrap().join("test.md");
        let path2 = std::env::current_dir().unwrap().join("other.md");

        let blocks = vec![SpecBlock {
            id: "FR-01".to_string(),
            file_path: path.clone(),
            line_start: 10,
            line_end: 12,
            ..Default::default()
        }];

        let refs = vec![RefUse {
            target_id: "FR-01".to_string(),
            file_path: path2.clone(),
            line: 5,
            col_start: 10,
            col_end: 15,
        }];

        let uri = Url::from_file_path(&path).unwrap();
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 9,
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
        assert!(result.is_some());
        let locs = result.unwrap();
        assert_eq!(locs.len(), 1); // One standalone ref
    }

    #[test]
    fn test_references_no_match() {
        let path = std::env::current_dir().unwrap().join("test.md");
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
