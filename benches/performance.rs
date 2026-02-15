use criterion::{Criterion, criterion_group, criterion_main};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;

use docgraph::core::collect::collect_workspace_all;
use docgraph::core::config::Config;
use docgraph::core::engine::execute_query;
use docgraph::core::lint::check_workspace;
use docgraph::core::parser::parse_query;

/// Generate test workspace with specified number of files and nodes
fn generate_test_workspace(dir: &Path, num_files: usize, nodes_per_file: usize) {
    for file_idx in 0..num_files {
        let file_path = dir.join(format!("doc_{:04}.md", file_idx));
        let mut file = File::create(&file_path).expect("Failed to create file");

        writeln!(file, "# Document {}", file_idx).expect("Failed to write");
        writeln!(file).expect("Failed to write");

        for node_idx in 0..nodes_per_file {
            let id = format!("FR-{:04}-{:03}", file_idx, node_idx);
            writeln!(file, "<a id=\"{}\"></a>", id).expect("Failed to write");
            writeln!(file).expect("Failed to write");
            writeln!(file, "## {} Requirement {}", id, node_idx).expect("Failed to write");
            writeln!(file).expect("Failed to write");
            writeln!(
                file,
                "Description for requirement {} in document {}.",
                node_idx, file_idx
            )
            .expect("Failed to write");
            writeln!(file).expect("Failed to write");

            // Add some references
            if node_idx > 0 {
                let prev_id = format!("FR-{:04}-{:03}", file_idx, node_idx - 1);
                writeln!(file, "References [{}](#{})", prev_id, prev_id).expect("Failed to write");
                writeln!(file).expect("Failed to write");
            }
        }
    }

    // Create minimal docgraph.toml
    let config_path = dir.join("docgraph.toml");
    let mut config_file = File::create(&config_path).expect("Failed to create config");
    writeln!(
        config_file,
        r#"[graph]
name = "Benchmark Test"
ignore = []

[node_types]
FR = {{ desc = "Functional Requirement" }}
"#
    )
    .expect("Failed to write config");
}

fn bench_collect_1000_nodes(c: &mut Criterion) {
    let dir = tempdir().expect("Failed to create temp dir");
    // 100 files × 10 nodes = 1000 nodes
    generate_test_workspace(dir.path(), 100, 10);

    c.bench_function("collect_1000_nodes_100_files", |b| {
        b.iter(|| {
            let (blocks, _refs) = collect_workspace_all(dir.path(), &[], None);
            assert!(
                blocks.len() >= 1000,
                "Expected 1000+ nodes, got {}",
                blocks.len()
            );
        })
    });
}

fn bench_lint_1000_nodes(c: &mut Criterion) {
    let dir = tempdir().expect("Failed to create temp dir");
    // 100 files × 10 nodes = 1000 nodes
    generate_test_workspace(dir.path(), 100, 10);

    let config = Config::load(dir.path()).expect("Failed to load config");

    c.bench_function("lint_1000_nodes_100_files", |b| {
        b.iter(|| {
            let diagnostics = check_workspace(dir.path(), false, None, true, &config, None);
            // Just run the check, verify diagnostic count to ensure rules are running
            assert!(!diagnostics.is_empty());
        })
    });
}

fn bench_query_match_label(c: &mut Criterion) {
    let dir = tempdir().expect("Failed to create temp dir");
    generate_test_workspace(dir.path(), 100, 10);

    let (nodes, _refs) = collect_workspace_all(dir.path(), &[], None);
    let config = Config::load(dir.path()).expect("Failed to load config");
    let query = parse_query("MATCH (n:FR) RETURN n.id").expect("Failed to parse query");

    c.bench_function("query_match_label_1000_nodes", |b| {
        b.iter(|| {
            let result = execute_query(&query, &nodes, &config);
            assert!(
                result.rows.len() >= 1000,
                "Expected 1000+ rows, got {}",
                result.rows.len()
            );
        })
    });
}

fn bench_query_where_filter(c: &mut Criterion) {
    let dir = tempdir().expect("Failed to create temp dir");
    generate_test_workspace(dir.path(), 100, 10);

    let (nodes, _refs) = collect_workspace_all(dir.path(), &[], None);
    let config = Config::load(dir.path()).expect("Failed to load config");
    let query =
        parse_query("MATCH (n) WHERE n.type = \"FR\" RETURN n.id").expect("Failed to parse query");

    c.bench_function("query_where_filter_1000_nodes", |b| {
        b.iter(|| {
            let result = execute_query(&query, &nodes, &config);
            assert!(
                result.rows.len() >= 1000,
                "Expected 1000+ rows, got {}",
                result.rows.len()
            );
        })
    });
}

fn bench_query_relationship(c: &mut Criterion) {
    let dir = tempdir().expect("Failed to create temp dir");
    generate_test_workspace(dir.path(), 100, 10);

    let (nodes, _refs) = collect_workspace_all(dir.path(), &[], None);
    let config = Config::load(dir.path()).expect("Failed to load config");
    let query =
        parse_query("MATCH (a:FR)-[]->(b:FR) RETURN a.id, b.id").expect("Failed to parse query");

    c.bench_function("query_relationship_1000_nodes", |b| {
        b.iter(|| {
            let result = execute_query(&query, &nodes, &config);
            assert!(
                !result.rows.is_empty(),
                "Expected some relationship results"
            );
        })
    });
}

criterion_group!(
    benches,
    bench_collect_1000_nodes,
    bench_lint_1000_nodes,
    bench_query_match_label,
    bench_query_where_filter,
    bench_query_relationship
);
criterion_main!(benches);
