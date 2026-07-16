#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use tempfile::TempDir;
use typst_world::SourceSnapshot;

use super::super::{HVal, Harvest, HarvestWorld, batch_harvest, harvest};

fn project() -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "[package]").unwrap();
    dir
}

fn write_marker_file(dir: &TempDir, name: &str, marker: &str) -> PathBuf {
    let file = dir.path().join(name);
    std::fs::write(&file, format!("#metadata((marker: \"{marker}\"))\n")).unwrap();
    file
}

fn marker_name(harvested: &Harvest) -> Option<&str> {
    harvested
        .markers
        .first()
        .and_then(|marker| marker.value.get("marker"))
        .and_then(HVal::as_str)
}

#[test]
fn matches_sequential_harvest_in_order() {
    let dir = project();
    let files: Vec<PathBuf> = (0..5)
        .map(|i| write_marker_file(&dir, &format!("f{i}.typ"), &format!("m.{i}")))
        .collect();

    let batch_results = batch_harvest(&files, |path| Ok(HarvestWorld::new(path)?));

    assert_eq!(batch_results.len(), files.len());
    for (path, batch_result) in files.iter().zip(batch_results) {
        let sequential_world = HarvestWorld::new(path).unwrap();
        let sequential = harvest(&sequential_world).unwrap();
        let batched = batch_result.unwrap();
        assert_eq!(batched.markers, sequential.markers);
    }
}

#[test]
fn isolates_a_failing_item_to_its_own_slot() {
    let dir = project();
    let good_a = write_marker_file(&dir, "a.typ", "m.a");
    let bad = dir.path().join("bad.typ");
    std::fs::write(&bad, "#import \"@local/does-not-exist:9.9.9\": *\n").unwrap();
    let good_b = write_marker_file(&dir, "b.typ", "m.b");

    let files = vec![good_a, bad, good_b];
    let results = batch_harvest(&files, |path| Ok(HarvestWorld::new(path)?));

    assert_eq!(results.len(), 3);
    assert!(results.first().unwrap().is_ok());
    assert!(results.get(1).unwrap().is_err());
    assert!(results.get(2).unwrap().is_ok());
}

#[test]
fn shared_snapshot_harvests_every_file_correctly() {
    let dir = project();
    let files: Vec<PathBuf> = (0..3)
        .map(|i| write_marker_file(&dir, &format!("s{i}.typ"), &format!("m.{i}")))
        .collect();
    let root = dir.path().to_path_buf();
    let snapshot = SourceSnapshot::new();

    let results = batch_harvest(&files, |path| {
        Ok(HarvestWorld::with_root(path, &root)?.with_shared_sources(&snapshot))
    });

    assert_eq!(results.len(), files.len());
    for (i, result) in results.into_iter().enumerate() {
        let harvested = result.unwrap();
        assert_eq!(marker_name(&harvested), Some(format!("m.{i}").as_str()));
    }
}
