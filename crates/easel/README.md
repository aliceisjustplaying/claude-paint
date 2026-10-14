# easel

A live painting session over the `paint` engine: a canvas in a background
process that takes Lua chunks one at a time. The guide is
`notes/easel_guide.md` at the top of the repository.

```sh
cargo build --release -p easel
target/release/easel help
```

`easel save [path]` writes a PNG inside the studio. Relative paths are
relative to the studio, including when the easel was opened from another
directory. Absolute paths must also be inside the studio after resolving
parent symlinks; the destination itself must not be a symlink or a
hard-linked file. The default destination follows the same checks.
