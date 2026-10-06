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

### Eval errors

A file that fails to evaluate returns `HarvestError::Eval` with `typst-world`'s
`EvalError`: every diagnostic with its message, location and trace.
`main_location()` names the first point inside the harvested file, walking the
trace when the error was raised in a file it imports:

```rust,no_run
use typst_harvest::{HarvestError, HarvestWorld, harvest};

# fn run() -> Result<(), HarvestError> {
let world = HarvestWorld::new(std::path::Path::new("task.typ"))?;
if let Err(HarvestError::Eval(err)) = harvest(&world) {
    match err.main_location() {
        Some(at) => eprintln!("{}:{}:{}: {err}", at.path, at.line, at.column),
        None => eprintln!("{err}"),
    }
}
# Ok(())
# }
```

### Batch harvesting

The `batch` feature (default-on) adds `batch_harvest`: evaluate many files in
parallel across cores via `rayon`. The caller builds each `World` (so it
controls roots, `@local` overrides, and can share one `SourceSnapshot` across
worlds so a common imported prelude is parsed once); `batch_harvest` harvests
each on the worker thread that built it and returns results in input order,
isolating a per-item failure to that item's slot.

```rust,no_run
use typst_harvest::{HarvestError, HarvestWorld, batch_harvest};

# fn run() -> Result<(), HarvestError> {
let files = vec![std::path::PathBuf::from("a.typ"), std::path::PathBuf::from("b.typ")];
let results = batch_harvest(&files, |path| Ok(HarvestWorld::new(path)?));
for result in results {
    match result {
        Ok(harvest) => println!("{} markers", harvest.markers.len()),
        Err(err) => eprintln!("skipped: {err}"),
    }
}
# Ok(())
# }
```

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
