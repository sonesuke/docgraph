use crate::cli::args::OutputFormat;
use crate::core::{collect, config, engine};
use anyhow::Context;
use comfy_table::Table;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn handle_query(query_str: String, format: OutputFormat, path: PathBuf) -> ExitCode {
    match try_query(query_str, format, path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn try_query(query_str: String, format: OutputFormat, path: PathBuf) -> anyhow::Result<ExitCode> {
    let config = config::Config::load(&path).context("failed to load docgraph.toml")?;
    let (blocks, _) = collect::collect_workspace_all(&path, &config.graph.ignore, None);

    let query = crate::core::parser::parse_query(&query_str).context("failed to parse query")?;
    let result = engine::execute_query(&query, &blocks);

    match format {
        OutputFormat::Table => {
            if result.rows.is_empty() {
                println!("No results found.");
            } else {
                let mut table = Table::new();
                table.load_preset(comfy_table::presets::UTF8_FULL);
                table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
                table.set_header(&result.columns);

                for row in result.rows {
                    table.add_row(row);
                }

                println!("{table}");
            }
        }
        OutputFormat::Json => {
            // Convert to list of objects (dictionaries)
            let mut json_rows = Vec::new();
            for row in result.rows {
                let mut obj = serde_json::Map::new();
                for (i, col_name) in result.columns.iter().enumerate() {
                     if let Some(val) = row.get(i) {
                         obj.insert(col_name.clone(), serde_json::Value::String(val.clone()));
                     }
                }
                json_rows.push(serde_json::Value::Object(obj));
            }
            let json_out = serde_json::to_string_pretty(&json_rows).context("failed to serialize to JSON")?;
            println!("{}", json_out);
        }
    }

    Ok(ExitCode::SUCCESS)
}
