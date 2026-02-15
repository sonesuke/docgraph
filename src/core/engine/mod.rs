use crate::core::config::Config;
use crate::core::parser::ast;
use crate::core::types::SpecBlock;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityId {
    Node(usize),
    Relationship {
        from_idx: usize,
        to_idx: usize,
        rel: String,
    },
}

type Bindings = HashMap<String, EntityId>;

pub fn execute_query(query: &ast::Query, nodes: &[SpecBlock], config: &Config) -> QueryResult {
    // 1. Match patterns
    let mut bindings_list: Vec<Bindings> = vec![HashMap::new()];

    for pattern_part in &query.match_clause.patterns {
        let mut last_node_variable: Option<String> = None;

        for chain in &pattern_part.chains {
            match chain {
                ast::PatternChain::Node(node_pat) => {
                    // Ensure node has a variable for later reference (internal or explicit)
                    // If explicit, use it. If not, we still need to track it for next steps?
                    // Actually match_node_pattern binds explicit vars.
                    // If implicit (no var), we can't key it in `Bindings`.
                    // This is a limitation: Path traversal requires explicit variables or internal unique IDs.
                    // For this MVP, let's fallback to requiring variables for path start.

                    if let Some(ref v) = node_pat.variable {
                        last_node_variable = Some(v.clone());
                    }

                    bindings_list = match_node_pattern(node_pat, nodes, bindings_list);
                }
                ast::PatternChain::Relationship(rel_pat, node_pat) => {
                    if let Some(ref start_var) = last_node_variable {
                        bindings_list = match_relationship_pattern(
                            start_var,
                            rel_pat,
                            node_pat,
                            nodes,
                            config,
                            bindings_list,
                        );

                        if let Some(ref v) = node_pat.variable {
                            last_node_variable = Some(v.clone());
                        }
                    } else {
                        // Error or panic? For MVP, skip if start node has no variable.
                        // Ideally we should auto-generate variables during parsing or here.
                        eprintln!(
                            "Warning: Relationship pattern requires start node to have a variable."
                        );
                    }
                }
            }
        }
    }

    // 2. Filter with WHERE
    if let Some(where_clause) = &query.where_clause {
        bindings_list.retain(|bindings| {
            evaluate_expression(&where_clause.expression, bindings, nodes, config)
        });
    }

    // 3. Project with RETURN
    let mut expanded_columns = Vec::new();
    let mut item_projections = Vec::new(); // Store closure or flag to know how to expand each item

    for item in &query.return_clause.items {
        match &item.expression {
            ast::Expression::Comparison(comp)
                if comp.operator.is_none()
                    && comp.right.is_none()
                    && comp.left.property.is_none()
                    && item.alias.is_none() =>
            {
                // Expand node variable (only if NO alias and NO property)
                let var = &comp.left.variable;
                expanded_columns.push(format!("{}.id", var));
                expanded_columns.push(format!("{}.type", var));
                expanded_columns.push(format!("{}.name", var));
                expanded_columns.push(format!("{}.file", var));
                expanded_columns.push(format!("{}.line", var));
                expanded_columns.push(format!("{}.content", var));
                item_projections.push(Projection::Node(var.clone()));
            }
            _ => {
                // Single column (with or without alias)
                if let Some(ref alias) = item.alias {
                    expanded_columns.push(alias.clone());
                } else {
                    match &item.expression {
                        ast::Expression::Comparison(comp)
                            if comp.operator.is_none() && comp.right.is_none() =>
                        {
                            if let Some(ref prop) = comp.left.property {
                                expanded_columns.push(format!("{}.{}", comp.left.variable, prop));
                            } else {
                                expanded_columns.push(comp.left.variable.clone());
                            }
                        }
                        _ => expanded_columns.push("expression".to_string()),
                    }
                }
                item_projections.push(Projection::Single(item.expression.clone()));
            }
        }
    }

    let mut rows = Vec::new();
    for bindings in bindings_list {
        let mut row = Vec::new();
        for proj in &item_projections {
            match proj {
                Projection::Single(expr) => {
                    row.push(evaluate_expression_value(expr, &bindings, nodes, config));
                }
                Projection::Node(var) => {
                    if let Some(EntityId::Node(idx)) = bindings.get(var) {
                        let node = &nodes[*idx];
                        row.push(node.id.clone());
                        row.push(node.node_type.clone());
                        row.push(node.name.clone().unwrap_or_else(|| "null".to_string()));
                        row.push(node.file_path.to_string_lossy().to_string());
                        row.push(node.line_start.to_string());
                        row.push(node.content.clone());
                    } else {
                        for _ in 0..6 {
                            row.push("null".to_string());
                        }
                    }
                }
            }
        }
        rows.push(row);
    }

    QueryResult {
        columns: expanded_columns,
        rows,
    }
}

