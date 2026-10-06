#import "@local/mindtape:0.2.0": *

#show: task.with(
  title: "carry eval diagnostics in harvest errors",
  status: done(
    2026,
    10,
    7,
  )[HarvestError::Eval carries typst-world 0.4's EvalError; Location comes from typst-world and gains a column. 0.4.0.],
)

== Summary

`HarvestError::Eval(String)` held the diagnostics already joined into one
message, so a caller could not say which line of the harvested file broke.
MindTape printed `tasks/missing-comma.typ: eval error: expected comma` for a
30-line task.

== Scope

`HarvestError::Eval` carries `typst-world`'s `EvalError`, whose
`main_location()` names the first point inside the harvested file. A marker's
`Location` is now `typst-world`'s, resolved by `World::locate`, and gains a
1-based column.

Found while closing MindTape's
`truth/tasks/name-the-line-of-a-task-files-eval-error.typ`.
