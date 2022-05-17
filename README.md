# `cargo-action-fmt`

Takes JSON-formatted `cargo check` (and `cargo clippy`) output and formats it for GitHub actions.

### Examples

This tool can be used with a variety of `cargo` commands:

```shell
:; cargo check -q --message-format=json | clippy-action-fmt
```

```shell
:; cargo clippy -q --message-format=json | clippy-action-fmt
```

```shell
:; cargo doc --message-format=json | clippy-action-fmt
```

It's primarily intended to be used in a GitHub Actions workflow:

```yaml
  docs:
    timeout-minutes: 10
    runs-on: ubuntu-latest
    container:
      image: docker://rust:1.60-bullseye
    steps:
      - uses: olix0r/cargo-action-fmt@v1.0.1
      - uses: actions/checkout@v2
      - run: cargo doc --no-deps --message-format=json | cargo-action-fmt
```

![Example annotation](https://user-images.githubusercontent.com/240738/153767390-66f859d4-da3f-4e1e-846b-02605e8be628.png)
