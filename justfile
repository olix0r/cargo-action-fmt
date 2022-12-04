# Defaults for development.
profile := 'debug'
target := 'x86_64-unknown-linux-gnu'

fetch *args:
    @just-cargo fetch {{ args }}

build *args:
    @just-cargo profile={{ profile }} target={{ target }} build --frozen {{ args }}

package dir: build
    @mkdir -p '{{ dir }}/{{ profile }}'
    tar czf '{{ dir }}/{{ profile }}/cargo-action-fmt-{{ target }}.tar.gz' \
        -C 'target/{{ target }}/{{ profile }}' \
        cargo-action-fmt

crate-version:
    @just-cargo crate-version 'cargo-action-fmt'