enum Projection {
    Single(ast::Expression),
    Node(String),
}

fn match_node_pattern(
    node_pat: &ast::NodePattern,
    nodes: &[SpecBlock],
    current_bindings: Vec<Bindings>,
) -> Vec<Bindings> {
    let mut next_bindings = Vec::new();

    for bindings in current_bindings {
        for (i, node) in nodes.iter().enumerate() {
            // Check labels
            let label_match = if node_pat.labels.is_empty() {
                true
            } else {
                node_pat.labels.contains(&node.node_type)
            };

            if !label_match {
                continue;
            }

            // Bind variable
            if let Some(ref var) = node_pat.variable {
                if let Some(entity) = bindings.get(var) {
                    if let EntityId::Node(prev_idx) = entity {
                        if *prev_idx != i {
                            continue;
                        }
                        next_bindings.push(bindings.clone());
                    } else {
                        // Var already bound to non-node?
                        continue;
                    }
                } else {
                    let mut new_bindings = bindings.clone();
                    new_bindings.insert(var.clone(), EntityId::Node(i));
                    next_bindings.push(new_bindings);
                }
            } else {
                next_bindings.push(bindings.clone());
            }
        }
    }
    next_bindings
}

fn match_relationship_pattern(
    start_node_var: &str,
    rel_pat: &ast::RelationshipPattern,
    end_node_pat: &ast::NodePattern,
    nodes: &[SpecBlock],
    config: &Config,
    current_bindings: Vec<Bindings>,
) -> Vec<Bindings> {
    let mut next_bindings = Vec::new();

    // Build adjacency maps with rel type
    // key: source_idx, value: (target_idx, rel)
    let mut forward_adj: HashMap<usize, Vec<(usize, String)>> = HashMap::new();
    let mut backward_adj: HashMap<usize, Vec<(usize, String)>> = HashMap::new();

    for (idx, node) in nodes.iter().enumerate() {
        for edge in &node.edges {
            if let Some(target_idx) = nodes.iter().position(|n| n.id == edge.id) {
                let target_node = &nodes[target_idx];
                // Find rel from docgraph.toml
                let rel = find_relationship_rel(config, &node.node_type, &target_node.node_type);

                forward_adj
                    .entry(idx)
                    .or_default()
                    .push((target_idx, rel.clone()));
                backward_adj.entry(target_idx).or_default().push((idx, rel));
            }
        }
    }

    let min_hops = rel_pat.range.as_ref().and_then(|r| r.start).unwrap_or(1);
    let max_hops = rel_pat.range.as_ref().and_then(|r| r.end).unwrap_or(1);

    for bindings in current_bindings {
        if let Some(EntityId::Node(start_idx)) = bindings.get(start_node_var) {
            let start_idx = *start_idx;
            // BFS for reachability
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((start_idx, 0, None::<String>)); // (curr, dist, last_rel)

            let mut visited = std::collections::HashSet::new();
            visited.insert((start_idx, 0));

            while let Some((curr, dist, last_rel)) = queue.pop_front() {
                if dist > max_hops {
                    continue;
                }

                if dist >= min_hops && dist > 0 {
                    // Check rel_type if specified
                    let rel_match = if let Some(ref target_rel_type) = rel_pat.rel_type {
                        if let Some(ref actual_rel) = last_rel {
                            actual_rel == target_rel_type
                        } else {
                            false
                        }
                    } else {
                        true
                    };

                    if rel_match {
                        // Check if current node matches end_node_pat
                        let node = &nodes[curr];
                        let label_match = if end_node_pat.labels.is_empty() {
                            true
                        } else {
                            end_node_pat.labels.contains(&node.node_type)
                        };

                        if label_match {
                            let mut new_bindings = bindings.clone();

                            // Bind relationship variable if present (only for length 1 for now)
                            // Cypher behavior: (n)-[r*1..2]->(m) makes r a list.
                            // Our engine is MVP, let's only bind r if dist == 1.
                            if dist == 1
                                && let Some(ref r_var) = rel_pat.variable
                                && let Some(ref rel_val) = last_rel
                            {
                                new_bindings.insert(
                                    r_var.clone(),
                                    EntityId::Relationship {
                                        from_idx: start_idx,
                                        to_idx: curr,
                                        rel: rel_val.clone(),
                                    },
                                );
                            }

                            // Bind end variable
                            if let Some(ref var) = end_node_pat.variable {
                                if let Some(EntityId::Node(prev_idx)) = bindings.get(var) {
                                    if *prev_idx == curr {
                                        next_bindings.push(new_bindings);
                                    }
                                } else {
                                    new_bindings.insert(var.clone(), EntityId::Node(curr));
                                    next_bindings.push(new_bindings);
                                }
                            } else {
                                next_bindings.push(new_bindings);
                            }
                        }
                    }
                }

                // Continue traversal
                if dist < max_hops {
                    let mut neighbors = Vec::new();
                    match rel_pat.direction {
                        ast::Direction::Right => {
                            if let Some(n) = forward_adj.get(&curr) {
                                neighbors.extend(n);
                            }
                        }
                        ast::Direction::Left => {
                            if let Some(n) = backward_adj.get(&curr) {
                                neighbors.extend(n);
                            }
                        }
                        ast::Direction::Both => {
                            if let Some(n) = forward_adj.get(&curr) {
                                neighbors.extend(n);
                            }
                            if let Some(n) = backward_adj.get(&curr) {
                                neighbors.extend(n);
                            }
                        }
                    }

                    for (next, ctx) in neighbors {
                        if !visited.contains(&(*next, dist + 1)) {
                            visited.insert((*next, dist + 1));
                            queue.push_back((*next, dist + 1, Some(ctx.clone())));
                        }
                    }
                }
            }
        }
    }

    next_bindings
}

