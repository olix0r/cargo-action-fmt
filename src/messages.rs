// SPDX-License-Identifier: MIT

/// Represents a message written by Cargo, serialized by [`serde_json`].
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "reason", rename_all = "kebab-case")]
pub enum CargoMessage {
	/// Catches artifacts emitted by Cargo.
	CompilerArtifact(serde_json::Value),
	/// Catches a build script execution
	BuildScriptExecuted(serde_json::Value),
	/// The actual contents that we are interested in
	CompilerMessage {
		/// The contents of the actual message
		message: CompilerMessage,
	},
	/// Build is finished
	BuildFinished {
		/// Is `true` when the compilation succeeded, false otherwise
		success: bool,
	},
}

impl CargoMessage {
	/// Read from a path (or from `/dev/stdin`) and provide an iterable
	pub fn read(path: Option<&std::path::PathBuf>) -> anyhow::Result<Vec<Self>> {
		Ok(if let Some(path) = path {
			serde_json::Deserializer::from_reader(std::fs::File::open(path)?)
				.into_iter::<Self>()
				.collect::<Result<Vec<_>, _>>()?
		} else {
			serde_json::Deserializer::from_reader(std::io::stdin())
				.into_iter::<Self>()
				.collect::<Result<Vec<_>, _>>()?
		})
	}

	/// Evaluate a message emitted by Cargo and print it in GitHub Actions' format.
	pub fn evaluate(self) -> anyhow::Result<()> {
		match self {
			Self::CompilerArtifact(_) | Self::BuildScriptExecuted(_) => {},
			Self::CompilerMessage { message } => {
				if message.code.is_some() {
					// ? Encode newlines
					let msg = message.rendered.replace('\n', "%0A");
					for span in message.spans {
						println!(
							"::{} file={},line={},endLine={},col={},\
							 endColumn={}::{}",
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
			},
			Self::BuildFinished { success } => {
				if !success {
					anyhow::bail!("Build failed")
				}
			},
		}
		Ok(())
	}
}

/// Contains information about messages (that we are interested in) that Cargo emitted.
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct CompilerMessage {
	/// TODO
	rendered: String,
	/// TODO
	code:     Option<serde_json::Value>,
	level:    String,
	/// TODO
	spans:    Vec<CompilerMessageSpan>,
}

/// TODO
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
struct CompilerMessageSpan {
	/// TODO
	column_start: usize,
	/// TODO
	column_end:   usize,
	/// TODO
	file_name:    String,
	/// TODO
	line_start:   usize,
	/// TODO
	line_end:     usize,
}
