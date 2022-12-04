# `cargo-action-fmt`

Takes JSON-formatted `cargo check` (and `cargo clippy`) output and formats it
for GitHub actions.

## Examples

This tool can be used with a variety of `cargo` commands:

```shell
:; cargo check -q --message-format=json | cargo-action-fmt
```

```shell
:; cargo clippy -q --message-format=json | cargo-action-fmt
```

```shell
:; cargo doc --message-format=json | cargo-action-fmt
```

Note that this tool does **not** currently support `cargo fmt` or `cargo test`
output. However, you can invoke `cargo test` so that test compilation errors are
annotated properly:

```shell
:; cargo test --no-run --message-format=json | cargo-action-fmt
```

### GitHub Action

It's primarily intended to be used in a GitHub Actions workflow:

```yaml
  docs:
    runs-on: ubuntu-latest
    container: rust:slim
    steps:
      - uses: olix0r/cargo-action-fmt/setup@v2
      - uses: actions/checkout@v2
      - run: cargo doc --no-deps --message-format=json | cargo-action-fmt
```

![Example annotation](https://user-images.githubusercontent.com/240738/153767390-66f859d4-da3f-4e1e-846b-02605e8be628.png)