fn find_relationship_rel(config: &Config, from_type: &str, to_type: &str) -> String {
    if let Some(node_conf) = config.nodes.get(from_type) {
        for rule in &node_conf.rules {
            if rule.dir == "to"
                && rule.targets.contains(&to_type.to_string())
                && let Some(ref rel_val) = rule.rel
            {
                return rel_val.clone();
            }
        }
    }
    if let Some(node_conf) = config.nodes.get(to_type) {
        for rule in &node_conf.rules {
            if rule.dir == "from"
                && rule.targets.contains(&from_type.to_string())
                && let Some(ref rel_val) = rule.rel
            {
                return rel_val.clone();
            }
        }
    }
    "references".to_string()
}

fn evaluate_expression(
    expr: &ast::Expression,
    bindings: &Bindings,
    nodes: &[SpecBlock],
    config: &Config,
) -> bool {
    match expr {
        ast::Expression::And(exprs) => exprs
            .iter()
            .all(|e| evaluate_expression(e, bindings, nodes, config)),
        ast::Expression::Or(exprs) => exprs
            .iter()
            .any(|e| evaluate_expression(e, bindings, nodes, config)),
        ast::Expression::Comparison(comp) => {
            let left_val = evaluate_property_or_variable(&comp.left, bindings, nodes, config);
            if let Some(right_term) = &comp.right {
                let right_val = match right_term {
                    ast::Term::Literal(lit) => match lit {
                        ast::Literal::String(s) => s.clone(),
                        ast::Literal::Number(n) => n.to_string(),
                    },
                    ast::Term::PropertyOrVariable(pv) => {
                        evaluate_property_or_variable(pv, bindings, nodes, config)
                    }
                };

                if let Some(op) = &comp.operator {
                    match op {
                        ast::ComparisonOperator::Eq => left_val == right_val,
                        ast::ComparisonOperator::NotEq => left_val != right_val,
                        ast::ComparisonOperator::Contains => left_val.contains(&right_val),
                        ast::ComparisonOperator::Lt => left_val < right_val,
                        ast::ComparisonOperator::Gt => left_val > right_val,
                        ast::ComparisonOperator::LtEq => left_val <= right_val,
                        ast::ComparisonOperator::GtEq => left_val >= right_val,
                    }
                } else {
                    !left_val.is_empty() && left_val != "null"
                }
            } else {
                !left_val.is_empty() && left_val != "null"
            }
        }
    }
}

