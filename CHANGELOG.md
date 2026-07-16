# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-07-16

### Added

- `batch_harvest`: harvest many `.typ` files in parallel across cores via
  `rayon`, behind the new default-on `batch` feature. The caller builds each
  `World` (so it keeps roots, `@local` overrides, and can share one
  `SourceSnapshot` across worlds so a common imported prelude is parsed
  once); results preserve input order and isolate a per-item build/eval
  failure to that item's slot rather than aborting the batch.

## [0.1.0]

Initial extraction from the mindtape workspace (`crates/typst-harvest`).

### Added

- `harvest()`: evaluate a `.typ` file (via `typst-world`'s eval-only `World`)
  and collect its `metadata()` markers, projected to `typst-world`'s
  Typst-free `HVal` tree.
- `Harvest`/`Marker`/`Location`: harvested markers with their value, optional
  label, and source location (path plus line, package-qualified for
  `@local/<name>:<version>` imports).
- `Harvest::with_marker`/`Harvest::with_label`: filter markers by the
  dict-key or Typst-label namespacing convention.
- `HarvestError`, plus `World`/`HVal`/`convert`/`find_project_root`/
  `format_date` re-exported from `typst-world` for one-crate consumption.
