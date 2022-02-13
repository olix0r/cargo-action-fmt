#![deny(warnings, rust_2018_idioms)]

use anyhow::Result;
use clap::Parser;
use serde_json as json;
use std::path::PathBuf;

/// Converts cargo check (and clippy) JSON output to the GitHub Action error format
#[derive(Debug, Parser)]
#[clap(version)]
struct Args {
    path: Option<PathBuf>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "reason", rename_all = "kebab-case")]
enum CargoMessage {
    CompilerArtifact(json::Value),
    BuildScriptExecuted(json::Value),
    CompilerMessage { message: CompilerMessage },
    BuildFinished { success: bool },
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
struct CompilerMessage {
    rendered: String,
    code: Option<json::Value>,
    level: String,
    spans: Vec<CompilerMessageSpan>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
struct CompilerMessageSpan {
    column_start: usize,
    column_end: usize,
    file_name: String,
    line_start: usize,
    line_end: usize,
}

fn main() -> Result<()> {
    let Args { path } = Args::parse();

    let msgs = if let Some(p) = path {
        let f = std::fs::File::open(p)?;
        json::Deserializer::from_reader(f)
            .into_iter::<CargoMessage>()
            .collect::<Result<Vec<_>, _>>()?
    } else {
        json::Deserializer::from_reader(std::io::stdin())
            .into_iter::<CargoMessage>()
            .collect::<Result<Vec<_>, _>>()?
    };

    for msg in msgs {
        match msg {
            CargoMessage::CompilerArtifact(_) => {}
            CargoMessage::BuildScriptExecuted(_) => {}
            CargoMessage::CompilerMessage { message } => {
                if message.code.is_some() {
                    let msg = encode_newlines(message.rendered);
                    for span in message.spans.into_iter() {
                        println!(
                            "::{} file={},line={},endLine={},col={},endColumn={}::{}",
                            message.level,
                            span.file_name,
                            span.line_start,
                            span.line_end,
                            span.column_start,
                            span.column_end,
                            msg,
                        );
                    }
                }
            }
            CargoMessage::BuildFinished { success } => {
                if !success {
                    anyhow::bail!("command failed")
                }
            }
        }
    }

    Ok(())
}

fn encode_newlines(orig: String) -> String {
    orig.replace('\n', "%0A")
}