fn evaluate_expression_value(
    expr: &ast::Expression,
    bindings: &Bindings,
    nodes: &[SpecBlock],
    config: &Config,
) -> String {
    match expr {
        ast::Expression::Comparison(comp) => {
            if comp.operator.is_none() && comp.right.is_none() {
                evaluate_property_or_variable(&comp.left, bindings, nodes, config)
            } else {
                evaluate_expression(expr, bindings, nodes, config).to_string()
            }
        }
        _ => "complex_expr".to_string(),
    }
}

fn evaluate_property_or_variable(
    pv: &ast::PropertyOrVariable,
    bindings: &Bindings,
    nodes: &[SpecBlock],
    _config: &Config,
) -> String {
    if let Some(entity) = bindings.get(&pv.variable) {
        match entity {
            EntityId::Node(idx) => {
                let node = &nodes[*idx];
                if let Some(ref prop) = pv.property {
                    match prop.as_str() {
                        "id" => node.id.clone(),
                        "node_type" => node.node_type.clone(),
                        "type" => node.node_type.clone(),
                        "name" => node.name.clone().unwrap_or_else(|| "null".to_string()),
                        "file" => node.file_path.to_string_lossy().to_string(),
                        "line" => node.line_start.to_string(),
                        "content" => node.content.clone(),
                        _ => "null".to_string(),
                    }
                } else {
                    node.id.clone()
                }
            }
            EntityId::Relationship { rel, .. } => {
                if let Some(ref prop) = pv.property {
                    match prop.as_str() {
                        "type" => rel.clone(),
                        _ => "null".to_string(),
                    }
                } else {
                    rel.clone()
                }
            }
        }
    } else {
        "null".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::EdgeUse;
    use crate::core::types::SpecBlock;
    use std::path::PathBuf;

    fn mock_nodes() -> Vec<SpecBlock> {
        // UC01 -> FR01 -> MOD01
        vec![
            SpecBlock {
                id: "UC_001".to_string(),
                node_type: "UC".to_string(),
                name: Some("User Login".to_string()),
                file_path: PathBuf::from("test.md"),
                edges: vec![EdgeUse {
                    id: "FR_001".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            SpecBlock {
                id: "FR_001".to_string(),
                node_type: "FR".to_string(),
                name: Some("Authentication".to_string()),
                file_path: PathBuf::from("test.md"),
                edges: vec![EdgeUse {
                    id: "MOD_001".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            SpecBlock {
                id: "MOD_001".to_string(),
                node_type: "MOD".to_string(),
                name: Some("AuthModule".to_string()),
                file_path: PathBuf::from("test.md"),
                ..Default::default()
            },
        ]
    }

    #[test]
    fn test_execute_range_query() {
        let q = crate::core::parser::parse_query("MATCH (u:UC)-[*1..2]->(m:MOD) RETURN u.id, m.id")
            .unwrap();
        let nodes = mock_nodes();
        let config = Config::default();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "UC_001");
        assert_eq!(result.rows[0][1], "MOD_001");
    }

    #[test]
    fn test_execute_match_all() {
        let q = crate::core::parser::parse_query("MATCH (n) RETURN n.id").unwrap();
        let nodes = mock_nodes();
        let config = Config::default();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 3);
    }

    #[test]
    fn test_execute_match_label() {
        let q = crate::core::parser::parse_query("MATCH (n:UC) RETURN n.id").unwrap();
        let nodes = mock_nodes();
        let config = Config::default();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "UC_001");
    }

    #[test]
    fn test_execute_where() {
        let q = crate::core::parser::parse_query("MATCH (n) WHERE n.id = \"FR_001\" RETURN n.id")
            .unwrap();
        let nodes = mock_nodes();
        let config = Config::default();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "FR_001");
    }

    #[test]
    fn test_execute_property_access() {
        let q = crate::core::parser::parse_query("MATCH (n:UC) RETURN n.name, n.file").unwrap();
        let nodes = mock_nodes();
        let config = Config::default();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "User Login");
        assert_eq!(result.rows[0][1], "test.md");
    }

    #[test]
    fn test_execute_relationship() {
        let q =
            crate::core::parser::parse_query("MATCH (u:UC)-[]->(f:FR) RETURN u.id, f.id").unwrap();
        let nodes = mock_nodes();
        let config = Config::default();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "UC_001");
        assert_eq!(result.rows[0][1], "FR_001");
    }

    #[test]
    fn test_execute_where_operators() {
        let nodes = mock_nodes();

        let config = Config::default();
        // Not Equal <>
        let q = crate::core::parser::parse_query("MATCH (n) WHERE n.id <> \"UC_001\" RETURN n.id")
            .unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 2);
        assert!(!result.rows.iter().any(|r| r[0] == "UC_001"));

        // Contains
        let q = crate::core::parser::parse_query(
            "MATCH (n) WHERE n.name CONTAINS \"Login\" RETURN n.id",
        )
        .unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "UC_001");

        // Greater Than >
        // (Alphabetical comparison for strings)
        let q = crate::core::parser::parse_query("MATCH (n) WHERE n.id > \"MOD_001\" RETURN n.id")
            .unwrap();
        let result = execute_query(&q, &nodes, &config);
        // UC_001 > MOD_001 is true? 'U' > 'M'. Yes.
        // FR_001 > MOD_001? 'F' > 'M'? No.
        assert!(result.rows.iter().any(|r| r[0] == "UC_001"));
        assert!(!result.rows.iter().any(|r| r[0] == "FR_001"));
    }

    #[test]
    fn test_execute_where_logical() {
        let nodes = mock_nodes();

        let config = Config::default();
        // AND
        let q = crate::core::parser::parse_query(
            "MATCH (n) WHERE n.node_type = \"UC\" AND n.name CONTAINS \"Login\" RETURN n.id",
        )
        .unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "UC_001");

        // OR
        let q = crate::core::parser::parse_query(
            "MATCH (n) WHERE n.id = \"UC_001\" OR n.id = \"FR_001\" RETURN n.id",
        )
        .unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 2);
    }

    #[test]
    fn test_execute_match_multiple_labels() {
        let nodes = mock_nodes();

        let config = Config::default();
        // (n:UC:FR) -> Should be OR (UC or FR) based on current implementation
        // Parser supports multi-labels. Engine `match_node_pattern` uses `labels.contains(&node.node_type)`.
        let q = crate::core::parser::parse_query("MATCH (n:UC:FR) RETURN n.id").unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 2);
        assert!(result.rows.iter().any(|r| r[0] == "UC_001"));
        assert!(result.rows.iter().any(|r| r[0] == "FR_001"));
        assert!(!result.rows.iter().any(|r| r[0] == "MOD_001"));
    }

    #[test]
    fn test_execute_node_expansion() {
        let nodes = mock_nodes();
        let config = Config::default();
        let q = crate::core::parser::parse_query("MATCH (n:UC) RETURN n").unwrap();
        let result = execute_query(&q, &nodes, &config);

        // Should have 6 columns
        assert_eq!(result.columns.len(), 6);
        assert_eq!(result.columns[0], "n.id");
        assert_eq!(result.columns[1], "n.type");
        assert_eq!(result.columns[2], "n.name");
        assert_eq!(result.columns[3], "n.file");
        assert_eq!(result.columns[4], "n.line");
        assert_eq!(result.columns[5], "n.content");

        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "UC_001");
        assert_eq!(result.rows[0][1], "UC");
        assert_eq!(result.rows[0][2], "User Login");
    }

    #[test]
    fn test_execute_relationship_type_and_rel() {
        use crate::core::config::{NodeConfig, RuleConfig};
        let nodes = mock_nodes();
        let mut config = Config::default();

        // Setup rules in config
        // UC uses FR
        let mut uc_rules = NodeConfig::default();
        uc_rules.rules.push(RuleConfig {
            dir: "to".to_string(),
            targets: vec!["FR".to_string()],
            rel: Some("uses".to_string()),
            ..Default::default()
        });
        config.nodes.insert("UC".to_string(), uc_rules);

        // FR implemented by MOD
        let mut fr_rules = NodeConfig::default();
        fr_rules.rules.push(RuleConfig {
            dir: "to".to_string(),
            targets: vec!["MOD".to_string()],
            rel: Some("implemented_by".to_string()),
            ..Default::default()
        });
        config.nodes.insert("FR".to_string(), fr_rules);

        // Test 1: Query with relationship variable and type
        let q =
            crate::core::parser::parse_query("MATCH (u:UC)-[r]->(f:FR) RETURN u.id, r.type, f.id")
                .unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "UC_001");
        assert_eq!(result.rows[0][1], "uses");
        assert_eq!(result.rows[0][2], "FR_001");

        // Test 2: Filtering by relationship type
        let q =
            crate::core::parser::parse_query("MATCH (u:UC)-[r:uses]->(f:FR) RETURN f.id").unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "FR_001");

        let q =
            crate::core::parser::parse_query("MATCH (u:UC)-[r:other]->(f:FR) RETURN f.id").unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 0);
    }

    #[test]
    fn test_execute_alias_and_ops() {
        let nodes = mock_nodes();
        let config = Config::default();

        // Testing Alias and Comparison Ops
        let q = crate::core::parser::parse_query(
            "MATCH (n:UC) RETURN n.id AS identifier, n.name AS name",
        )
        .unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.columns[0], "identifier");
        assert_eq!(result.columns[1], "name");

        // Comparison ops (Number literal - though our values are strings usually)
        let q =
            crate::core::parser::parse_query("MATCH (n) WHERE n.id < \"ZZ\" RETURN n.id").unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert!(!result.rows.is_empty());
    }

    #[test]
    fn test_execute_reverse_and_both_directions() {
        let nodes = mock_nodes();
        let mut config = Config::default();
        use crate::core::config::{NodeConfig, RuleConfig};

        // UC -> FR
        let mut uc_rules = NodeConfig::default();
        uc_rules.rules.push(RuleConfig {
            dir: "to".to_string(),
            targets: vec!["FR".to_string()],
            rel: Some("uses".to_string()),
            ..Default::default()
        });
        config.nodes.insert("UC".to_string(), uc_rules);

        // Reverse matching: (f:FR)<-[]-(u:UC)
        let q = crate::core::parser::parse_query("MATCH (f:FR)<-[r]-(u:UC) RETURN r.type").unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "uses");

        // Undirected matching: (u:UC)-[]-(f:FR)
        let q = crate::core::parser::parse_query("MATCH (u:UC)-[r]-(f:FR) RETURN r.type").unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "uses");

        // Undirected from other side
        let q = crate::core::parser::parse_query("MATCH (f:FR)-[r]-(u:UC) RETURN r.type").unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], "uses");
    }

    #[test]
    fn test_execute_match_mismatch() {
        let nodes = mock_nodes();
        let config = Config::default();

        // Variable already bound to different node
        let q = crate::core::parser::parse_query("MATCH (n:UC), (m:FR) WHERE n = m RETURN n.id")
            .unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 0);

        // End node label mismatch in relationship
        let q = crate::core::parser::parse_query("MATCH (u:UC)-[]->(f:MOD) RETURN u.id").unwrap();
        let result = execute_query(&q, &nodes, &config);
        assert_eq!(result.rows.len(), 0);
    }
}
