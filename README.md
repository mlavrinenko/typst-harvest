# typst-harvest

[![CI](https://github.com/mlavrinenko/typst-harvest/actions/workflows/ci.yml/badge.svg)](https://github.com/mlavrinenko/typst-harvest/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/typst-harvest.svg)](https://crates.io/crates/typst-harvest)
[![License: MIT](https://img.shields.io/crates/l/typst-harvest.svg)](LICENSE-MIT)

Labelled metadata-marker harvesting for Typst documents, on top of typst-world — a substrate for file-based tools that use Typst as a data medium

## Install

```bash
cargo add typst-harvest
```

## Usage

```rust,no_run
use typst_harvest::{HarvestWorld, harvest};

let world = HarvestWorld::new(std::path::Path::new("task.typ"))?;
let result = harvest(&world)?;
for marker in result.with_marker("mindtape.task") {
    println!("{:?}", marker.value);
}
# Ok::<(), typst_harvest::HarvestError>(())
```

Evaluates a `.typ` file with `typst-eval` (never layout or render) and
collects the `metadata()` markers it emits, projected to `typst-world`'s
Typst-free `HVal` tree. The `World` and value tree are re-exported from
[`typst-world`](https://crates.io/crates/typst-world); this crate adds the
harvesting step on top.

## Development

Prerequisites: [Nix](https://nixos.org/) with flakes enabled.

```bash
direnv allow         # or: nix develop

just check           # fmt + clippy + tests + file-size + drift check
just build
just test
just cover           # code coverage (70% minimum)
just fmt             # format code
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for coding conventions.

## License

MIT
