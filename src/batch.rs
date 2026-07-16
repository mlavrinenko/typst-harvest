//! Parallel batch harvesting across many `.typ` files.
//!
//! [`batch_harvest`] fans a slice of items out across CPU cores via
//! [`rayon`], building one short-lived [`HarvestWorld`] per item and
//! harvesting it on the worker thread that built it — the world never
//! crosses a thread boundary. Sharing a [`typst_world::SourceSnapshot`]
//! across the worlds the caller builds lets a common imported prelude (a
//! `@local` package, a shared include) be parsed once instead of once per
//! item.

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{Harvest, HarvestError, HarvestWorld, harvest};

/// Harvest many items in parallel, one [`HarvestWorld`] per item.
///
/// `build` constructs the [`HarvestWorld`] for one item; `batch_harvest`
/// calls it and then [`harvest`] on the same worker thread, so the world
/// itself is never sent across threads — only `T` (via `&T`, requiring
/// `Sync`) and the harvested [`Harvest`]/[`HarvestError`] results cross back.
/// The caller owns world construction, so it decides the project root,
/// `@local` package overrides, injected globals, and whether the worlds
/// share one [`typst_world::SourceSnapshot`] (itself `Clone` and safe to
/// share — see its docs) so a common prelude is parsed once instead of once
/// per item.
///
/// The returned `Vec` has exactly `items.len()` entries in the same order as
/// `items`: slot `i` holds the result for `items[i]`. This is a batch, not a
/// transaction — a `build` or eval failure for one item lands as an `Err` in
/// exactly that item's slot and does not abort or skip the others.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
///
/// use typst_harvest::{HarvestError, HarvestWorld, SourceSnapshot, batch_harvest};
///
/// # fn run() -> Result<(), HarvestError> {
/// let root = Path::new("/project");
/// let files = vec![root.join("a.typ"), root.join("b.typ")];
///
/// // Shared across every world built below, so a common `@local` prelude
/// // the files import is parsed once, not once per file.
/// let snapshot = SourceSnapshot::new();
///
/// let results = batch_harvest(&files, |path| {
///     Ok(HarvestWorld::with_root(path, root)?.with_shared_sources(&snapshot))
/// });
///
/// for (path, result) in files.iter().zip(results) {
///     match result {
///         Ok(harvest) => println!("{}: {} markers", path.display(), harvest.markers.len()),
///         Err(err) => eprintln!("{}: {err}", path.display()),
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn batch_harvest<T, F>(items: &[T], build: F) -> Vec<Result<Harvest, HarvestError>>
where
    T: Sync,
    F: Fn(&T) -> Result<HarvestWorld, HarvestError> + Sync + Send,
{
    items
        .par_iter()
        .map(|item| build(item).and_then(|world| harvest(&world)))
        .collect()
}

#[cfg(test)]
#[path = "batch_tests.rs"]
mod tests;
