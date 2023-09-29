// SPDX-License-Identifier: MIT

// Preventing `unsafe` code in `main.rs` completely.
#![deny(unsafe_code)]
// Clippy lint target one. Enables all lints that are on by
// default (correctness, suspicious, style, complexity, perf).
#![deny(clippy::all)]
// Clippy lint target two. Enables lints which are rather strict
// or have occasional false positives.
#![deny(clippy::nursery)]
// Clippy lint target three. Enables new lints that are still
// under development
#![deny(clippy::pedantic)]
// Clippy lint target four. Enable lints for the cargo manifest
// file, a.k.a. Cargo.toml.
#![deny(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
// Clippy lint(s) target(s) five. Custom linting rules.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_docs_in_private_items)]
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![deny(rustdoc::all)]
// All other, generic lint targets that were not
// covered previously
#![deny(missing_debug_implementations)]

//! cargo-action-fmt
//!
//! Convert the output of cargo check (and more) to the
//! GitHub Actions comment format.

/// Actually responsible for the conversion of Cargo's outputs to GitHub's
/// format.
mod messages;

/// Holds the arguments that this binary can accept.
#[derive(clap::Parser, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
	/// When provided, read the messages from the file described by this path.
	#[arg(short, long)]
	pub path: Option<std::path::PathBuf>,
}

fn main() -> anyhow::Result<()> {
	let arguments = <Arguments as clap::Parser>::parse();

	for msg in messages::CargoMessage::read(arguments.path.as_ref())? {
		msg.evaluate()?;
	}

	Ok(())
}
