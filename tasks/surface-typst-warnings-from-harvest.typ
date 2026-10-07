#import "@local/mindtape:0.2.0": *

#show: task.with(
  title: "surface Typst warnings from harvest",
  status: proposed(2026, 10, 7),
)

== Summary

`harvest()` drops the warnings Typst's eval leaves in its `Sink`, both when
the file evaluates and when it fails. A caller cannot show a deprecation or an
unused-rename warning in the file it harvested.

== Scope

`Harvest` gains `warnings: Vec<Diagnostic>`, resolved through typst-world's
`World::diagnostic`. `HarvestError::Eval` becomes
`Eval { error: EvalError, warnings: Vec<Diagnostic> }`, so a failed file
keeps the warnings raised before the error. Ships as 0.6.0 (breaking).
