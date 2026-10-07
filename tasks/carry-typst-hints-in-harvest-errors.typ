#import "@local/mindtape:0.2.0": *

#show: task.with(
  title: "carry Typst's hints in harvest errors",
  status: proposed(2026, 10, 7),
)

== Summary

typst-world 0.5.0 keeps each diagnostic's severity, Typst's hints and labelled
trace points, and drops the `eval error: ` prefix from `WorldError::Eval`.
`HarvestError::Eval` still adds that prefix, and `Hint`, `Severity` and
`TracePoint` are not re-exported, so a caller cannot name the types it reads.

== Scope

Require typst-world 0.5.0, re-export `Hint`, `Severity` and `TracePoint`, and
display `HarvestError::Eval` as Typst's message alone. Ships as 0.5.0
(breaking).
