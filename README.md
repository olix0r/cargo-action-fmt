# `clippy-action-fmt`

Takes JSON-formatted `cargo check` (and `cargo clippy`) output and formats it for GitHub actions.

### Examples

```shell
:; cargo check -q --message-format=json | clippy-action-fmt
```

```shell
:; cargo clippy -q --message-format=json | clippy-action-fmt
```
