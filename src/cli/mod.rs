pub mod args;
pub mod handlers;

use crate::lsp;
use args::{Cli, Commands};
use clap::Parser;
use std::process::ExitCode;

pub async fn run() -> ExitCode {
    let args = Cli::parse();

    match args.command {
        Commands::Lsp => {
            lsp::run_server().await;
            ExitCode::SUCCESS
        }
        Commands::Check {
            path,
            json,
            fix,
            rule,
        } => handlers::check::handle_check(path, json, fix, rule),
        Commands::Fmt { path, rule } => handlers::check::handle_fmt(path, rule),
        Commands::Rule { rule } => handlers::rule::handle_rule(rule),
        Commands::Graph { path } => handlers::graph::handle_graph(path),
        Commands::List { query, path } => handlers::list::handle_list(query, path),
        Commands::Trace {
            from,
            to,
            path,
            direction,
        } => handlers::trace::handle_trace(from, to, path, direction),
        Commands::Describe { id, path } => handlers::describe::handle_describe(id, path),
    }
}
