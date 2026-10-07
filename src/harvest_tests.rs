#![allow(clippy::unwrap_used)]

use super::super::{HVal, HarvestError, HarvestWorld, Hint, Severity, harvest};

fn project(file_body: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "[package]").unwrap();
    let file = dir.path().join("task.typ");
    std::fs::write(&file, file_body).unwrap();
    (dir, file)
}

#[test]
fn harvests_a_marker() {
    let (_dir, file) =
        project("#metadata((marker: \"mindtape.task\", fields: (title: \"T\"), extras: (:)))\n");
    let world = HarvestWorld::new(&file).unwrap();
    let result = harvest(&world).unwrap();

    let markers: Vec<_> = result.with_marker("mindtape.task").collect();
    assert_eq!(markers.len(), 1);
    let fields = markers.first().unwrap().value.get("fields").unwrap();
    assert_eq!(fields.get("title").and_then(HVal::as_str), Some("T"));
}

#[test]
fn ignores_files_without_the_marker() {
    let (_dir, file) = project("= Just a heading\n\nSome prose.\n");
    let world = HarvestWorld::new(&file).unwrap();
    let result = harvest(&world).unwrap();
    assert_eq!(result.with_marker("mindtape.task").count(), 0);
}

#[test]
fn eval_error_is_reported() {
    let (_dir, file) = project("#import \"@local/does-not-exist:9.9.9\": *\n");
    let world = HarvestWorld::new(&file).unwrap();
    assert!(harvest(&world).is_err());
}

// --- speconaut-shaped capabilities ---------------------------------------
//
// The three tests below exercise the surface a label-namespaced consumer
// (speconaut) needs from the shared crate, on a flat in-tree document with no
// such consumer present: a marker keyed by label rather than a dict field, its
// source location, host-native globals, and a synthesised package entry source.

#[test]
fn captures_label_and_location() {
    // The canonical Typst queryable form: `#metadata(..) <label>`.
    let (_dir, file) =
        project("#set page(width: auto)\n#metadata((kind: \"feature\")) <speconaut>\n");
    let world = HarvestWorld::new(&file).unwrap();
    let result = harvest(&world).unwrap();

    let marker = result.with_label("speconaut").next().unwrap();
    assert_eq!(
        marker.value.get("kind").and_then(HVal::as_str),
        Some("feature")
    );
    let loc = marker.location.as_ref().unwrap();
    assert_eq!(loc.path, "task.typ");
    assert_eq!(loc.line, 2);
    assert_eq!(loc.column, 2);
}

#[test]
fn globals_inject_host_native_bindings() {
    use typst::foundations::{Scope, Value};

    let (_dir, file) = project("#metadata((marker: INJECTED))\n");
    let mut scope = Scope::new();
    scope.define("INJECTED", Value::Str("demo.global".into()));
    let world = HarvestWorld::new(&file).unwrap().with_globals(&scope);

    let result = harvest(&world).unwrap();
    assert_eq!(result.with_marker("demo.global").count(), 1);
}

#[test]
fn source_override_backs_a_package_entry() {
    use typst::syntax::package::{PackageSpec, PackageVersion};
    use typst::syntax::{FileId, RootedPath, VirtualPath, VirtualRoot};

    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join(".mindtape")).unwrap();

    // A package dir with a manifest naming its entry point, but no entry file
    // on disk — the entry source is injected.
    let pkg = dir.path().join("pkg");
    std::fs::create_dir(&pkg).unwrap();
    std::fs::write(
        pkg.join("typst.toml"),
        "[package]\nname = \"demo\"\nversion = \"1.0.0\"\nentrypoint = \"lib.typ\"\n",
    )
    .unwrap();

    let file = dir.path().join("task.typ");
    std::fs::write(&file, "#import \"@local/demo:1.0.0\": *\n#hit\n").unwrap();

    let spec = PackageSpec {
        namespace: "local".into(),
        name: "demo".into(),
        version: PackageVersion {
            major: 1,
            minor: 0,
            patch: 0,
        },
    };
    let vpath = VirtualPath::new("lib.typ").unwrap();
    let entry = FileId::new(RootedPath::new(VirtualRoot::Package(spec), vpath));

    let world = HarvestWorld::new(&file)
        .unwrap()
        .with_local_package("demo", pkg)
        .with_source_override(entry, "#let hit = metadata((marker: \"demo.hit\"))\n");

    let result = harvest(&world).unwrap();
    assert_eq!(result.with_marker("demo.hit").count(), 1);
}

#[test]
fn eval_error_names_the_line_in_the_main_file() {
    let (_dir, file) = project("= Title\n\n#let tags = (\"a\" \"b\")\n");
    let world = HarvestWorld::new(&file).unwrap();
    let Err(HarvestError::Eval(err)) = harvest(&world) else {
        panic!("expected an eval error");
    };
    assert_eq!(err.to_string(), "expected comma");
    let at = err.main_location().unwrap();
    assert_eq!((at.path.as_str(), at.line, at.column), ("task.typ", 3, 17));
}

#[test]
fn eval_error_inside_an_import_names_the_main_files_line() {
    let (dir, file) = project("#import \"doc.typ\": doc\n#show: doc.with(\n  titel: \"x\",\n)\n");
    std::fs::write(
        dir.path().join("doc.typ"),
        "#let doc(title: none, body) = { assert(title != none); body }\n",
    )
    .unwrap();
    let world = HarvestWorld::new(&file).unwrap();
    let Err(HarvestError::Eval(err)) = harvest(&world) else {
        panic!("expected an eval error");
    };
    assert_eq!(err.to_string(), "unexpected argument: titel");
    assert_eq!(err.main_location().map(|at| at.line), Some(3));
}

#[test]
fn eval_error_displays_typsts_message_and_keeps_its_hints() {
    let (_dir, file) = project("#let a = 1\n#a-b\n");
    let world = HarvestWorld::new(&file).unwrap();
    let Err(err) = harvest(&world) else {
        panic!("expected an eval error");
    };
    assert_eq!(err.to_string(), "unknown variable: a-b");
    let HarvestError::Eval(err) = err else {
        panic!("expected an eval error");
    };
    let first = err.diagnostics.first().unwrap();
    assert_eq!(first.severity, Severity::Error);
    let hints: Vec<&Hint> = first.hints.iter().collect();
    assert!(
        hints
            .iter()
            .any(|hint| hint.message.contains("subtraction")),
        "{hints:?}"
    );
}
