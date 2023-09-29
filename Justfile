# -----------------------------------------------
# ----  Just  -----------------------------------
# ----  https://github.com/casey/just  ----------
# -----------------------------------------------

set shell             := [ "bash", "-eu", "-o", "pipefail", "-c" ]
set dotenv-load       := false

CARGO                 := 'cargo'

# show this help message
default:
	just --list

# -----------------------------------------------
# ----  Build  ----------------------------------
# -----------------------------------------------

# build the binary
@build *arguments:
	{{ CARGO }} build --frozen {{  arguments  }}

# cleanup build files
@clean:
	{{ CARGO }} clean

# build the code documentation
@documentation arguments:
	{{ CARGO }} doc --no-deps --lib --document-private-items {{ arguments }}

# -----------------------------------------------
# ----  Format and Lint  ------------------------
# -----------------------------------------------

# format the code
@fmt:
	{{ CARGO }} fmt --message-format human

# check Rust code (common errors, documentation, etc.)
@check: fmt
	{{ CARGO }} check
	{{ CARGO }} clippy
	{{ CARGO }} fmt --check --message-format human
	{{ CARGO }} doc

# lint Rust code with clippy
@rslint: fmt
	{{ CARGO }} clippy --lib --all-features -- -D warnings
	{{ CARGO }} clippy --bin yourprojectname --all-features -- -D warnings

# -----------------------------------------------
# ----  Tests  ----------------------------------
# -----------------------------------------------

# run tests
@test *arguments:
	{{ CARGO }} test {{ arguments }}

# run tests with the `nexttest` runner
@nextest *arguments:
	{{ CARGO }} nextest run
